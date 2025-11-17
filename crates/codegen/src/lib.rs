pub mod error;
mod lower;

use syntax::ast::ProgramNode;

use crate::error::CodegenError;

/// Compile a program AST into an object file in memory.
///
/// The returned bytes are a complete object file that the system linker can consume.
pub fn compile_program_to_object(program: &ProgramNode) -> Result<Vec<u8>, CodegenError> {
    lower::compile_program(program)
}
