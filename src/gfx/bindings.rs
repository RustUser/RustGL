use std::ffi::CString;
use std::mem::size_of;
use gl::*;
use std::ptr::{null_mut};
use crate::gfx::shader::Shader;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum BufferDataType {
    Float = gl::FLOAT
}

impl BufferDataType {
    pub fn size(&self) -> usize {
        match self {
            BufferDataType::Float => size_of::<f32>()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawType {
    StreamDraw = gl::STREAM_DRAW,
    StaticDraw = gl::STATIC_DRAW,
    DynamicDraw = gl::DYNAMIC_DRAW,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum BufferType {
    ArrayBuffer = gl::ARRAY_BUFFER
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ColorFlags {
    ColorBufferBit = GLConsts::ColorBufferBit as u32,
    DepthBufferBit = GLConsts::DepthBufferBit as u32,
}

impl std::ops::BitOr for ColorFlags {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum IV {
    CompileStatus = gl::COMPILE_STATUS,

    LinkStatus = gl::LINK_STATUS,

    InfoLogLength = gl::INFO_LOG_LENGTH,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawMode {
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum GLConsts {
    DepthTest = gl::DEPTH_TEST,
    ColorBufferBit = gl::COLOR_BUFFER_BIT,
    DepthBufferBit = gl::DEPTH_BUFFER_BIT,

    Always = gl::ALWAYS,
    Never = gl::NEVER,
    Less = gl::LESS,
    Equal = gl::EQUAL,
    LEqual = gl::LEQUAL,
    Greater = gl::GREATER,
    NotEqual = gl::NOTEQUAL,
    GEqual = gl::GEQUAL,

    Line = gl::LINE,
    Fill = gl::FILL,

    Back = gl::BACK,
    Front = gl::FRONT,
    FrontAndBack = gl::FRONT_AND_BACK,

    CullFace = gl::CULL_FACE,

    CounterClockwise = gl::CCW,
    Clockwise = gl::CW,
    Blend = gl::BLEND,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PolygonMode {
    Line = GLConsts::Line as u32,
    Fill = GLConsts::Fill as u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Face {
    Back = GLConsts::Back as u32,
    Front = GLConsts::Front as u32,
    FrontAndBack = GLConsts::FrontAndBack as u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Ordering {
    CounterClockWise = GLConsts::CounterClockwise as u32,
    ClockWise = GLConsts::Clockwise as u32,
}

pub mod program {
    use std::ptr::null_mut;
    use crate::gfx::bindings::{create_whitespace_cstring_with_len, IV};
    use crate::gfx::program::Program;
    use crate::gfx::shader::Shader;

    pub fn use_program(program: &Program) {
        unsafe {
            gl::UseProgram(program.id());
        }
    }

    pub fn disable_program() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_program() -> u32 {
        unsafe {
            gl::CreateProgram()
        }
    }

    pub fn attach_shader<S: Shader>(program: &Program, shader: &S) {
        unsafe {
            gl::AttachShader(program.id(), shader.id());
        }
    }

    pub fn link_program(program: &Program) {
        unsafe {
            gl::LinkProgram(program.id());
        }
    }

    pub fn get_program_iv(program: &Program, iv: IV) -> i32 {
        unsafe {
            let mut success: i32 = 0;
            gl::GetProgramiv(program.id(), iv as u32, &mut success);
            success
        }
    }

    pub fn program_iv(program: &Program, iv: IV) -> bool {
        get_program_iv(program, iv) == 1
    }

    pub fn program_log_len(program: &Program) -> i32 {
        get_program_iv(program, IV::InfoLogLength)
    }

    pub fn program_info_log(program: &Program) -> String {
        unsafe {
            let log_size = program_log_len(program);
            let buffer = create_whitespace_cstring_with_len(log_size as usize);

            gl::GetProgramInfoLog(
                program.id(),
                log_size,
                null_mut(),
                buffer.as_ptr() as *mut gl::types::GLchar,
            );
            buffer.to_string_lossy().to_string()
        }
    }
}

pub mod shader {
    use std::ffi::CString;
    use std::ptr::null;
    use super::*;

    pub fn gl_create_shader(shader_type: ShaderType) -> u32 {
        unsafe {
            CreateShader(shader_type as u32)
        }
    }

    pub fn shader_source<T: ToString>(source: T, shader: &dyn Shader) {
        unsafe {
            let source = source.to_string();
            let source = &CString::new(source).unwrap();

            ShaderSource(shader.id(), 1, &source.as_ptr(), null());
        }
    }

    pub fn compile_shader(shader: &dyn Shader) {
        unsafe {
            CompileShader(shader.id());
        }
    }

    pub fn get_shader_iv<S: Shader + ?Sized>(shader: &S, iv: IV) -> i32 {
        unsafe {
            let mut success: i32 = 0;
            GetShaderiv(shader.id(), iv as u32, &mut success);
            success
        }
    }

    pub fn info_log_len<S: Shader + ?Sized>(shader: &S) -> i32 {
        get_shader_iv(shader, IV::InfoLogLength)
    }

    pub fn shader_info_log<S: Shader + ?Sized>(shader: &S) -> String {
        unsafe {
            let log_size = info_log_len(shader);
            let buffer = create_whitespace_cstring_with_len(log_size as usize);

            gl::GetShaderInfoLog(
                shader.id(),
                log_size,
                null_mut(),
                buffer.as_ptr() as *mut gl::types::GLchar,
            );
            buffer.to_string_lossy().to_string()
        }
    }

    pub fn delete_shader<S: Shader>(shader: &S) {
        unsafe {
            gl::DeleteShader(shader.id());
        }
    }
}

pub mod buffers {
    use std::ffi::c_void;
    use crate::DrawType;
    use crate::gfx::bindings::BufferType;
    use crate::gfx::objects::Buffer;
    use crate::gfx::objects::vertex_array_object::VertexArrayObject;

    pub fn gen_buffers(count: i32) -> u32 {
        unsafe {
            let mut id: u32 = 0;
            gl::GenBuffers(count, &mut id);
            id
        }
    }

    pub fn bind_buffer<B: Buffer + ?Sized>(buffer_type: BufferType, buffer: &B) {
        unsafe {
            gl::BindBuffer(buffer_type as u32, buffer.id());
        }
    }

    pub fn buffer_data_array(buffer_type: BufferType, size: isize, data: *const c_void, draw_type: DrawType) {
        unsafe {
            gl::BufferData(buffer_type as u32, size, data, draw_type as u32);
        }
        println!("glBufferData({:?}, {}, {:?}, {:?})", buffer_type, size, data, draw_type);
    }

    pub fn gen_vertex_arrays(size: i32) -> u32 {
        unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(size, &mut vao);
            vao
        }
    }

    pub fn bind_vertex_array(vao: &VertexArrayObject) {
        unsafe {
            gl::BindVertexArray(vao.id());
        }
    }

    pub fn unbind_vertex_array() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub mod graphics {
    use crate::gfx::bindings::{DrawMode, Face, GLConsts, Ordering, PolygonMode};
    use crate::Program;

    pub fn clear(flags: u32) {
        unsafe {
            gl::Clear(flags)
        }
    }

    pub fn clear_color(color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
        }
    }

    pub fn draw_arrays(mode: DrawMode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode as u32, first, count);
        }
    }

    pub fn viewport(position: [i32; 2], size: [i32; 2]) {
        unsafe {
            gl::Viewport(position[0], position[1], size[0], size[1])
        }
    }

    pub fn enable(constant: GLConsts) {
        unsafe {
            gl::Enable(constant as u32);
        }
    }

    pub fn cull_face(face: Face) {
        unsafe {
            gl::CullFace(face as u32);
        }
    }

    pub fn front_face(ordering: Ordering) {
        unsafe {
            gl::FrontFace(ordering as u32);
        }
    }

    pub fn disable(constant: GLConsts) {
        unsafe {
            gl::Disable(constant as u32);
        }
    }

    pub fn depth_mask(depth_mask: bool) {
        unsafe {
            let mask = match depth_mask {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::DepthMask(mask)
        }
    }

    pub fn depth_func(constant: GLConsts) {
        unsafe {
            gl::DepthFunc(constant as u32);
        }
    }

    pub fn polygon_mode(face: Face, mode: PolygonMode) {
        unsafe {
            gl::PolygonMode(face as u32, mode as u32);
        }
    }

    pub fn shaded_wireframe<F>(face: Face, wireframe_color: &[f32; 4], draw: F) where F: Fn() {
        polygon_mode(face, PolygonMode::Fill);
        Program::current_program(|program| program.set_uniform_vec4("color", &[1.0; 4]));
        draw();

        polygon_mode(face, PolygonMode::Line);
        Program::current_program(|program| program.set_uniform_vec4("color", wireframe_color));
        draw();

        polygon_mode(face, PolygonMode::Fill);
    }
}

pub mod attrib_pointer {
    use std::ffi::c_void;
    use std::mem::transmute;
    use crate::BufferDataType;
    use crate::gfx::objects::vertex_attrib_pointer::VertexAttribPointer;

    pub fn vertex_attrib_pointer(id: u32, size: i32, data_type: BufferDataType, normalized: bool, stride: i32, offset: *const c_void) {
        unsafe {
            let normalized = match normalized {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::VertexAttribPointer(id, size, data_type as u32, normalized, stride, offset);
            let offset: isize = transmute(offset);
            println!("VertexAttribPointer({}, {}, {:?}, {}, {}, {})", id, size, data_type, normalized, stride, offset);
        }
    }

    pub fn enable_vertex_attrib_array(attrib_pointer: &VertexAttribPointer) {
        unsafe {
            gl::EnableVertexAttribArray(attrib_pointer.id());
        }
    }

    pub fn disable_vertex_attrib_array(attrib_pointer: &VertexAttribPointer) {
        unsafe {
            gl::DisableVertexAttribArray(attrib_pointer.id());
        }
    }
}

pub mod uniforms {
    use std::ffi::CString;
    use vecmath::{Matrix4, Vector2, Vector4};
    use crate::Program;

    pub fn uniform_location<T: ToString>(program: &Program, name: T) -> i32 {
        unsafe {
            let name = name.to_string();
            let name = CString::new(name).unwrap();

            gl::GetUniformLocation(program.id(), name.as_ptr())
        }
    }

    pub fn uniform_1ui(location: i32, value: &u32) {
        unsafe {
            gl::Uniform1ui(location, *value);
        }
    }

    pub fn uniform_1i(location: i32, value: &i32) {
        unsafe {
            gl::Uniform1i(location, *value);
        }
    }

    pub fn uniform_1f(location: i32, value: &f32) {
        unsafe {
            gl::Uniform1f(location, *value);
        }
    }

    pub fn uniform_1fv(location: i32, count: i32, value: &f32) {
        unsafe {
            gl::Uniform1fv(location, count, value as *const f32);
        }
    }

    pub fn uniform_2fv(location: i32, count: i32, value: &Vector2<f32>) {
        unsafe {
            gl::Uniform2fv(location, count, value.as_ptr());
        }
    }

    pub fn uniform_4fv(location: i32, count: i32, value: &Vector4<f32>) {
        unsafe {
            gl::Uniform4fv(location, count, value.as_ptr());
        }
    }

    pub fn uniform_matrix4fv(location: i32, count: i32, transpose: bool, value: &Matrix4<f32>) {
        unsafe {
            let transpose = match transpose {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::UniformMatrix4fv(location, count, transpose, value[0].as_ptr())
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}