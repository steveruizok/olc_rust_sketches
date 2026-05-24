extern crate olc_pixel_game_engine;
use std::borrow::Borrow;

use crate::olc_pixel_game_engine as olc;
use crate::olc_pixel_game_engine::Vi2d;

const SCREEN_WIDTH: i32 = 680;
const SCREEN_HEIGHT: i32 = 500;

struct CldrawLine {
  points: Vec<Vi2d>,
}

impl CldrawLine {
  fn new() -> Self {
    Self {
      points: Vec::new(),
    }
  }
}

// Data belonging to the application.
struct CldrawApplication {
  time: f32,
  game_over: i32,
  is_drawing: bool,
  current_line: CldrawLine,
  lines: Vec<CldrawLine>,
}

// Unique methods for the application.
impl CldrawApplication {
  fn new() -> Self {
    Self {
      time: 0.0,
      game_over: 0,
      is_drawing: false,
      current_line: CldrawLine::new(),
      lines: Vec::new(),
    }
  }

  // fn game_over(&mut self) {
  //   self.game_over = 1;
  // }

  // fn game_start(&mut self) {
  //   self.game_over = 0;
  // }
}

// The main application.
impl olc::Application for CldrawApplication {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    // Will run once when the application starts.
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::WHITE);

    if self.game_over == 1 {
      // end of game
    } else if self.game_over == 0 {
      // game is running
      self.time += elapsed_time;

      let (mx, my) = (olc::get_mouse_x(), olc::get_mouse_y());

      if olc::get_mouse(0).pressed {
        // left mouse pressed, start drawing
        self.is_drawing = true;
      } else if olc::get_mouse(0).released {
        // left mouse released, stop drawing
        self.is_drawing = false;

        let mut copy = CldrawLine::new();
        for i in 0..self.current_line.points.len() {
          copy.points.push(self.current_line.points[i].clone());
        }
        self.lines.push(copy);
        self.current_line = CldrawLine::new();
      }

      if self.is_drawing {
        if mx <= 0 || mx >= olc::get_draw_target_width() - 1 {
          // noop
        } else if my <= 0 || my >= olc::get_draw_target_height() - 1 {
          // noop
        } else {
          self.current_line.points.push(Vi2d::new(mx, my));
        }
      }

      /* --------------------- Render --------------------- */

      if self.current_line.points.len() > 0 {
        for i in 0..self.current_line.points.len() - 1 {
          let a = self.current_line.points[i];
          let b = self.current_line.points[i + 1];
          olc::draw_line(a.x, a.y, b.x, b.y, olc::BLACK);
          //   // olc::fill_circle(self.current_line.xs[i], self.current_line.ys[i], 3, olc::BLACK);
        }
      }

      olc::draw_string(40, 40, &olc::get_fps().to_string(), olc::BLACK)?;
    }

    // Will run on each frame.
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Will run once when the application ends.
    Ok(())
  }
}

fn main() {
  let mut app = CldrawApplication::new();
  olc
    ::start(
      "rldraw", // name
      &mut app,
      SCREEN_WIDTH, // screen width
      SCREEN_HEIGHT, // screen height
      1, // pixel width
      1 // pixel height
    )
    .unwrap();
}
