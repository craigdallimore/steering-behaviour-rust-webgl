pub fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d) -> &web_sys::CanvasRenderingContext2d {
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
}
