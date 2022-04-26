extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./shared.rs"]
mod shared;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Point {
    Point { x, y }
  }

  pub fn add(&mut self, x: f32, y: f32) -> &mut Point {
    self.x += x;
    self.y += y;
    return self;
  }

  pub fn add_point(&mut self, other: &Point) -> &mut Point {
    self.x += other.x;
    self.y += other.y;
    return self;
  }

  pub fn sub(&mut self, x: f32, y: f32) -> &mut Point {
    self.x -= x;
    self.y -= y;
    return self;
  }

  pub fn div(&mut self, x: f32, y: f32) -> &mut Point {
    self.x /= x;
    self.y /= y;
    return self;
  }

  pub fn mul(&mut self, x: f32, y: f32) -> &mut Point {
    self.x *= x;
    self.y *= y;
    return self;
  }
  pub fn uni(&mut self) -> &mut Point {
    let length = self.length();
    self.x /= length;
    self.y /= length;
    return self;
  }

  pub fn get_dot(&self, other: &Point) -> f32 {
    self.x * other.x + self.y * other.y
  }

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn get_tan(&self, x: f32, y: f32) -> Point {
    let new_pt = &mut self.clone();
    new_pt.sub(x, y).uni();
    return Point {
      x: new_pt.x,
      y: new_pt.y,
    };
  }
  pub fn rot_with(&mut self, x: f32, y: f32, r: f32) -> &mut Point {
    if r == 0.0 {
      return self;
    };

    let ox = self.x - x;
    let oy = self.y - y;
    let s = f32::sin(r);
    let c = f32::cos(r);
    self.x = x + (ox * c - oy * s);
    self.y = y + (ox * s + oy * c);
    return self;
  }

  pub fn clone(&self) -> Point {
    let pt = Point {
      x: self.x,
      y: self.y,
    };
    return pt;
  }

  pub fn sub_point(&mut self, other: &Point) -> &mut Point {
    self.x -= other.x;
    self.y -= other.y;
    return self;
  }

  pub fn draw(&self, color: Option<olc::Pixel>) {
    olc::fill_circle(
      self.x as i32,
      self.y as i32,
      2,
      match color {
        Some(t) => t,
        None => olc::BLACK,
      },
    );
  }
}
