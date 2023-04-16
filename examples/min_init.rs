use sdl3_sys::{SDL_Init, SDL_Quit, SDL_INIT_EVERYTHING};

fn main() {
  if unsafe { SDL_Init(SDL_INIT_EVERYTHING) } == 0 {
    println!("SDL was initialized!");
  } else {
    println!("SDL initialization failed.");
  }
  unsafe { SDL_Quit() };
}
