extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./point.rs"]
mod point;

#[path = "./shared.rs"]
mod shared;

use point::Point;
use shared::get_outer_tangents;

const ERROR: f32 = 8.0;
const DPR: f32 = 0.75;

pub struct Draw {
  point: Point,
  p0: Point,
  p1: Point,
  p2: Point,
  p3: Point,
  prev: Point,
  pressure: f32,
  streamline: f32,
  points: Vec<Point>,
  is_complete: bool,
  error: f32,
}

impl Draw {
  pub fn new(x: f32, y: f32) -> Draw {
    Draw {
      point: Point::new(x, y),
      p0: Point::new(x, y),
      p1: Point::new(x, y),
      p2: Point::new(x, y),
      p3: Point::new(x, y),
      prev: Point::new(0.0, 0.0),
      pressure: 0.125,
      streamline: 0.25,
      points: vec![Point::new(0.0, 0.0), Point::new(0.0, 0.0)],
      is_complete: false,
      error: ERROR,
    }
  }

  pub fn add_point(&mut self, x: f32, y: f32) {
    // if the point is the same as last frame, bail.
    if x == self.p3.x && y == self.p3.y {
      return;
    }

    // shift points
    self.p0 = self.p1;
    self.p1 = self.p2;
    self.p2 = self.p3;
    self.p3 = Point { x, y };

    // pop last point
    self.points.pop();

    // simulate pressure
    let distance = self.p2.dist_point(&self.p3);
    if distance > 0.0 {
      let p = self.pressure;
      let sp = (distance / 12.0).min(1.0);
      self.pressure = (p + (1.0 - sp).min(1.0) - p) * (sp * 0.275).min(1.0).max(0.0);
    };

    // Get the point in local space
    let mut local_point = Point::new(x - self.point.x, y - self.point.y);

    let mut min_x = local_point.x.min(0.0);
    let mut min_y = local_point.y.min(0.0);

    // Find the difference in direction
    let u1 = self.p0.get_tan_point(&self.p1);
    let u3 = self.p2.get_tan_point(&self.p3);
    let dpr = u1.get_dot(&u3);

    if dpr.is_nan() {
      // the dpr will be NAN for the first two points,
      // and we always want to push these to the line
      self.points.push(local_point);
      self.prev = local_point;
    } else {
      // Reduce the error by an amount proportional to the dot product
      self.error -= 1.0 - ((dpr - 1.0) / (1.0 - DPR)) * (ERROR - 1.0);

      // If the error has run out, push a point to the line
      if (self.error < 0.0) {
        let mut streamlined_point = local_point.clone();
        streamlined_point.lrp(&self.prev, self.streamline);

        min_x = min_x.min(streamlined_point.x);
        min_y = min_y.min(streamlined_point.y);
        self.points.push(streamlined_point);
        self.prev = streamlined_point;
        self.error = ERROR;
      }
    }

    // now push the local point
    self.points.push(local_point);

    // if the new top left is above / left of x=0,0
    // offset each point (and the shape's point)
    // so that all points are above zero
    if min_x < 0.0 || min_y < 0.0 {
      self.points.iter_mut().for_each(|point| {
        point.x -= min_x;
        point.y -= min_y;
      });

      self.point.x += min_x;
      self.point.y += min_y;
    }
  }

  pub fn complete(&mut self) {
    self.is_complete = true;
  }

  pub fn draw(&self, color: olc::Pixel) {
    for i in 1..self.points.len() {
      let p0 = &self.points[i - 1];
      let p1 = &self.points[i];
      olc::draw_line(
        (p0.x + self.point.x) as i32,
        (p0.y + self.point.y) as i32,
        (p1.x + self.point.x) as i32,
        (p1.y + self.point.y) as i32,
        color,
      );
    }
  }
}
