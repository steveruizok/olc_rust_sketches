extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./point.rs"]
mod point;
#[path = "./shared.rs"]
mod shared;

use point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
  pub x1: f32,
  pub y1: f32,
  pub x2: f32,
  pub y2: f32,
}

impl Segment {
  pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Segment {
    Segment { x1, y1, x2, y2 }
  }

  pub fn get_start(&self) -> Point {
    Point::new(self.x1, self.y1)
  }

  pub fn get_end(&self) -> Point {
    Point::new(self.x2, self.y2)
  }

  pub fn get_center(&self) -> Point {
    Point::new((self.x1 + self.x2) / 2.0, (self.y1 + self.y2) / 2.0)
  }

  pub fn get_length(&self) -> f32 {
    ((self.x1 - self.x2) * (self.x1 - self.x2) + (self.y1 - self.y2) * (self.y1 - self.y2)).sqrt()
  }

  pub fn get_direction(&self) -> Point {
    let length = self.get_length();
    if length == 0.0 {
      return Point::new(0.0, 0.0);
    }
    let ux = (self.x2 - self.x1) / length;
    let uy = (self.y2 - self.y1) / length;
    return Point::new(ux, uy);
  }

  pub fn get_line_intersection(&self, seg: &Segment) -> Option<Point> {
    let abx = self.x1 - seg.x1;
    let aby = self.y1 - seg.y1;
    let bvx = seg.x2 - seg.x1;
    let bvy = seg.y2 - seg.y1;
    let avx = self.x2 - self.x1;
    let avy = self.y2 - self.y1;
    let ua_t = bvx * aby - bvy * abx;
    let ub_t = avx * aby - avy * abx;
    let u_b = bvy * avx - bvx * avy;
    if ua_t == 0.0 || ub_t == 0.0 {
      return None;
    }

    if u_b == 0.0 {
      return None;
    }

    let ua = ua_t / u_b;
    let ub = ub_t / u_b;
    if 0.0 <= ua && ua <= 1.0 && 0.0 <= ub && ub <= 1.0 {
      return Some(Point::new(self.x1 + ua * avx, self.y1 + ua * avy));
    } else {
      return None;
    }
  }

  pub fn draw(&self, color: Option<olc::Pixel>) {
    olc::draw_line(
      self.x1 as i32,
      self.y1 as i32,
      self.x2 as i32,
      self.y2 as i32,
      match color {
        Some(t) => t,
        None => olc::BLACK,
      },
    )
  }
}
