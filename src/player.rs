use bracket_lib::prelude as bracket;
use bracket_lib::prelude::*;
use legion::world::Entry;

use crate::net;
use crate::structs::*;

pub fn create_player(gs: &mut State, player: Player, pos: Position) {
  let id = player.id.clone();
  let entity = gs.ecs.push((
    pos,
    Renderable {
      glyph: '@',
      fg: RGB::named(bracket::WHITE),
      bg: RGB::named(bracket::BLACK),
    },
    player,
  ));
  gs.players.insert(id, entity);
}

pub fn get_player_info(info: &str) -> Option<(Player, Position)> {
  let player_info: Vec<&str> = info.split(",").collect();
  let mut info = None;
  if player_info.len() == 3 {
    let player = Player {
      id: String::from(player_info[0]),
    };
    let pos = Position {
      x: player_info[1].parse().unwrap(),
      y: player_info[2].parse().unwrap(),
    };
    info = Some((player, pos));
  }
  info
}

pub fn get_player_entry(gs: &mut State, player_id: String) -> Option<Entry> {
  if let Some(entity) = gs.players.get(&player_id) {
    let entry = gs.ecs.entry(*entity).unwrap();
    Some(entry)
  } else {
    None
  }
}

pub fn move_player(gs: &mut State, player_id: String, x: i32, y: i32) {
  if let Some(mut entry) = get_player_entry(gs, player_id) {
    entry.remove_component::<Position>();
    entry.add_component::<Position>(Position { x, y });
  }
}

pub fn move_player_delta(gs: &mut State, player_id: String, delta_x: i32, delta_y: i32) {
  if let Some(mut entry) = get_player_entry(gs, player_id) {
    let mut pos: Position = *entry.get_component_mut::<Position>().unwrap();
    pos.x = pos.x + delta_x;
    pos.y = pos.y + delta_y;
    entry.remove_component::<Position>();
    entry.add_component::<Position>(pos);
  }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
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
