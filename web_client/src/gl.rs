use crate::util::*;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use wasm_bindgen::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub fn get_canvas(id: &str) -> Result<WebGl2RenderingContext, js_sys::Object> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
}

pub struct VertiForm {
    verties: Vec<f32>,
    size: u32,
    gl: WebGl2RenderingContext,
    program: Option<WebGlProgram>,
}
impl VertiForm {
    pub fn new(gl: WebGl2RenderingContext, data: Vec<f32>, size: u32) -> Self {
        VertiForm {
            verties: data.into(),
            size: size,
            gl: gl,
            program: None,
        }
    }
    fn compile_shader(
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
            log!(
                "error {}",
                context
                    .get_shader_info_log(&shader)
                    .unwrap_or("".to_string())
            );
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
    fn link_program(
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

    pub fn init_program<F>(&mut self, sourceinit: F) -> Result<(), JsValue>
    where
        F: Fn() -> (String, String),
    {
        let gl = &self.gl;
        let (vertex_source, frag_source) = sourceinit();

        //log!("vs: {:?}",vertex_source);
        //log!("fs: {:?}",frag_source);

        let vert_shader = Self::compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_source.as_str(),
        )?;
        let frag_shader = Self::compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            frag_source.as_str(),
        )?;
        let program = Self::link_program(gl, &vert_shader, &frag_shader)?;
        self.program = Some(program);
        gl.use_program(self.program.as_ref());
        Ok(())
    }
    pub fn init_vertex(&self) -> Result<(), JsValue> {
        let gl = &self.gl;

        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();

        let vertices_location = self.verties.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + self.verties.len() as u32,
        );

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        Ok(())
    }
    pub fn get_attrib(&self, name: &str) -> u32 {
        self.gl
            .get_attrib_location(self.program.as_ref().unwrap(), name) as u32
    }
    pub fn get_uniform(&self, name: &str) -> Option<WebGlUniformLocation> {
        self.gl
            .get_uniform_location(self.program.as_ref().unwrap(), name)
    }
    pub fn vertex_data_form(&self, attrib_name: &str) -> Result<(), JsValue> {
        let gl = &self.gl;

        let va = gl.create_vertex_array();
        gl.bind_vertex_array(va.as_ref()); // this is only for vertex

        let position_att_location = self.get_attrib(attrib_name);

        gl.enable_vertex_attrib_array(position_att_location);
        gl.vertex_attrib_pointer_with_i32(
            position_att_location,
            self.size as i32,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        Ok(())
    }
    pub fn init_buffer(&self, source: Vec<f32>) -> Result<(), JsValue> {
        let gl = &self.gl;

        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();

        let source_location = source.as_ptr() as u32 / 4;
        let source_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(source_location, source_location + source.len() as u32);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &source_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        Ok(())
    }
    pub fn attrib_data_form(&self, attrib_name: &str, size: i32) -> Result<(), JsValue> {
        let gl = &self.gl;

        let attrib_location = self.get_attrib(attrib_name);
        gl.enable_vertex_attrib_array(attrib_location);

        gl.vertex_attrib_pointer_with_i32(
            attrib_location,
            size,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        Ok(())
    }
    pub fn uniform_data_form<F>(&self, uniform_name: &str, custom: F) -> Result<(), JsValue>
    where
        F: Fn(&WebGl2RenderingContext, WebGlUniformLocation),
    {
        let gl = &self.gl;

        let uniform_location = self.get_uniform(uniform_name);
        match uniform_location {
            Some(pos) => {
                custom(gl, pos);
            }
            None => {}
        }
        Ok(())
    }
    pub fn draw(&self) -> Result<(), JsValue> {
        let gl = &self.gl;
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // Set the matrix.
        //gl.uniform_matrix2fv_with_f32_array(matrixlocation.as_ref(), false, &matrix);

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.verties.len() as i32 / self.size as i32,
        );
        Ok(())
    }
}
