use crate::gl::get_canvas;
use crate::gl::VertiForm;
use js_sys::Math;
use wasm_bindgen::prelude::*;

pub fn start() -> Result<(), JsValue> {
    let context = get_canvas("inner1").unwrap();

    let positions: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
    ];
    let rgb1 = (
        Math::random() as f32,
        Math::random() as f32,
        Math::random() as f32,
        1.0,
    );
    let rgb2 = (
        Math::random() as f32,
        Math::random() as f32,
        Math::random() as f32,
        1.0,
    );

    let colors: [f32; 24] = [
        rgb1.0, rgb1.1, rgb1.2, rgb1.3, rgb1.0, rgb1.1, rgb1.2, rgb1.3, rgb1.0, rgb1.1, rgb1.2,
        rgb1.3, rgb2.0, rgb2.1, rgb2.2, rgb2.3, rgb2.0, rgb2.1, rgb2.2, rgb2.3, rgb2.0, rgb2.1,
        rgb2.2, rgb2.3,
    ];

    let mut sample_vert = VertiForm::new(context, positions.to_vec(), 2);

    sample_vert.init_program(|| {
        (
            include_str!("../gl/gomoku/vertex.glsl").to_string(),
            include_str!("../gl/gomoku/frag.glsl").to_string(),
        )
    })?;

    sample_vert.init_vertex()?;
    sample_vert.vertex_data_form("a_position")?;

    sample_vert.init_buffer(colors.to_vec())?;
    sample_vert.attrib_data_form("a_color", 4)?;

    sample_vert.draw()?;

    Ok(())
}
