extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

// Data belonging to the application.
struct ExampleProgram {}

// Unique methods for the application.
impl ExampleProgram {}

// The main application.
impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    // Will run once when the application starts.
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    // Will run on each frame.
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Will run once when the application ends.
    Ok(())
  }
}

fn main() {
  let mut example = ExampleProgram {};
  olc::start(
    "Hello, World!", // name
    &mut example,
    320, // screen width
    240, // screen height
    2,   // pixel width
    2,   // pixel height
  )
  .unwrap();
}
