use bracket_lib::prelude as bracket;
use bracket_lib::prelude::*;
use std::str;

use crate::constants::{HEIGHT, WIDTH};
use crate::structs::*;

pub fn rgba_to_rgb(fg: RGBA, bg: RGB) -> RGB {
  let r = ((1.0 - fg.a) * bg.r) + (fg.a * fg.r);
  let g = ((1.0 - fg.a) * bg.g) + (fg.a * fg.g);
  let b = ((1.0 - fg.a) * bg.b) + (fg.a * fg.b);
  RGB::from_f32(r, g, b)
}

pub fn apply_opacity(color: RGB, bg: RGB, alpha: f32) -> RGB {
  let with_alpha = RGBA::from_f32(color.r, color.g, color.b, alpha);
  rgba_to_rgb(with_alpha, bg)
}

pub fn render_glyph(gs: &mut State, c: char, x: i32, y: i32, fg: RGB, bg: RGB) {
  if c != ' ' {
    gs.ecs.push((
      Position { x: x, y: y },
      Renderable {
        glyph: c,
        fg: fg,
        bg: bg,
      },
    ));
  }
}

pub fn draw_ascii(gs: &mut State, ascii: &str, x_offset: i32, y_offset: i32) {
  for (i, line) in ascii.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      let (x, y) = (j as i32 + x_offset, i as i32 + y_offset);
      render_glyph(
        gs,
        c,
        x,
        y,
        RGB::named(bracket::WHITE),
        RGB::named(bracket::BLACK),
      )
    }
  }
}

pub fn generate_stars() -> Vec<(Renderable, Position)> {
  let mut rng = RandomNumberGenerator::new();
  let mut noise = FastNoise::seeded(rng.next_u64());
  noise.set_noise_type(NoiseType::WhiteNoise);
  noise.set_fractal_octaves(5);
  noise.set_fractal_gain(0.5);
  noise.set_fractal_lacunarity(4.0);
  noise.set_frequency(2.0);

  let mut stars = Vec::new();
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let noise_x = WIDTH as f32;
      let noise_y = HEIGHT as f32;
      let n = noise.get_noise((x as f32) / noise_x, (y as f32) / noise_y);
      let should_show: f32 = rng.rand();

      if n > 0.0 && n < 0.8 && should_show > 0.75 {
        let mut renderable = Renderable {
          glyph: '.',
          fg: RGB::named(bracket::WHITE),
          bg: RGB::named(bracket::BLACK),
        };
        renderable.transparentize(n);
        let position = Position { x, y };
        stars.push((renderable, position));
      }
    }
  }

  stars
}
