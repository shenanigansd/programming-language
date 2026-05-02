//! Module resolution and loading machinery.
//!
//! Resolves `use a::b::c;` to a file `<root>/a/b/c.amarok`, evaluates its
//! body in an isolated sub-state, and caches the resulting top-level
//! functions and variables for flat-merge into the importing module.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use amarok_syntax::{Diagnostic, Program, SourceMap, Span};

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
    roots: Vec<PathBuf>,
    cache: HashMap<PathBuf, ModuleExports>,
    loading: HashSet<PathBuf>,
    source_map: SourceMap,
}

impl ModuleLoader {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Resolve `a::b::c` to a canonical path or return a span-attached
    /// `Module not found` diagnostic listing every path that was tried.
    pub(crate) fn resolve_or_diag(
        &self,
        segments: &[String],
        use_span: Span,
    ) -> Result<PathBuf, Diagnostic> {
        self.resolve(segments).map_err(|tried| {
            Diagnostic::new(format!(
                "Module not found: {}\nTried:\n{}",
                segments.join("::"),
                Self::format_tried_paths(&tried),
            ))
            .with_span(use_span)
        })
    }

    /// Read the file at `canonical`, register it with the source map, and
    /// parse it into a [`Program`]. All failure modes are wrapped in a
    /// span-attached [`Diagnostic`] for the caller to bubble up.
    pub(crate) fn read_and_parse(
        &mut self,
        canonical: &Path,
        use_span: Span,
    ) -> Result<Program, Diagnostic> {
        let source = std::fs::read_to_string(canonical).map_err(|error| {
            Diagnostic::new(format!(
                "Failed to read module {}: {}",
                canonical.display(),
                error
            ))
            .with_span(use_span)
        })?;

        let file_id = self
            .source_map
            .add_file(canonical.to_path_buf(), source.clone());

        amarok_parser::parse_program_with_file_id(&source, file_id).map_err(|diagnostic| {
            let span = diagnostic.span.unwrap_or(use_span);
            Diagnostic::new(format!(
                "Failed to parse module {}: {}",
                canonical.display(),
                diagnostic.message
            ))
            .with_span(span)
        })
    }

    pub(crate) fn cache_get(&self, path: &Path) -> Option<ModuleExports> {
        self.cache.get(path).cloned()
    }

    pub(crate) fn cache_insert(&mut self, path: PathBuf, exports: ModuleExports) {
        self.cache.insert(path, exports);
    }

    pub(crate) fn is_loading(&self, path: &Path) -> bool {
        self.loading.contains(path)
    }

    pub(crate) fn begin_loading(&mut self, path: PathBuf) {
        self.loading.insert(path);
    }

    pub(crate) fn end_loading(&mut self, path: &Path) {
        self.loading.remove(path);
    }

    fn resolve(&self, segments: &[String]) -> Result<PathBuf, Vec<PathBuf>> {
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
    fn format_tried_paths(tried: &[PathBuf]) -> String {
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
