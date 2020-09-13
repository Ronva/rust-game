use rltk::RGB;

use crate::utils;

// Components

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
  pub glyph: char,
  pub fg: RGB,
  pub bg: RGB,
}

impl Renderable {
  pub fn transparentize(&mut self, alpha: f32) {
    self.fg = utils::apply_opacity(self.fg, self.bg, alpha);
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
  pub id: String
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ignore {}

// Components end
