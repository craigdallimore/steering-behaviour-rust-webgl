// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod vector;
mod draw;
mod canvas;
mod kinematic;
mod program_stage;

use game_loop::game_loop;
//use draw::draw_grid;
use vector::Vector;
use web_sys::WebGl2RenderingContext;
use program_stage::{get_stage_program};

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

    //draw_grid(ctx, &self);

    ctx.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0, // starting point
      6  // number of points to draw
    );

  }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let dimensions = Vector(800.0, 800.0);
  let game = State::new(dimensions);
  let ctx = get_context()?;

  let vertex_buffer = ctx.create_buffer().unwrap();
  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

  let stage_program = get_stage_program(&ctx)?;

  ctx.use_program(Some(&stage_program));

  if let Some(resolution_location) = ctx.get_uniform_location(&stage_program, "u_resolution") {
    ctx.uniform2f(Some(&resolution_location), dimensions.0 as f32, dimensions.1 as f32);
  }

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&ctx);
  });

  Ok(())
}
