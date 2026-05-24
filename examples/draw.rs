extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

#[path = "./primitives/draw.rs"]
mod draw;
#[path = "./primitives/point.rs"]
mod point;

use draw::Draw;
use point::Point;

// Data belonging to the application.
struct Drawing {
  lines: Vec<Draw>,
}

// Unique methods for the application.
impl Drawing {
  fn update(&mut self) {}
}

// The main application.
impl olc::Application for Drawing {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    olc::clear(olc::WHITE);

    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::WHITE);

    let mouse = olc::get_mouse(0);
    let x = olc::get_mouse_x();
    let y = olc::get_mouse_y();
    if mouse.pressed {
      self.lines.push(Draw::new(x as f32, y as f32));
    }

    if mouse.held {
      let idx = self.lines.len() - 1;
      let mut line = self.lines.get_mut(idx).unwrap();
      line.add_point(x as f32, y as f32);
    }

    for line in &self.lines {
      line.draw(olc::BLACK);
    }

    if olc::get_key(olc::Key::ESCAPE).pressed {
      self.lines.clear();
    }
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Will run once when the application ends.
    Ok(())
  }
}

fn main() {
  let mut app = Drawing { lines: vec![] };
  olc::start(
    "Draw", // name
    &mut app, 320, // screen width
    240, // screen height
    2,   // pixel width
    2,   // pixel height
  )
  .unwrap();
}
