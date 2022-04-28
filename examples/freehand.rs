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
// use segment::Segment;

const TAU: f32 = (std::f32::consts::PI) / 2.0;

// For error
const ERR: f32 = 12.0;
const SENSITIVITY: f32 = 0.9;

// Pressure
const MIN_DISTANCE: f32 = 12.0;
const RATE_OF_CHANGE: f32 = 0.275;

// Radius
const MIN_RADIUS: f32 = 4.0;
const MAX_RADIUS: f32 = 32.0;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
  point: Point,
  pressure: f32,
}

struct Freehand {
  lines: Vec<Vec<Node>>,
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
  pressure: f32,
  debug: bool,
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

  fn get_current_line(&mut self) -> &mut Vec<Node> {
    let idx = self.lines.len() - 1;
    let line = &mut self.lines[idx];
    return line;
  }

  fn add_point_to_current_line(&mut self, point: Node) {
    let line = self.get_current_line();
    line.push(point);
  }

  fn remove_point_from_current_line(&mut self) {
    let line = self.get_current_line();
    line.pop();
  }

  fn render_outer_tangents(&self, x1: f32, y1: f32, r1: f32, x2: f32, y2: f32, r2: f32) {
    // Angle from 0deg horizontal line to line between circle centers
    let gamma = ((y1 - y2) / (x2 - x1)).atan();
    // Angle between line from center of c1 to center of c2 and r2 - r1 at x2, y2
    let beta = ((r2 - r1) / (((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1))).sqrt()).asin();
    // Angle between 90deg vertical line and a right angle to the tangent line
    let alpha = gamma - beta;
    // Angle between 90deg vertical line and a right angle to the other tangent line
    let theta = gamma + beta;

    // First circle bottom tangent point
    let t0 = Point::new(x1 + r1 * (TAU - alpha).cos(), y1 + r1 * (TAU - alpha).sin());

    // // Second circle bottom tangent point
    let t1 = Point::new(x2 + r2 * (TAU - alpha).cos(), y2 + r2 * (TAU - alpha).sin());

    // First circle top tangent point
    let t2 = Point::new(
      x1 + r1 * (-TAU - theta).cos(),
      y1 + r1 * (-TAU - theta).sin(),
    );

    // Second circle top tangent point
    let t3 = Point::new(
      x2 + r2 * (-TAU - theta).cos(),
      y2 + r2 * (-TAU - theta).sin(),
    );

    // let seg0 = Segment::new(t0.x, t0.y, t1.x, t1.y);
    // seg0.draw(Some(olc::BLUE));

    // let seg1 = Segment::new(t2.x, t2.y, t3.x, t3.y);
    // seg1.draw(Some(olc::DARK_YELLOW));

    // t0.draw(Some(olc::BLUE));
    // t1.draw(Some(olc::BLUE));

    // t2.draw(Some(olc::DARK_YELLOW));
    // t3.draw(Some(olc::DARK_YELLOW));

    olc::fill_triangle(
      t0.x as i32,
      t0.y as i32,
      t1.x as i32,
      t1.y as i32,
      t2.x as i32,
      t2.y as i32,
      olc::RED,
    );

    olc::fill_triangle(
      t2.x as i32,
      t2.y as i32,
      t1.x as i32,
      t1.y as i32,
      t3.x as i32,
      t3.y as i32,
      olc::RED,
    );

    if self.debug {
      olc::draw_triangle(
        t0.x as i32,
        t0.y as i32,
        t1.x as i32,
        t1.y as i32,
        t2.x as i32,
        t2.y as i32,
        olc::BLACK,
      );

      olc::draw_triangle(
        t2.x as i32,
        t2.y as i32,
        t1.x as i32,
        t1.y as i32,
        t3.x as i32,
        t3.y as i32,
        olc::BLACK,
      );
    }
  }

