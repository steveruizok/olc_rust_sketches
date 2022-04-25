extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct Bounce {
  x: f32,
  y: f32,
  dx: f32,
  dy: f32,
  speed: f32,
  radius: f32,
  width: i32,
  height: i32,
}

impl Bounce {
  fn initialize(&mut self) {
    self.width = olc::screen_width();
    self.height = olc::screen_height();

    self.speed = 2.0;

    self.dx = self.speed * ((olc::c_rand() % 10) as f32 / 10.0);
    self.dy = self.speed * ((olc::c_rand() % 10) as f32 / 10.0);
    olc::set_pixel_mode(olc::PixelMode::ALPHA);
    olc::clear(olc::Pixel::rgb(240, 191, 58));
  }
}

impl olc::Application for Bounce {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.initialize();
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::fill_rect(
      0,
      0,
      self.width,
      self.height,
      olc::Pixel::rgba(240, 191, 58, 5),
    );

    // Deflect the ball using clicks

    if olc::get_mouse(0).pressed {
      let mouse_x = olc::get_mouse_x();
      let mouse_y = olc::get_mouse_y();

      let dx = mouse_x as f32 - self.x;
      let dy = mouse_y as f32 - self.y;

      let distance = (dx * dx + dy * dy).sqrt();
      self.speed = (100.0 - distance).max(25.0).min(100.0) / 10.0;

      self.dx = (self.dx + ((dx / distance) * self.speed)).min(3.0);
      self.dy = (self.dy + ((dy / distance) * self.speed)).min(3.0);
    }

    // Bounce the ball off of the walls

    let mut bounce_x = false;
    let mut bounce_y = false;

    self.x += -(self.dx / 10.0);
    self.y += -(self.dy / 10.0);

    if self.x > (self.width as f32 - (self.radius)) {
      self.x = self.width as f32 - (self.radius);
      bounce_x = true;
    }

    if self.x < self.radius {
      self.x = self.radius;
      bounce_x = true;
    }

    if self.y > (self.height as f32 - (self.radius)) {
      self.y = self.height as f32 - (self.radius);
      bounce_y = true;
    }

    if self.y < self.radius {
      self.y = self.radius;
      bounce_y = true;
    }

    if bounce_x {
      self.dx = -self.dx;
    }
    if bounce_y {
      self.dy = -self.dy;
    }
    // Display

    olc::fill_circle(self.x as i32, self.y as i32, self.radius as i32, olc::BLACK);
    olc::fill_circle(
      self.x as i32,
      self.y as i32,
      self.radius as i32 - 2,
      olc::WHITE,
    );

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Bounce {
    x: 10.0,
    y: 10.0,
    dx: 0.0,
    dy: 0.0,
    speed: 0.0,
    radius: 10.0,
    width: 0,
    height: 0,
  };
  olc::start("Bounce", &mut app, 350, 240, 2, 2).unwrap();
}
