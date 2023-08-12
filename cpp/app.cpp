#include<SDL2/SDL.h>
#include<random>
#include<algorithm>
#include<iostream>
#include<ranges>
#include <vector>

void print_vector(std::vector<int>);

void draw_state(SDL_Renderer *renderer,std::vector<int> v,int red,int blue) {
    int index = 0;
    for (int i: v) {
        if (index == red)
            SDL_SetRenderDrawColor(renderer, 255,0,0,255);
        else if (index == blue)
            SDL_SetRenderDrawColor(renderer, 0,0,255,255);
        else 
            SDL_SetRenderDrawColor(renderer, 255,255,255,255);

        SDL_RenderDrawLine(renderer,index,99,index,i);
        index += 1;
    }
    
}

std::vector<int> sorting(SDL_Renderer *renderer,std::vector<int> v) {
    for(int i=0; i < v.size(); i++) {
        for(int j= i + 1; j < v.size(); j++) {
            if (v[i] > v[j] ) {
                std::swap(v[i],v[j]);
            };

            SDL_SetRenderDrawColor(renderer, 0,0,0,255);
            SDL_RenderClear(renderer);

            draw_state(renderer, v,i,j);

            SDL_RenderPresent(renderer);
            SDL_Delay(5);
        }
    }

    print_vector(v);

    return v;

}

int main(int argl,char **argv) {
    std::random_device rd; // Seed Source For The random number engine
    std::uniform_int_distribution<int> d(1,99); // Random number from range

    std::vector<int> v;

    for(int i=0; i <100; i++) {
        v.push_back(d(rd));
    }

    SDL_Init( SDL_INIT_VIDEO | SDL_INIT_AUDIO );

    SDL_Window* window = nullptr;
    SDL_Renderer* renderer = nullptr;
    SDL_CreateWindowAndRenderer(
            100*10, 
            70*10, 
            0, 
            &window, 
            &renderer);

    SDL_RenderSetScale(renderer, 10, 7);

    std::vector<int> sorted = sorting(renderer,v);
    

    if (std::ranges::is_sorted(sorted)) {
        std::cout << "Sorted";
    }

   
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    return 0;
}

void print_vector(std::vector<int> v) {
    for (int a : v) {
        std::cout << a << " "; 
    }

    std::cout << std::endl;

}
