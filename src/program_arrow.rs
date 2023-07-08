use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{domain::kinematic::Kinematic, vector::Vector};
use nalgebra::Vector2;

pub fn setup_arrow_program(
    ctx: &WebGl2RenderingContext,
    arrow_program: &WebGlProgram,
    dimensions: &Vector,
) -> () {
    let position = ctx.get_attrib_location(&arrow_program, "a_position");
    ctx.enable_vertex_attrib_array(position as u32);
    ctx.vertex_attrib_pointer_with_i32(
        position as u32,               // location
        2,                             // size (components per iteration)
        WebGl2RenderingContext::FLOAT, // type to get from buffer
        false,                         // normalized
        0,                             // stride (bytes to advance per iteration)
        0,                             // offset (bytes from start of buffer)
    );

    if let Some(resolution_location) = ctx.get_uniform_location(&arrow_program, "u_resolution") {
        ctx.uniform2f(
            Some(&resolution_location),
            dimensions.0 as f32,
            dimensions.1 as f32,
        );
    }
}

pub fn buffer_arrow_data(ctx: &WebGl2RenderingContext, kinematics: &Vec<Kinematic>) -> () {
    let pi = std::f32::consts::PI;
    let vs: Vec<f32> = kinematics
        .iter()
        .flat_map(|kinematic| {
            // Create a translation vector
            let translation = Vector2::new(kinematic.position.0, kinematic.position.1);

            // Create a rotation angle in radians, corrected so 0rad is north
            let rotation_angle = (2.0 * pi) - kinematic.orientation;

            // Create a scaling vector
            let scaling = Vector2::new(0.5, 0.5);

            // Define your shape's vertices as 2D vectors
            let vertices: [Vector2<f32>; 6] = [
                // north facing
                /*
                            Vector2::new(0.0, -25.0),
                            Vector2::new(0.0, 10.0),
                            Vector2::new(25.0, 25.0),
                            Vector2::new(0.0, -25.0),
                            Vector2::new(0.0, 10.0),
                            Vector2::new(-25.0, 25.0),
                */
                // east facing
                Vector2::new(25.0, 0.0),
                Vector2::new(-10.0, 0.0),
                Vector2::new(-25.0, -25.0),
                Vector2::new(25.0, 0.0),
                Vector2::new(-10.0, 0.0),
                Vector2::new(-25.0, 25.0),
            ];

            // Apply the transformations to your shape's vertices
            let vertices: Vec<f32> = vertices
                .iter()
                .flat_map(|v| {
                    let a = v.component_mul(&scaling);
                    let b = nalgebra::Rotation2::new(rotation_angle) * a;
                    let c = b + translation;
                    [c.x, c.y]
                })
                .collect();

            vertices
        })
        .collect();

    unsafe {
        let vertex_array = js_sys::Float32Array::view(&vs);
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}
