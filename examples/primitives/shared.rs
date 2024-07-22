#[path = "./point.rs"]
mod point;

use point::Point;

pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = (std::f32::consts::PI) / 2.0;
pub const PI2: f32 = (std::f32::consts::PI) * 2.0;

pub fn get_outer_tangents(x1: f32, y1: f32, r1: f32, x2: f32, y2: f32, r2: f32) -> Vec<Point> {
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

  // Second circle bottom tangent point
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

  return if x1 <= x2 {
    vec![t2, t3, t0, t1]
  } else {
    vec![t0, t1, t2, t3]
  };
}
