use std::time::Duration;

use anyhow::Result;
use rand::{distributions::Uniform, prelude::Distribution};
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::WindowCanvas, rect::Point};


struct Visualizer {
    canvas: WindowCanvas,
    vector: Vec<u32>,
}

impl Visualizer {

    fn new(canvas: WindowCanvas) -> Self {
    let between = Uniform::from(1..100);
    let mut rng = rand::thread_rng();
    let vector: Vec<u32> = (1..100).map(|_| between.sample(&mut rng) ).collect();
        Self { canvas, vector }
    }

    fn reset(&mut self) {
        let between = Uniform::from(1..100);
        let mut rng = rand::thread_rng();
        let vector: Vec<u32> = (1..100).map(|_| between.sample(&mut rng) ).collect();
        self.vector = vector; 

        std::thread::sleep(Duration::from_secs(2));

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn draw_state(&mut self,red: usize,blue: usize) {

        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.clear();

        for (i,el) in self.vector.iter().enumerate() {
            let color = match i {
                r if r == red => Color::RGB(255, 0, 0),
                b if b == blue => Color::RGB(0, 0, 255),
                _ => Color::RGB(255,255,255),
            };

            self.canvas.set_draw_color(color);

            let p1 = Point::new(i as i32+1,100);
            let p2 = Point::new(i as i32+1,*el as i32);
            self.canvas.draw_line(p1,p2).unwrap();

        }
        self.canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }

    fn selection_sort(&mut self) {
        let mut min_index = 0;
        for i in 0..self.vector.len() {
            for j in i+1..self.vector.len() {
                if self.vector[j] < self.vector[min_index] {
                    min_index = j;
                }
            }
            self.vector.swap(i, min_index);
            self.draw_state(i, min_index);
        }
    }

    fn bubble_sort(&mut self) {
        for i in 0..self.vector.len() {
            for j in i+1..self.vector.len() {
                if self.vector[i] > self.vector[j] {
                    self.vector.swap(i, j);
                }
                self.draw_state(i, j);
            }
        }
    }

    fn quick_sort(&mut self) {
        self._quicksort(0, self.vector.len());
    }

    fn _quicksort(&mut self,start: usize,end: usize) {
        if self.vector[start..end].len() > 1 {
            let parition_index = self._partition(start,end) + start;

            self._quicksort(start, parition_index);
            self._quicksort(parition_index+1,end);
        }

    }

    fn _partition(&mut self,start: usize, end: usize) -> usize {
        let len = self.vector[start..end].len();
        let pivot = self.vector[start..end][len - 1];
        let mut i = 0;
        let mut j = 0;

        while j < len - 1 {
            if self.vector[start..end][j] <= pivot {
                self.vector[start..end].swap(i, j);
                self.draw_state(i, j);
                i += 1;
            }
            j += 1;
        }

        self.vector[start..end].swap(i, len - 1);
        self.draw_state(len - 1, i);

        i
    }
    
}


fn main() -> Result<(),String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Visualizer", 700, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_scale(7.0,6.0)?;

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();


    let mut visualizer = Visualizer::new(canvas); 

    let mut event_pump = sdl_context.event_pump()?;

    video_subsystem.disable_screen_saver();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                        break 'running;
                    },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    visualizer.bubble_sort();
                    visualizer.reset();
                
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    visualizer.selection_sort();
                    visualizer.reset();
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    visualizer.quick_sort();
                    visualizer.reset();
                },
                _ => {}
            }    
        }

        println!("> BubbleSort : B\n> Selection Sort: S\n> QuickSort: Q\n> Quit: ESCAPE\n\n");

        visualizer.canvas.present();
        std::thread::sleep(Duration::from_secs(1/60));
    }


    
    Ok(())
}
