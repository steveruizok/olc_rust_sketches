extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

pub trait Drawable {
  fn draw(&self, color: Option<olc::Pixel>);
}
