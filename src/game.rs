use legion::world::Entry;
use legion::*;
use rltk::prelude::*;
use rltk::{BTerm, GameState, RGB};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;

use crate::constants::*;
use crate::net;
use crate::structs::*;
use crate::utils;

pub struct State {
  pub ecs: World,
  pub socket: UdpSocket,
  pub players: HashMap<String, Entity>,
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();

    // listen for UDP messages
    let mut buf = vec![0_u8; 536870912];
    match self.socket.recv(&mut buf) {
      Ok(received) => {
        let decoded = str::from_utf8(&buf[..received]).unwrap();
        self.process_server_data(String::from(decoded));
      }
      _ => {}
    }

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

impl State {
  fn create_player(&mut self, player_id: String, x: i32, y: i32) {
    let entity: Entity = self.ecs.push((
      Position { x: x, y: y },
      Renderable {
        glyph: '@',
        fg: RGB::named(rltk::WHITE),
        bg: RGB::named(rltk::BLACK),
      },
      Player {},
    ));
    self.players.insert(player_id, entity);
  }
  fn process_server_data(&mut self, data: String) {
    let strings: Vec<&str> = data.split(":").collect();
    let (op, info) = (strings[0], strings[1]);
    match op {
      // c: current player has connected
      "c" => {
        let players: Vec<&str> = info.split(";").collect();
        for player in players.iter() {
          if let Some(PlayerInfo { id, x, y }) = self.get_player_info(player) {
            self.create_player(id, x, y);
          }
        }
      }
      // u: a player's position has been updated
      "u" => {
        if let Some(PlayerInfo { id, x, y }) = self.get_player_info(info) {
          let player_id = id.clone();
          match get_player_entry(self, player_id) {
            // if player exists move them, otherwise create them
            Some(_entry) => move_player(self, id, x, y),
            None => self.create_player(id, x, y),
          }
        }
      }
      _ => {}
    }
  }
  fn get_player_info(&mut self, info: &str) -> Option<PlayerInfo> {
    let player_info: Vec<&str> = info.split(",").collect();
    let mut info = None;
    if player_info.len() == 3 {
      let player_id = String::from(player_info[0]);
      let x = player_info[1].parse().unwrap();
      let y = player_info[2].parse().unwrap();
      info = Some(PlayerInfo {
        id: player_id,
        x: x,
        y: y,
      })
    }
    info
  }
}

fn get_player_entry(gs: &mut State, player_id: String) -> Option<Entry> {
  let mut pos = None;
  if let Some(entity) = gs.players.get(&player_id) {
    if let Some(entry) = gs.ecs.entry(*entity) {
      pos = Some(entry)
    }
  }
  pos
}

fn move_player(gs: &mut State, player_id: String, x: i32, y: i32) {
  if let Some(mut entry) = get_player_entry(gs, player_id) {
    entry.remove_component::<Position>();
    entry.add_component::<Position>(Position { x: x, y: y });
  }
}

fn move_player_delta(gs: &mut State, player_id: String, delta_x: i32, delta_y: i32) {
  if let Some(mut entry) = get_player_entry(gs, player_id) {
    let mut pos: Position = *entry.get_component_mut::<Position>().unwrap();
    pos.x = pos.x + delta_x;
    pos.y = pos.y + delta_y;
    entry.remove_component::<Position>();
    entry.add_component::<Position>(pos);
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

  utils::draw_stars(&mut gs);
  // utils::draw_ascii(&mut gs, PLANET, 3, 3);
  gs.create_player(String::from("me"), 0, 0);

  rltk::main_loop(context, gs)
}
