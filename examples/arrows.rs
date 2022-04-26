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
use rect::Rect;
use segment::Segment;

struct ArrowLink {
  from: usize,
  to: usize,
}

struct Arrows {
  rects: Vec<Rect>,
  arrows: Vec<ArrowLink>,
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

    self.arrows.iter().for_each(|arrow| {
      let rect1 = self.rects.get(arrow.from).unwrap();
      let rect2 = self.rects.get(arrow.to).unwrap();
      arrow::draw_arrow(rect1, rect2, None);
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
      if self.selected.is_none() {
        for rect in self.rects.iter() {
          if rect.intersect_point(x as f32, y as f32) {
            self.selected = Some(i);
            break;
          }
          i += 1;
        }
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
  let rect0 = Rect::new(0.0, 0.0, 50.0, 50.0);
  let rect1 = Rect::new(75.0, 50.0, 50.0, 50.0);
  let rect2 = Rect::new(25.0, 100.0, 50.0, 50.0);

  let mut app = Arrows {
    rects: vec![rect0, rect1, rect2],
    arrows: vec![ArrowLink { from: 0, to: 1 }, ArrowLink { from: 1, to: 2 }],
    width: 0,
    height: 0,
    px: 0,
    py: 0,
    selected: None,
  };
  olc::start("Arrows", &mut app, 350, 240, 2, 2).unwrap();
}
