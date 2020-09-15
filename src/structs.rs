use bracket_lib::prelude::*;
// use doryen_rs::*;
use legion::*;
use std::collections::HashMap;
use std::net::UdpSocket;

use crate::net;
use crate::player::player_input;
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
  pub id: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ignore {}

// Components end

// Game state

pub struct State {
  pub ecs: World,
  pub socket: UdpSocket,
  pub players: HashMap<String, Entity>,
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();

    // listen for UDP messages
    net::udp_listener(self);

    // listen for player input
    player_input(self, ctx);

    let mut query = <(&Renderable, &mut Position)>::query()
      .filter(!component::<Ignore>() & maybe_changed::<Position>());

    for (render, pos) in query.iter_mut(&mut self.ecs) {
      ctx.set(pos.x, pos.y, render.fg, render.bg, to_cp437(render.glyph));
    }
  }
}

impl State {
  pub fn new() -> Self {
    let socket = net::connect_to_server();
    Self {
      ecs: World::default(),
      socket: socket,
      players: HashMap::new(),
    }
  }
}

// Game state end
