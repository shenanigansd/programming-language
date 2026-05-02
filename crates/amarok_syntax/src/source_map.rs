use std::path::{Path, PathBuf};

/// Identifies a source file registered with a [`SourceMap`].
///
/// `FileId::DUMMY` is used for spans not tied to any file (e.g. synthetic
/// spans from `Span::zero()` and tests that don't go through a `SourceMap`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

impl FileId {
    pub const DUMMY: FileId = FileId(u32::MAX);

    #[must_use]
    pub fn is_dummy(self) -> bool {
        self == FileId::DUMMY
    }
}

/// A single source file registered with a [`SourceMap`].
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub source: String,
}

/// Registry of source files participating in a single compilation/run.
///
/// Diagnostics carry a `FileId` (via their `Span`); the renderer looks up the
/// file's path and source via this map.
#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    #[must_use]
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Register a file and return its [`FileId`].
    ///
    /// # Panics
    ///
    /// Panics if the number of registered files exceeds `u32::MAX`.
    pub fn add_file(&mut self, path: impl Into<PathBuf>, source: impl Into<String>) -> FileId {
        let id = FileId(u32::try_from(self.files.len()).expect("too many source files"));
        self.files.push(SourceFile {
            path: path.into(),
            source: source.into(),
        });
        id
    }

    #[must_use]
    pub fn get(&self, file_id: FileId) -> Option<&SourceFile> {
        if file_id.is_dummy() {
            return None;
        }
        self.files.get(file_id.0 as usize)
    }

    #[must_use]
    pub fn path_of(&self, file_id: FileId) -> Option<&Path> {
        self.get(file_id).map(|f| f.path.as_path())
    }

    #[must_use]
    pub fn source_of(&self, file_id: FileId) -> Option<&str> {
        self.get(file_id).map(|f| f.source.as_str())
    }
}
