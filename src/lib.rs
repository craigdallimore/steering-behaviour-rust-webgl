// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod vector;
mod draw;
mod canvas;
mod kinematic;
mod program_stage;
mod program_arrow;

use game_loop::game_loop;
use draw::{draw_stage, draw_arrow};
use vector::Vector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};
use program_stage::{get_stage_program, setup_stage_program, buffer_stage_data};
use program_arrow::{get_arrow_program, setup_arrow_program, buffer_arrow_data};

use wasm_bindgen::prelude::*;
use canvas::get_context;

pub struct Game {
  dimensions: Vector,
  stage_program: WebGlProgram,
  arrow_program: WebGlProgram
}

impl Game {
  fn new(
    dimensions: Vector,
    stage_program: WebGlProgram,
    arrow_program: WebGlProgram
  ) -> Game {
    Game {
      dimensions,
      stage_program,
      arrow_program,
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

    ctx.use_program(Some(&self.stage_program));
    buffer_stage_data(&ctx);
    draw_stage(ctx);

    ctx.use_program(Some(&self.arrow_program));
    buffer_arrow_data(&ctx);
    draw_arrow(ctx);

  }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let dimensions = Vector(800.0, 800.0);
  let ctx = get_context()?;

  let stage_vertex_buffer = ctx.create_buffer().unwrap();
  let arrow_vertex_buffer = ctx.create_buffer().unwrap();

  let stage_program = get_stage_program(&ctx)?;
  let arrow_program = get_arrow_program(&ctx)?;

  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&stage_vertex_buffer));
  setup_stage_program(&ctx, &stage_program, &dimensions);

  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&arrow_vertex_buffer));
  setup_arrow_program(&ctx, &arrow_program);


  let game = Game::new(
    dimensions,
    stage_program,
    arrow_program,
  );

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&ctx);
  });

  Ok(())
}
