use rltk::{RGB, RGBA};

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
