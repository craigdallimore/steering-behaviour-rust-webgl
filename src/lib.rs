// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case, non_upper_case_globals)]

mod canvas;
mod domain;
mod draw;
mod program_arrow;
mod program_stage;
mod steering;
mod vector;

use domain::initial_state::{Action, State};
use draw::{draw_arrow, draw_stage};
use game_loop::game_loop;
use program_arrow::{buffer_arrow_data, setup_arrow_program};
use program_stage::{buffer_stage_data, setup_stage_program};
use vector::Vector;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use canvas::{get_context, make_program};
use wasm_bindgen::prelude::*;

pub struct Game {
    state: State,
    dimensions: Vector,
    stage_program: WebGlProgram,
    arrow_program: WebGlProgram,
}

impl Game {
    fn new(dimensions: Vector, stage_program: WebGlProgram, arrow_program: WebGlProgram) -> Game {
        Game {
            dimensions,
            stage_program,
            arrow_program,
            state: State::new(),
        }
    }

    fn update(self: &mut Self, time: f64) {
        let action: Action = Action::Tick(time as f32);
        self.state.dispatch(action);
    }

    fn render(&self, ctx: &WebGl2RenderingContext) {
        // Tell webgl the -1 <-> +1 clipspace maps to 0 <-> dimension{x,y}
        ctx.viewport(0, 0, self.dimensions.0 as i32, self.dimensions.1 as i32);

        ctx.use_program(Some(&self.stage_program));
        buffer_stage_data(&ctx);
        draw_stage(ctx);

        ctx.use_program(Some(&self.arrow_program));

        let kinematics = &self.state.characters.iter().map(|c| c.kinematic).collect();

        buffer_arrow_data(&ctx, &kinematics);
        draw_arrow(ctx, kinematics.len());
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let (ctx, width, height) = get_context()?;

    let dimensions = Vector(width, height);
    let stage_vertex_buffer = ctx.create_buffer().unwrap();
    let arrow_vertex_buffer = ctx.create_buffer().unwrap();

    let stage_program = make_program(&ctx, "vs-stage", "fs-stage")?;
    let arrow_program = make_program(&ctx, "vs-arrow", "fs-arrow")?;

    ctx.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&stage_vertex_buffer),
    );
    setup_stage_program(&ctx, &stage_program);

    ctx.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&arrow_vertex_buffer),
    );
    ctx.use_program(Some(&arrow_program));
    setup_arrow_program(&ctx, &arrow_program, &dimensions);

    let game = Game::new(dimensions, stage_program, arrow_program);

    game_loop(
        game,
        240,
        0.1,
        |g| {
            g.game.update(g.last_frame_time());
        },
        move |g| {
            g.game.render(&ctx);
        },
    );

    Ok(())
}
