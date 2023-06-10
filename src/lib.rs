// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod vector;
mod draw;
mod canvas;
mod kinematic;
mod program_stage;

use game_loop::game_loop;
use draw::draw_stage;
use vector::Vector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use program_stage::{get_stage_program};

use wasm_bindgen::prelude::*;
use canvas::get_context;

pub struct Game {
  dimensions: Vector,
  stage_vertex_buffer: WebGlBuffer
}

impl Game {
  fn new(
    dimensions: Vector,
    stage_vertex_buffer: WebGlBuffer
  ) -> Game {
    Game {
      dimensions,
      stage_vertex_buffer
    }
  }

  fn update(&mut self, _time: f64) {
    //self.emitter.update(time, self.dimensions);
  }

  fn render(&self, ctx: &WebGl2RenderingContext) {

    ctx.viewport(
      0,
      0,
      self.dimensions.0 as i32,
      self.dimensions.1 as i32
    );

    draw_stage(ctx, &self.stage_vertex_buffer);

  }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let dimensions = Vector(800.0, 800.0);
  let ctx = get_context()?;

  let stage_vertex_buffer = ctx.create_buffer().unwrap();
  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&stage_vertex_buffer));

  let stage_program = get_stage_program(&ctx)?;

  ctx.use_program(Some(&stage_program));

  if let Some(resolution_location) = ctx.get_uniform_location(&stage_program, "u_resolution") {
    ctx.uniform2f(Some(&resolution_location), dimensions.0 as f32, dimensions.1 as f32);
  }

  let game = Game::new(
    dimensions,
    stage_vertex_buffer
  );

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&ctx);
  });

  Ok(())
}
