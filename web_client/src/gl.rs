use wasm_bindgen::prelude::*;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram,WebGl2RenderingContext, WebGlShader};
use web_sys::console;


#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("inner1").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es
        
        in vec4 a_position;
        
        void main() {
            gl_Position = a_position;
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        
        precision mediump float;
        
        out vec4 outColor;
        void main() {
            outColor = vec4(1, 0, 1.5, 1);
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let position_att_location = context.get_attrib_location(&program,"a_position") as u32;
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    
    let positions: [f32; 6] = [
        0.0,0.0,
        0.0,0.5,
        0.7,0.0,
    ];
    
    let memory_buffer = wasm_bindgen::memory()
    .dyn_into::<WebAssembly::Memory>()?
    .buffer();
    
    let vertices_location = positions.as_ptr() as u32 / 4;    
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + positions.len() as u32);

    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );
    
    //log!("va: {}",position_att_location);
    let va=context.create_vertex_array();
    context.bind_vertex_array(va.as_ref());
    context.enable_vertex_attrib_array(position_att_location);
    
    context.vertex_attrib_pointer_with_i32(
        position_att_location,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0
    );
    
    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        3
    );
    
    Ok(())
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}