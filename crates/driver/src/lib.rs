pub mod error;

use std::path::{Path, PathBuf};
use std::process::Command;

use codegen::compile_program_to_object;
use syntax::parse_source;

use crate::error::DriverError;

pub struct CompilationOptions {
    pub output_path: Option<PathBuf>,
}

impl CompilationOptions {
    pub fn simple() -> Self {
        CompilationOptions { output_path: None }
    }
}

pub fn compile_file<P: AsRef<Path>>(
    source_path: P,
    options: &CompilationOptions,
) -> Result<PathBuf, DriverError> {
    let source_path = source_path.as_ref();

    // 1. Read source file
    let source_text = std::fs::read_to_string(source_path)
        .map_err(|error| DriverError::new(format!("Failed to read file: {}", error)))?;

    // 2. Parse into AST
    let program = parse_source(&source_text);

    // 3. Codegen: AST -> object bytes
    let object_bytes = compile_program_to_object(&program)
        .map_err(|error| DriverError::new(format!("Code generation failed: {}", error)))?;

    // 4. Decide file paths
    let object_path = source_path.with_extension("o");
    let executable_path = options
        .output_path
        .clone()
        .unwrap_or_else(|| source_path.with_extension(""));

    // 5. Write object file
    std::fs::write(&object_path, &object_bytes)
        .map_err(|error| DriverError::new(format!("Failed to write object file: {}", error)))?;

    // 6. Link
    let status = Command::new("cc")
        .arg(&object_path)
        .arg("-o")
        .arg(&executable_path)
        .status()
        .map_err(|error| DriverError::new(format!("Failed to execute linker: {}", error)))?;

    if !status.success() {
        return Err(DriverError::new(format!(
            "Linker failed with status: {}",
            status
        )));
    }

    Ok(executable_path)
}
