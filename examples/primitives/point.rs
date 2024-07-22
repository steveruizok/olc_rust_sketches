extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

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

  pub fn is_equal(&self, other: &Point) -> bool {
    return self.x == other.x && self.y == other.y;
  }

  pub fn get_dot(&self, other: &Point) -> f32 {
    self.x * other.x + self.y * other.y
  }

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn dist_point(&self, other: &Point) -> f32 {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    (dx * dx + dy * dy).sqrt()
  }

  pub fn get_tan(&self, x: f32, y: f32) -> Point {
    let new_pt = &mut self.clone();
    new_pt.sub(x, y).uni();
    return *new_pt;
  }

  pub fn get_tan_point(&self, other: &Point) -> Point {
    let new_pt = &mut self.clone();
    new_pt.sub_point(other).uni();
    return *new_pt;
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

  pub fn med(&mut self, other: &Point) -> Point {
    let pt = Point {
      x: (self.x + other.x) / 2.0,
      y: (self.y + other.y) / 2.0,
    };
    return pt;
  }

  pub fn lrp(&mut self, other: &Point, t: f32) -> &mut Point {
    self.x = ((other.x - self.x) * t) + self.x;
    self.y = ((other.y - self.y) * t) + self.y;
    return self;
  }

  pub fn sub_point(&mut self, other: &Point) -> &mut Point {
    self.x -= other.x;
    self.y -= other.y;
    return self;
  }

  pub fn angle(&self, x: f32, y: f32) -> f32 {
    let dx = x - self.x;
    let dy = y - self.y;
    let angle = f32::atan2(dy, dx);
    return angle;
  }

  pub fn angle_point(&self, other: &Point) -> f32 {
    let dx = other.x - self.x;
    let dy = other.y - self.y;
    let angle = f32::atan2(dy, dx);
    return angle;
  }

  pub fn clockwise_point(&self, center: &Point, other: &Point) -> bool {
    return (center.x - self.x) * (other.y - self.y) - (other.x - self.x) * (center.y - self.y)
      < 0.0;
  }

  pub fn cross_point(&self, other: &Point) -> f32 {
    let p0 = &mut self.clone();
    p0.uni();
    let p1 = &mut other.clone();
    p1.uni();
    return p0.x * p1.y - p1.x * p0.y;
  }

  pub fn draw(&self, color: olc::Pixel) {
    olc::fill_circle(self.x as i32, self.y as i32, 2, color);
  }
}
