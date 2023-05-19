// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod vector;
mod draw;
mod canvas;
mod kinematic;

use game_loop::game_loop;
use draw::draw_grid;
use vector::Vector;
use web_sys::WebGl2RenderingContext;

use wasm_bindgen::prelude::*;
use canvas::get_context;

pub struct State {
  dimensions: Vector
}

impl State {
  fn new(dimensions: Vector) -> State {
    State {
      dimensions
    }
  }

  fn update(&mut self, _time: f64) {
    //self.emitter.update(time, self.dimensions);
  }

  fn render(&self, ctx: &WebGl2RenderingContext) {
    draw_grid(ctx, &self);
  }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let dimensions = Vector(800.0, 800.0);
  let game = State::new(dimensions);
  let ctx = get_context()?;

  //game.emitter.max_particles = 10000;

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&ctx);
  });

  Ok(())
}
