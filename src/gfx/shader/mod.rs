use crate::gfx::bindings::shader::{get_shader_iv, shader_info_log};
use crate::gfx::bindings::{IV, ShaderType};
use crate::gfx::shader::error::Error;

pub mod fragment_shader;
pub mod vertex_shader;
pub mod error;

pub type ShaderResult = Result<(), Error>;

pub trait Shader {
    fn id(&self) -> u32;
    fn compilation_status(&self) -> ShaderResult {
        let success = get_shader_iv(self, IV::CompileStatus) == 1;
        if !success {
            return Err(Error::CompilationError(shader_info_log(self)));
        }
        Ok(())
    }
    fn shader_type(&self) -> ShaderType;
}