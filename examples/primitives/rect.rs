extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./shared.rs"]
mod shared;

#[path = "./point.rs"]
mod point;

use point::Point;

#[path = "./segment.rs"]
mod segment;

use segment::Segment;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

impl Rect {
  pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
  }

  pub fn get_center(&self) -> Point {
    Point {
      x: self.x + (self.w / 2.0),
      y: self.y + (self.h / 2.0),
    }
  }

  pub fn get_corners(&self) -> Vec<Point> {
    vec![
      Point::new(self.x, self.y),
      Point::new(self.x + self.w, self.y),
      Point::new(self.x + self.w, self.y + self.h),
      Point::new(self.x, self.y + self.h),
    ]
  }

  pub fn get_segments(&self) -> Vec<Segment> {
    let corners = self.get_corners();
    return vec![
      Segment::new(corners[0].x, corners[0].y, corners[1].x, corners[1].y),
      Segment::new(corners[1].x, corners[1].y, corners[2].x, corners[2].y),
      Segment::new(corners[2].x, corners[2].y, corners[3].x, corners[3].y),
      Segment::new(corners[3].x, corners[3].y, corners[0].x, corners[0].y),
    ];
  }

  pub fn intersect_point(&self, x: f32, y: f32) -> bool {
    return x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h;
  }

  pub fn draw(&self, color: Option<olc::Pixel>) {
    olc::draw_rect(
      self.x as i32,
      self.y as i32,
      self.w as i32,
      self.h as i32,
      match color {
        Some(t) => t,
        None => olc::BLACK,
      },
    );
  }
}
