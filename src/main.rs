use bracket_lib::prelude::*;
use legion::*;

mod constants;
mod net;
mod player;
mod structs;
mod utils;

use crate::constants::{HEIGHT, WIDTH};
use crate::player::*;
use crate::structs::*;
use crate::utils::*;

pub fn main() -> BError {
  let context = BTermBuilder::simple(WIDTH, HEIGHT)
    .unwrap()
    .with_title("Title")
    .with_vsync(false)
    .with_advanced_input(true)
    .build()?;

  let mut gs = State::new();

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

  main_loop(context, gs)
}
