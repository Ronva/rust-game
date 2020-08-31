use rltk::prelude::*;
use rltk::{BTerm, GameState, RGB};
use specs::prelude::*;
use specs_derive::Component;

mod utils;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

// prettier-ignore
const PLANET: &str = r#"
                                             ___
                                          ,o88888
                                       ,o8888888'
                 ,:o:o:oooo.        ,8O88Pd8888"
             ,.::.::o:ooooOoOoO. ,oO8O8Pd888'"
           ,.:.::o:ooOoOoOO8O8OOo.8OOPd8O8O"
          , ..:.::o:ooOoOOOO8OOOOo.FdO8O8"
         , ..:.::o:ooOoOO8O888O8O,COCOO"
        , . ..:.::o:ooOoOOOO8OOOOCOCO"
         . ..:.::o:ooOoOoOO8O8OCCCC"o
            . ..:.::o:ooooOoCoCCC"o:o
            . ..:.::o:o:,cooooCo"oo:o:
         `   . . ..:.:cocoooo"'o:o:::'
         .`   . ..::ccccoc"'o:o:o:::'
        :.:.    ,c:cccc"':.:.:.:.:.'
      ..:.:"'`::::c:"'..:.:.:.:.:.'
    ...:.'.:.::::"'    . . . . .'
   .. . ....:."' `   .  . . ''
 . . . ...."'
 .. . ."'
"#;

const PLAYER: &str = r#"
  _
 /┼\
 \░/
"#;

#[derive(Component)]
struct Position {
  x: i32,
  y: i32,
}

#[derive(Component)]
struct Renderable {
  glyph: char,
  fg: RGB,
  bg: RGB,
}

impl Renderable {
  fn transparentize(&mut self, alpha: f32) {
    use utils::apply_opacity;
    self.fg = apply_opacity(self.fg, self.bg, alpha);
  }
}

struct State {
  ecs: World,
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    let positions = self.ecs.read_storage::<Position>();
    let renderables = self.ecs.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
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

fn render_glyph(gs: &mut State, c: char, x: i32, y: i32, fg: RGB, bg: RGB) {
  if c != ' ' {
    gs.ecs
      .create_entity()
      .with(Position { x: x, y: y })
      .with(Renderable {
        glyph: c,
        fg: fg,
        bg: bg,
      })
      .build();
  }
}

fn draw_ascii(gs: &mut State, ascii: &str, x_offset: i32, y_offset: i32) {
  for (i, line) in ascii.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      let (x, y) = (j as i32 + x_offset, i as i32 + y_offset);
      render_glyph(
        gs,
        c,
        x,
        y,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
      )
    }
  }
}

fn draw_stars(gs: &mut State) {
  let mut rng = RandomNumberGenerator::new();
  let mut noise = FastNoise::seeded(rng.next_u64());
  noise.set_noise_type(NoiseType::WhiteNoise);
  noise.set_fractal_octaves(5);
  noise.set_fractal_gain(0.5);
  noise.set_fractal_lacunarity(4.0);
  noise.set_frequency(2.0);

  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let noise_x = WIDTH as f32;
      let noise_y = HEIGHT as f32;
      let n = noise.get_noise((x as f32) / noise_x, (y as f32) / noise_y);
      let should_show: f32 = rng.rand();

      if n > 0.0 && n < 0.8 && should_show > 0.75 {
        let mut star = Renderable {
          glyph: '.',
          fg: RGB::named(rltk::WHITE),
          bg: RGB::named(rltk::BLACK),
        };
        star.transparentize(n);

        gs.ecs
          .create_entity()
          .with(Position { x: x, y: y })
          .with(star)
          .build();
      }
    }
  }
}

fn draw_player(gs: &mut State) {
  draw_ascii(gs, PLAYER, 38, 20);
}

embedded_resource!(TILE_FONT, "../resources/scientifica.png");

fn main() -> rltk::BError {
  link_resource!(TILE_FONT, "resources/scientifica.png");

  let context = RltkBuilder::simple80x50().build()?;

  // custom font experiment
  // let context = RltkBuilder::new()
  //   .with_dimensions(WIDTH, HEIGHT)
  //   .with_tile_dimensions(13, 13)
  //   .with_title("My Game")
  //   .with_font("scientifica.png", 13, 13)
  //   .with_simple_console(WIDTH, HEIGHT, "scientifica.png")
  //   .build()?;
  let mut gs = State { ecs: World::new() };

  gs.ecs.register::<Position>();
  gs.ecs.register::<Renderable>();

  draw_stars(&mut gs);
  draw_ascii(&mut gs, PLANET, 3, 3);
  draw_player(&mut gs);

  rltk::main_loop(context, gs)
}
