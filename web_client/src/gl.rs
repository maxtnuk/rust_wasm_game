use wasm_bindgen::prelude::*;
use js_sys::{WebAssembly,Math};
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram,WebGl2RenderingContext, WebGlShader};
use web_sys::console;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}
struct VertiForm{
    verties: Vec<f32>,
    size: u32,
    gl: WebGl2RenderingContext,
    program: Option<WebGlProgram>
}
impl VertiForm{
    fn new(gl: WebGl2RenderingContext,data: Vec<f32>,size: u32) -> Self
    {
        VertiForm{
            verties: data.into(),
            size: size,
            gl: gl,
            program: None
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
            log!("error {}",context.get_shader_info_log(&shader).unwrap_or("".to_string()));
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

    fn init_program(&mut self)->Result<(), JsValue>{
        let gl = &self.gl;
        let vertex_source=include_str!("gl/vertex.glsl");
        let frag_source=include_str!("gl/frag.glsl");
        
        //log!("vs: {:?}",vertex_source);
        //log!("fs: {:?}",frag_source);
        
        let vert_shader = Self::compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_source,
        )?;
        let frag_shader = Self::compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            frag_source,
        )?;
        let program = Self::link_program(gl, &vert_shader, &frag_shader)?;
        self.program=Some(program);
        gl.use_program(self.program.as_ref());
        Ok(())
    }
    fn init_vertex(&self)->Result<(), JsValue>{
        let gl = &self.gl;
        
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
        let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
        
        let vertices_location = self.verties.as_ptr() as u32 / 4;    
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + self.verties.len() as u32);
    
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        Ok(())
    }
    fn get_attrib(&self,name: &str) -> u32{
        self.gl.get_attrib_location(self.program.as_ref().unwrap(),name) as u32
    }
    fn vertex_data_form(&self)->Result<(), JsValue>{
        let gl = &self.gl;
        
        let position_att_location = self.get_attrib("a_position");
        let va=gl.create_vertex_array();
        
        gl.bind_vertex_array(va.as_ref());// this is only for vertex
        
        gl.enable_vertex_attrib_array(position_att_location);
        gl.vertex_attrib_pointer_with_i32(
            position_att_location,
            self.size as i32,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0
        );
        Ok(())
    }
    fn init_color(&self,color: Vec<f32>)->Result<(), JsValue>{
        let gl = &self.gl;
        
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
        let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
        
        let color_location = color.as_ptr() as u32 / 4;    
        let color_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(color_location, color_location + color.len() as u32);
    
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        Ok(())
    }
    fn color_data_form(&self)->Result<(), JsValue>{
        let gl = &self.gl;
        
        let color_att_location = self.get_attrib("a_color");
        gl.enable_vertex_attrib_array(color_att_location);
        
        gl.vertex_attrib_pointer_with_i32(
            color_att_location,
            4,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0
        );
        Ok(())
    }
    fn draw(&self)->Result<(), JsValue>{
        let gl = &self.gl; 
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    
        let matrixlocation = self.gl.get_uniform_location(self.program.as_ref().unwrap(),"u_matrix");
        let matrix= [0.0,0.0,0.0,0.0];
        // Set the matrix.
        //gl.uniform_matrix2fv_with_f32_array(matrixlocation.as_ref(), false, &matrix);
    
        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.verties.len() as i32 / self.size as i32
        );
        Ok(())
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
        
    let positions: [f32;12] = [
        -1.0,-1.0,
        1.0,-1.0,
        -1.0,1.0,
        1.0,-1.0,
        -1.0,1.0,
        1.0,1.0
    ];
    let rgb1 = (Math::random() as f32,Math::random() as f32,Math::random() as f32,1.0);
    let rgb2 = (Math::random() as f32,Math::random() as f32,Math::random() as f32,1.0);
    
    let colors: [f32;24] = [
        rgb1.0,rgb1.1,rgb1.2,rgb1.3,
        rgb1.0,rgb1.1,rgb1.2,rgb1.3,
        rgb1.0,rgb1.1,rgb1.2,rgb1.3,
        rgb2.0,rgb2.1,rgb2.2,rgb2.3,
        rgb2.0,rgb2.1,rgb2.2,rgb2.3,
        rgb2.0,rgb2.1,rgb2.2,rgb2.3
    ];
    
    let mut sample_vert=VertiForm::new(context,positions.to_vec(),2);
    
    sample_vert.init_program()?;
    
    sample_vert.init_vertex()?;
    sample_vert.vertex_data_form()?;
    
    sample_vert.init_color(colors.to_vec())?;
    sample_vert.color_data_form()?;
    
    sample_vert.draw()?;
    
    Ok(())
}