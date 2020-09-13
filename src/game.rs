use legion::*;
use rltk::prelude::*;
use rltk::{BTerm, GameState, RGB};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;

use crate::constants::*;
use crate::net;
use crate::structs::*;
use crate::utils::*;

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
      ctx.set(
        pos.x,
        pos.y,
        render.fg,
        render.bg,
        rltk::to_cp437(render.glyph),
      );
    }
  }
}

pub fn process_server_data(gs: &mut State, data: String) {
  let strings: Vec<&str> = data.split(":").collect();
  let (op, info) = (strings[0], strings[1]);
  match op {
    // c: current player has connected
    "c" => {
      let players: Vec<&str> = info.split(";").collect();
      for p in players.iter() {
        if let Some((player, pos)) = get_player_info(p) {
          create_player(gs, player, pos);
        }
      }
    }
    // u: a player's position has been updated
    "u" => {
      if let Some((player, pos)) = get_player_info(info) {
        let id = player.id.clone();
        if let Some(_entry) = get_player_entry(gs, id) {
          move_player(gs, player.id, pos.x, pos.y)
        } else {
          create_player(gs, player, pos)
        }
      }
    }
    _ => {}
  }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
  let id = String::from("me");
  // Player movement
  match ctx.key {
    None => {}
    Some(key) => match key {
      VirtualKeyCode::Left => {
        move_player_delta(gs, id, -1, 0);
        net::send_to_server(&mut gs.socket, b"ml");
      }
      VirtualKeyCode::Right => {
        move_player_delta(gs, id, 1, 0);
        net::send_to_server(&mut gs.socket, b"mr");
      }
      VirtualKeyCode::Up => {
        move_player_delta(gs, id, 0, -1);
        net::send_to_server(&mut gs.socket, b"mu");
      }
      VirtualKeyCode::Down => {
        move_player_delta(gs, id, 0, 1);
        net::send_to_server(&mut gs.socket, b"md");
      }
      _ => {}
    },
  }
}

pub fn run(socket: UdpSocket) -> rltk::BError {
  let context = RltkBuilder::simple(WIDTH, HEIGHT).unwrap().build()?;

  let mut gs = State {
    ecs: World::default(),
    socket: socket,
    players: HashMap::new(),
  };

  let stars = generate_stars();
  let _entities: &[Entity] = gs.ecs.extend(stars);
  // draw_ascii(&mut gs, PLANET, 3, 3);
  create_player(
    &mut gs,
    Player {
      id: String::from("me"),
    },
    Position { x: 0, y: 0 },
  );

  rltk::main_loop(context, gs)
}
