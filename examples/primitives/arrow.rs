extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

#[path = "./point.rs"]
mod point;
#[path = "./rect.rs"]
mod rect;
#[path = "./segment.rs"]
mod segment;

use point::Point;
use rect::Rect;
use segment::Segment;

pub fn draw_arrow(from: &Rect, to: &Rect, color: Option<olc::Pixel>) {
  let center1 = from.get_center();
  let center2 = to.get_center();

  let seg = &mut Segment::new(center1.x, center1.y, center2.x, center2.y);
  let u = seg.get_direction();

  if seg.x1 == seg.x2 && seg.y1 == seg.y2 {
    return;
  }

  match from
    .get_segments()
    .iter()
    .find(|edge| edge.get_line_intersection(seg).is_some())
  {
    Some(edge) => {
      let int = edge.get_line_intersection(seg).unwrap();
      seg.x1 = int.x;
      seg.y1 = int.y;
    }
    None => {
      seg.x1 = center1.x;
      seg.y1 = center1.y;
    }
  }

  match to
    .get_segments()
    .iter()
    .find(|edge| edge.get_line_intersection(seg).is_some())
  {
    Some(edge) => {
      let int = edge.get_line_intersection(seg).unwrap();
      seg.x2 = int.x;
      seg.y2 = int.y;
    }
    None => {
      seg.x2 = center2.x;
      seg.y2 = center2.y;
    }
  }

  let len = seg.get_length();
  if len < 20.0 || seg.get_start().get_dot(&seg.get_end()) < 0.0 {
    seg.x2 = seg.x1 + 10.0 * u.x;
    seg.y2 = seg.y1 + 10.0 * u.y;
  } else {
    seg.x2 -= 10.0 * u.x;
    seg.y2 -= 10.0 * u.y;
  }

  olc::draw_line(
    seg.x1 as i32,
    seg.y1 as i32,
    seg.x2 as i32,
    seg.y2 as i32,
    match color {
      Some(t) => t,
      None => olc::BLACK,
    },
  );
  // Head
  let arrow_head_length = f32::min(12.0, seg.get_length() / 2.0);

  let ac = Point::new(
    seg.x2 - arrow_head_length * u.x,
    seg.y2 - arrow_head_length * u.y,
  );
  let r = std::f32::consts::PI / 6.0;

  let pt1 = &mut ac.clone();
  let pt2 = &mut ac.clone();

  pt1.rot_with(seg.x2, seg.y2, r);
  pt2.rot_with(seg.x2, seg.y2, -r);

  olc::draw_line(
    pt1.x as i32,
    pt1.y as i32,
    seg.x2 as i32,
    seg.y2 as i32,
    olc::BLACK,
  );

  olc::draw_line(
    pt2.x as i32,
    pt2.y as i32,
    seg.x2 as i32,
    seg.y2 as i32,
    olc::BLACK,
  );
}
