extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./primitives/arrow.rs"]
mod arrow;
#[path = "./primitives/point.rs"]
mod point;
#[path = "./primitives/rect.rs"]
mod rect;
#[path = "./primitives/segment.rs"]
mod segment;
#[path = "./primitives/shared.rs"]
mod shared;

use point::Point;
// use rect::Rect;
// use segment::Segment;

const ERR: f32 = 12.0;

struct Freehand {
  lines: Vec<Vec<Point>>,
  prev: Point,
  width: i32,
  height: i32,
  px: i32,
  py: i32,
  p0: Point,
  p1: Point,
  p2: Point,
  p3: Point,
  err: f32,
}

impl Freehand {
  fn initialize(&mut self) {
    self.width = olc::screen_width();
    self.height = olc::screen_height();
    olc::clear(olc::Pixel::rgb(240, 191, 58));
  }

  fn set_prev_point(&mut self, p: Point) {
    self.prev = p;
  }

  fn create_current_line(&mut self) {
    let mut line = Vec::new();
    self.lines.push(line);
  }

  fn get_current_line(&mut self) -> &mut Vec<Point> {
    let idx = self.lines.len() - 1;
    let line = &mut self.lines[idx];
    return line;
  }

  fn add_point_to_current_line(&mut self, point: Point) {
    let line = self.get_current_line();
    line.push(point);
  }

  fn remove_point_from_current_line(&mut self) {
    let line = self.get_current_line();
    line.pop();
  }

  fn render(&mut self) {
    for line in &self.lines {
      for i in 1..line.len() {
        let a = line[i - 1];
        let b = line[i];
        olc::draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, olc::BLACK);
      }
    }
    for line in &self.lines {
      for a in line {
        olc::fill_circle(a.x as i32, a.y as i32, 2, olc::BLACK);
      }
    }
  }
}

impl olc::Application for Freehand {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.initialize();
    self.render();
    self.px = olc::get_mouse_x();
    self.py = olc::get_mouse_y();
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(240, 191, 58));
    let mouse = olc::get_mouse(0);
    // Position
    let x = olc::get_mouse_x();
    let y = olc::get_mouse_y();
    // Delta
    // let dx = x - self.px;
    // let dy = y - self.py;
    self.px = x;
    self.py = y;
    let currentPoint = Point::new(x as f32, y as f32);

    if currentPoint.is_equal(&self.p3) {
      self.render();
      return Ok(());
    }

    // When pressing...
    if mouse.pressed {
      self.set_prev_point(currentPoint);
      self.create_current_line();
      self.add_point_to_current_line(currentPoint);
      self.add_point_to_current_line(currentPoint);
      self.render();
      self.p0 = currentPoint;
      self.p1 = currentPoint;
      self.p2 = currentPoint;
      self.p3 = currentPoint;
    }

    // While dragging...
    if mouse.held {
      self.p0 = self.p1;
      self.p1 = self.p2;
      self.p2 = self.p3;
      self.p3 = currentPoint;

      let u1 = self.p0.get_tan(self.p1.x, self.p1.y);
      let u2 = self.p2.get_tan(self.p3.x, self.p3.y);
      let dpr = u1.get_dot(&u2);

      if dpr.is_nan() {
        return Ok(());
      }

      // modulate between 1 and .8, 1 and ERR

      let result = 1.0 + ((dpr - 1.0) / (0.9 - 1.0)) * (ERR - 1.0);
      self.err -= result;

      // Push a new permanent point
      if self.err < 0.0 {
        let streamlined = self.prev.med(&currentPoint);
        self.set_prev_point(streamlined);
        // Pop the temporary point
        self.remove_point_from_current_line();
        // Add the permeable point
        self.add_point_to_current_line(streamlined);
        // Add the temporary point
        self.add_point_to_current_line(currentPoint);

        while self.err < 0.0 {
          self.err += ERR;
        }
      } else {
        // Pop the temporay point, push a new temporary point
        self.remove_point_from_current_line();
        self.add_point_to_current_line(currentPoint);
      }
      // Render
      self.render();
    } else {
      self.render();
    }

    // When releasing...
    if mouse.released {
      // Push the current point into the current vector
      self.add_point_to_current_line(currentPoint);
      self.render();
    }

    // Clear on Escape
    if olc::get_key(olc::Key::ESCAPE).pressed {
      self.lines.clear();
    }

    Ok(())
  }
  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Freehand {
    lines: Vec::new(),
    prev: Point::new(0.0, 0.0),
    width: 0,
    height: 0,
    px: 0,
    py: 0,
    p0: Point::new(0.0, 0.0),
    p1: Point::new(0.0, 0.0),
    p2: Point::new(0.0, 0.0),
    p3: Point::new(0.0, 0.0),
    err: ERR,
  };
  olc::start("Freehand", &mut app, 655, 480, 2, 2).unwrap();
}
