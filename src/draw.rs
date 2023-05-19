use crate::State;
//use crate::kinematic::Kinematic;

pub fn draw_grid(
  ctx: &web_sys::WebGl2RenderingContext,
  state: &State
) -> () {

  ctx.viewport(
    0,
    0,
    state.dimensions.0 as i32,
    state.dimensions.1 as i32
  );


    /*
    ctx.save();
    ctx.set_stroke_style(&"rgb(227, 242, 253)".into());

    for x in (10..800).step_by(10) {
        ctx.begin_path();
        ctx.move_to(x as f64, 0.0);
        ctx.line_to(x as f64, 800.0);
        ctx.stroke();
    }
    for z in (10..800).step_by(10) {
        ctx.begin_path();
        ctx.move_to(0.0, z as f64);
        ctx.line_to(800.0, z as f64);
        ctx.stroke();
    }

    ctx.restore();
    ctx
    */
}

/*
pub fn draw_arrow<'a>(
    ctx: &'a web_sys::CanvasRenderingContext2d,
    c: &Kinematic,
) -> &'a web_sys::CanvasRenderingContext2d {
    ctx.save();

    ctx.transform(1.0, 0.0, 0.0, 1.0, c.position.0, c.position.1)
        .unwrap();
    ctx.set_fill_style(&"rgb(240, 98, 146)".into());
    ctx.rotate(c.orientation).unwrap();
    ctx.move_to(0.0, -10.0);
    ctx.begin_path();
    ctx.line_to(5.0, 10.0);
    ctx.line_to(0.0, 5.0);
    ctx.line_to(-5.0, 10.0);
    ctx.line_to(0.0, -10.0);
    ctx.fill();

    ctx.restore();
    ctx
}
*/
