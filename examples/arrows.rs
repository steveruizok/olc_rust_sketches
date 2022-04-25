extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct Arrow {
  from: usize,
  to: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
  x: f32,
  y: f32,
}

impl Point {
  fn new(x: f32, y: f32) -> Point {
    Point { x, y }
  }

  fn add(&mut self, x: f32, y: f32) -> &mut Point {
    self.x += x;
    self.y += y;
    return self;
  }

  fn add_point(&mut self, other: &Point) -> &mut Point {
    self.x += other.x;
    self.y += other.y;
    return self;
  }

  fn sub(&mut self, x: f32, y: f32) -> &mut Point {
    self.x -= x;
    self.y -= y;
    return self;
  }

  fn div(&mut self, x: f32, y: f32) -> &mut Point {
    self.x /= x;
    self.y /= y;
    return self;
  }

  fn mul(&mut self, x: f32, y: f32) -> &mut Point {
    self.x *= x;
    self.y *= y;
    return self;
  }
  fn uni(&mut self) -> &mut Point {
    let length = self.length();
    self.x /= length;
    self.y /= length;
    return self;
  }

  fn get_dot(&self, other: &Point) -> f32 {
    self.x * other.x + self.y * other.y
  }

  fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  fn get_tan(&self, x: f32, y: f32) -> Point {
    let new_pt = &mut self.clone();
    new_pt.sub(x, y).uni();
    return Point {
      x: new_pt.x,
      y: new_pt.y,
    };
  }
  fn rot_with(&mut self, x: f32, y: f32, r: f32) -> &mut Point {
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

  fn clone(&self) -> Point {
    let pt = Point {
      x: self.x,
      y: self.y,
    };
    return pt;
  }

  fn sub_point(&mut self, other: &Point) -> &mut Point {
    self.x -= other.x;
    self.y -= other.y;
    return self;
  }

  fn draw(&self, color: Option<olc::Pixel>) {
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

struct Segment {
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32,
}

impl Segment {
  fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Segment {
    Segment { x1, y1, x2, y2 }
  }

  fn get_start(&self) -> Point {
    Point::new(self.x1, self.y1)
  }

  fn get_end(&self) -> Point {
    Point::new(self.x2, self.y2)
  }

  fn get_center(&self) -> Point {
    Point::new((self.x1 + self.x2) / 2.0, (self.y1 + self.y2) / 2.0)
  }

  fn get_length(&self) -> f32 {
    ((self.x1 - self.x2) * (self.x1 - self.x2) + (self.y1 - self.y2) * (self.y1 - self.y2)).sqrt()
  }

  fn get_direction(&self) -> Point {
    let length = self.get_length();
    let ux = (self.x2 - self.x1) / length;
    let uy = (self.y2 - self.y1) / length;
    return Point::new(ux, uy);
  }

  fn get_line_intersection(&self, seg: &mut Segment) -> Option<Point> {
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

  fn draw(&self, color: Option<olc::Pixel>) {
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

struct Rect {
  x: f32,
  y: f32,
  w: f32,
  h: f32,
}

impl Rect {
  fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
  }

  fn get_center(&self) -> Point {
    Point {
      x: self.x + (self.w / 2.0),
      y: self.y + (self.h / 2.0),
    }
  }

  fn get_corners(&self) -> Vec<Point> {
    vec![
      Point::new(self.x, self.y),
      Point::new(self.x + self.w, self.y),
      Point::new(self.x + self.w, self.y + self.h),
      Point::new(self.x, self.y + self.h),
    ]
  }

  fn get_segments(&self) -> Vec<Segment> {
    let corners = self.get_corners();
    return vec![
      Segment::new(corners[0].x, corners[0].y, corners[1].x, corners[1].y),
      Segment::new(corners[1].x, corners[1].y, corners[2].x, corners[2].y),
      Segment::new(corners[2].x, corners[2].y, corners[3].x, corners[3].y),
      Segment::new(corners[3].x, corners[3].y, corners[0].x, corners[0].y),
    ];
  }

  fn intersect_point(&self, x: f32, y: f32) -> bool {
    return x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h;
  }

  fn draw(&self, color: Option<olc::Pixel>) {
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

struct Arrows {
  rects: Vec<Rect>,
  arrows: Vec<Arrow>,
  width: i32,
  height: i32,
  px: i32,
  py: i32,
  selected: Option<i32>,
}

impl Arrows {
  fn initialize(&mut self) {
    self.width = olc::screen_width();
    self.height = olc::screen_height();

    olc::clear(olc::Pixel::rgb(240, 191, 58));
  }

  fn render(&mut self) {
    self.rects.iter().for_each(|rect| {
      rect.draw(None);
    });

    // self.arrow.draw(None);
    // rect1.get_corners().iter().for_each(|p| p.draw(None));
    // rect2.get_corners().iter().for_each(|p| p.draw(None));

    let arrow = &mut Segment::new(
      self.rects[0].get_center().x,
      self.rects[0].get_center().y,
      self.rects[1].get_center().x,
      self.rects[1].get_center().y,
    );

    let mut i = 0;

    let arrows = self.arrows.iter().for_each(|from_to| {
      let rect1 = self.rects.get(from_to.from).unwrap();
      let rect2 = self.rects.get(from_to.to).unwrap();

      let center1 = rect1.get_center();
      let center2 = rect2.get_center();

      let arrow = &mut Segment::new(center1.x, center1.y, center2.x, center2.y);

      for segment in rect1.get_segments() {
        let int = segment.get_line_intersection(arrow);
        match int {
          Some(p) => {
            arrow.x1 = p.x;
            arrow.y1 = p.y;
            break;
          }
          None => {}
        }
      }

      for segment in rect2.get_segments() {
        let int = segment.get_line_intersection(arrow);
        match int {
          Some(p) => {
            arrow.x2 = p.x;
            arrow.y2 = p.y;
            break;
          }
          None => {}
        }
      }

      let u = arrow.get_direction();
      arrow.x1 += 10.0 * u.x;
      arrow.y1 += 10.0 * u.y;
      arrow.x2 -= 10.0 * u.x;
      arrow.y2 -= 10.0 * u.y;

      arrow.draw(None);

      // Arrowhead

      let arrow_head_length = f32::min(12.0, arrow.get_length() / 2.0);

      let ac = Point::new(
        arrow.x2 - arrow_head_length * u.x,
        arrow.y2 - arrow_head_length * u.y,
      );
      let r = std::f32::consts::PI / 6.0;

      let pt1 = &mut ac.clone();
      let pt2 = &mut ac.clone();

      pt1.rot_with(arrow.x2, arrow.y2, r);
      pt2.rot_with(arrow.x2, arrow.y2, -r);

      olc::draw_line(
        pt1.x as i32,
        pt1.y as i32,
        arrow.x2 as i32,
        arrow.y2 as i32,
        olc::BLACK,
      );

      olc::draw_line(
        pt2.x as i32,
        pt2.y as i32,
        arrow.x2 as i32,
        arrow.y2 as i32,
        olc::BLACK,
      );
    });
  }
}

impl olc::Application for Arrows {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.initialize();
    self.render();
    self.px = olc::get_mouse_x();
    self.py = olc::get_mouse_y();
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(240, 191, 58));
    self.render();

    let mouse = olc::get_mouse(0);
    let x = olc::get_mouse_x();
    let y = olc::get_mouse_y();
    let dx = x - self.px;
    let dy = y - self.py;
    self.px = x;
    self.py = y;

    if mouse.held {
      let mut i = 0;
      for rect in self.rects.iter() {
        if rect.intersect_point(x as f32, y as f32) {
          self.selected = Some(i);
        }
        i += 1;
      }

      if self.selected.is_some() {
        let idx = self.selected.unwrap() as usize;
        if let Some(rect) = self.rects.get_mut(idx) {
          rect.x += dx as f32;
          rect.y += dy as f32;
        }
      }
    } else if self.selected.is_some() {
      self.selected = None;
    }

    if olc::get_mouse(0).released {
      self.selected = None;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Arrows {
    rects: vec![
      Rect::new(0.0, 0.0, 50.0, 50.0),
      Rect::new(75.0, 50.0, 50.0, 50.0),
      Rect::new(25.0, 100.0, 50.0, 50.0),
    ],
    arrows: vec![Arrow { from: 0, to: 1 }, Arrow { from: 1, to: 2 }],
    width: 0,
    height: 0,
    px: 0,
    py: 0,
    selected: None,
  };
  olc::start("Arrows", &mut app, 350, 240, 2, 2).unwrap();
}
