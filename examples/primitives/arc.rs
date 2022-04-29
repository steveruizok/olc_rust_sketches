extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

const PI: f32 = std::f32::consts::PI;
const PI2: f32 = PI * 2.0;
const TAU: f32 = (std::f32::consts::PI) / 2.0;

#[path = "./point.rs"]
mod point;
use point::Point;

pub struct Arc {
  x: f32,
  y: f32,
  r: f32,
  pub a: f32,
  pub b: f32,
  pub short: bool,
}

impl Arc {
  pub fn new(x: f32, y: f32, r: f32, a: f32, b: f32, short: bool) -> Arc {
    Arc {
      x,
      y,
      r,
      a,
      b,
      short,
    }
  }

  fn lut(&self) -> Vec<Point> {
    let mut lut = vec![];

    let distance = (PI2 + (self.b - self.a)) % PI2;
    let end_point = self.get_end();

    let count = ((distance / PI2) * 32.0).floor() as i32;

    for i in 1..count {
      let angle = self.a + (i as f32 / count as f32) * (distance);
      let x = self.x + self.r * f32::cos(angle);
      let y = self.y + self.r * f32::sin(angle);
      lut.push(Point { x, y });
    }

    lut.push(end_point);

    return lut;
  }

  pub fn get_center(&self) -> Point {
    return Point::new(self.x, self.y);
  }

  pub fn get_start(&self) -> Point {
    return Point::new(
      self.x + self.r * f32::cos(self.a),
      self.y + self.r * f32::sin(self.a),
    );
  }

  pub fn get_end(&self) -> Point {
    return Point::new(
      self.x + self.r * f32::cos(self.b),
      self.y + self.r * f32::sin(self.b),
    );
  }
  pub fn get_distance(&self) -> f32 {
    let a = self.a;
    let mut b = self.b;
    if a > b {
      b += PI2;
    }
    let d = b - a;
    return d;
  }
  pub fn get_long_angle_distance(&self) -> f32 {
    let mut a = self.a;
    let b = self.b;
    if b > a {
      a += PI2;
    }
    return a - b;
  }

  pub fn get_sweep_flag(&self) -> bool {
    let short_angle_distance = self.get_distance();
    return short_angle_distance > 0.0;
  }

  pub fn get_point_at_t(&self, t: f32) -> Point {
    let delta = self.get_distance();
    let angle = self.a + t * (if delta > 0.0 { PI2 + delta } else { delta });
    let start = self.get_start();

    return Point::new(
      start.x + self.r * f32::cos(angle),
      start.y + self.r * f32::sin(angle),
    );
  }

  pub fn draw(&self, color: olc::Pixel) {
    let lut = self.lut();

    for i in 1..lut.len() {
      let p1 = lut[i - 1];
      let p2 = lut[i];
      olc::fill_triangle(
        self.x as i32,
        self.y as i32,
        p1.x as i32,
        p1.y as i32,
        p2.x as i32,
        p2.y as i32,
        color,
      );

      // olc::draw_triangle(
      //   self.x as i32,
      //   self.y as i32,
      //   p1.x as i32,
      //   p1.y as i32,
      //   p2.x as i32,
      //   p2.y as i32,
      //   olc::BLACK,
      // );
    }
  }

  pub fn draw_edge(&self, color: olc::Pixel) {
    let lut = self.lut();

    for i in 1..lut.len() {
      let p1 = lut[i - 1];
      let p2 = lut[i];
      olc::draw_line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32, color);
    }
  }
}
