//! Module resolution and loading machinery.
//!
//! Resolves `use a::b::c;` to a file `<root>/a/b/c.amarok`, evaluates its
//! body in an isolated sub-state, and caches the resulting top-level
//! functions and variables for flat-merge into the importing module.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use amarok_syntax::SourceMap;

use crate::function::Function;
use crate::value::Value;

/// What a module contributes when flat-merged into an importer.
#[derive(Debug, Clone, Default)]
pub(crate) struct ModuleExports {
    pub(crate) functions: HashMap<String, Function>,
    pub(crate) variables: HashMap<String, Value>,
}

/// Module loader state owned by the [`Interpreter`].
///
/// `roots` are tried in order during resolution. `cache` deduplicates loads
/// keyed by canonical filesystem path. `loading` is the in-progress set used
/// for cycle detection.
#[derive(Debug, Default)]
pub(crate) struct ModuleLoader {
    pub(crate) roots: Vec<PathBuf>,
    pub(crate) cache: HashMap<PathBuf, ModuleExports>,
    pub(crate) loading: HashSet<PathBuf>,
    pub(crate) source_map: SourceMap,
}

impl ModuleLoader {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Resolve `a::b::c` to the first `<root>/a/b/c.amarok` that exists.
    ///
    /// Returns the canonicalized path on success, or a list of attempted
    /// paths on failure (the caller turns this into a [`Diagnostic`]).
    pub(crate) fn resolve(&self, segments: &[String]) -> Result<PathBuf, Vec<PathBuf>> {
        let mut tried: Vec<PathBuf> = Vec::new();

        let mut relative = PathBuf::new();
        for (index, segment) in segments.iter().enumerate() {
            if index + 1 == segments.len() {
                relative.push(format!("{segment}.amarok"));
            } else {
                relative.push(segment);
            }
        }

        for root in &self.roots {
            let candidate = root.join(&relative);
            if candidate.is_file() {
                return Ok(candidate.canonicalize().unwrap_or(candidate));
            }
            tried.push(candidate);
        }

        Err(tried)
    }

    pub(crate) fn add_root(&mut self, root: impl Into<PathBuf>) {
        self.roots.push(root.into());
    }

    pub(crate) fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    pub(crate) fn source_map_mut(&mut self) -> &mut SourceMap {
        &mut self.source_map
    }

    /// Format a list of attempted paths for diagnostic output.
    pub(crate) fn format_tried_paths(tried: &[PathBuf]) -> String {
        if tried.is_empty() {
            "(no module roots configured)".to_string()
        } else {
            tried
                .iter()
                .map(|p| format!("  - {}", display_path(p)))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