  fn render(&mut self) {
    for line in &self.lines {
      for i in 1..line.len() {
        let a = line[i - 1];
        let b = line[i];
        olc::draw_line(
          a.point.x as i32,
          a.point.y as i32,
          b.point.x as i32,
          b.point.y as i32,
          olc::BLACK,
        );
      }
    }
    for line in &self.lines {
      let first = line[0];
      olc::fill_circle(
        first.point.x as i32,
        first.point.y as i32,
        (MIN_RADIUS + (first.pressure * (MAX_RADIUS - MIN_RADIUS))) as i32,
        olc::RED,
      );

      for i in 1..line.len() {
        let a = line[i - 1];
        let b = line[i];

        olc::fill_circle(
          b.point.x as i32,
          b.point.y as i32,
          (MIN_RADIUS + (b.pressure * (MAX_RADIUS - MIN_RADIUS))) as i32,
          olc::RED,
        );

        let a_radius = MIN_RADIUS + (a.pressure * (MAX_RADIUS - MIN_RADIUS));
        let b_radius = MIN_RADIUS + (b.pressure * (MAX_RADIUS - MIN_RADIUS));

        self.render_outer_tangents(
          a.point.x, a.point.y, a_radius, b.point.x, b.point.y, b_radius,
        );
      }
    }

    // Debugging
    // self.p0.draw(Some(olc::MAGENTA));
    // self.p1.draw(Some(olc::RED));
    // self.p2.draw(Some(olc::DARK_RED));
    // self.p3.draw(Some(olc::YELLOW));
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

    /* --------------------- INPUTS --------------------- */

    // Clear on Escape
    if olc::get_key(olc::Key::ESCAPE).pressed {
      self.lines.clear();
    }

    if olc::get_key(olc::Key::SPACE).pressed {
      // Enable debug mode on spacebar down
      self.debug = true;
    } else if olc::get_key(olc::Key::SPACE).released {
      // Disable debug mode on spacebar down
      self.debug = false;
    }

    let mouse = olc::get_mouse(0);
    // Position
    let x = olc::get_mouse_x();
    let y = olc::get_mouse_y();
    // let dx = x - self.px;
    // let dy = y - self.py;
    self.px = x;
    self.py = y;
    let current_point = Point::new(x as f32, y as f32);

    // If the current point hasn't changed, then just render and bail
    if current_point.is_equal(&self.p3) {
      self.render();
      return Ok(());
    }

    // When pressing the mouse...
    if mouse.pressed {
      self.set_prev_point(current_point);
      self.create_current_line();
      self.add_point_to_current_line(Node {
        point: current_point,
        pressure: self.pressure,
      });
      self.add_point_to_current_line(Node {
        point: current_point,
        pressure: self.pressure,
      });
      self.render();
      self.p0 = current_point;
      self.p1 = current_point;
      self.p2 = current_point;
      self.p3 = current_point;
    }

    // While dragging...
    if mouse.held {
      self.p0 = self.p1;
      self.p1 = self.p2;
      self.p2 = self.p3;
      self.p3 = current_point;

      // Getting the dot product between 0->1 and 2->3
      let u1 = self.p0.get_tan(self.p1.x, self.p1.y);
      let u2 = self.p2.get_tan(self.p3.x, self.p3.y);
      let dpr = u1.get_dot(&u2);

      if dpr.is_nan() {
        return Ok(());
      }

      // Find the pressure
      let distance = self.p2.dist_point(&self.p3);
      let sp = (distance / MIN_DISTANCE).min(1.0);
      let rp = (1.0 - sp).min(1.0);
      self.pressure = self.pressure + (rp - self.pressure) * (sp * RATE_OF_CHANGE);

      let current_pressure_point = Node {
        point: current_point,
        pressure: self.pressure,
      };
      // Create a result based on the dot product
      let result = 1.0 + ((dpr - 1.0) / (SENSITIVITY - 1.0)) * (ERR - 1.0);

      // Reduce the err by the result
      self.err -= result;

      // Push a new permanent point
      if self.err < 0.0 {
        // TODO: Sharp corners!
        // Find a point between the previous point and current mouse position
        let streamlined = self.prev.med(&current_point);
        // Make the permanent (streamlined) point the new previous point
        self.set_prev_point(streamlined);
        // Pop the temporary point
        self.remove_point_from_current_line();
        // Add the permanent (streamlined) point
        self.add_point_to_current_line(Node {
          point: self.prev,
          pressure: self.pressure,
        });
        // Add the temporary point
        self.add_point_to_current_line(Node {
          point: current_point,
          pressure: self.pressure,
        });

        while self.err < 0.0 {
          self.err += ERR;
        }
      } else {
        // Pop the temporay point, push a new temporary point
        self.remove_point_from_current_line();
        self.add_point_to_current_line(current_pressure_point);
      }
      // Render
      self.render();
    } else {
      self.render();
    }

    // When releasing...
    if mouse.released {
      // Push the current point into the current vector
      self.add_point_to_current_line(Node {
        point: current_point,
        pressure: self.pressure,
      });
      self.render();
    }

    Ok(())
  }
  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Freehand {
    // lines: vec![vec![
    //   Node {
    //     point: Point::new(100.0, 100.0),
    //     pressure: 0.5,
    //   },
    //   Node {
    //     point: Point::new(200.0, 200.0),
    //     pressure: 0.75,
    //   },
    //   Node {
    //     point: Point::new(250.0, 150.0),
    //     pressure: 0.5,
    //   },
    // ]],
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
    pressure: 0.125,
    debug: false,
  };
  olc::start("Freehand", &mut app, 372, 480, 2, 2).unwrap();
}
