use amarok_syntax::{Diagnostic, Span};

use crate::control_flow::ControlFlow;
use crate::eval::statements::execute_statement_list;
use crate::interpreter::Interpreter;
use crate::module_loader::ModuleExports;
use crate::scope::ScopeStack;

pub(crate) fn load_and_merge(
    interp: &mut Interpreter,
    segments: &[String],
    use_span: Span,
) -> Result<(), Diagnostic> {
    let canonical = interp.loader.resolve_or_diag(segments, use_span)?;

    if let Some(exports) = interp.loader.cache_get(&canonical) {
        merge_exports(interp, exports);
        return Ok(());
    }

    if interp.loader.is_loading(&canonical) {
        return Err(Diagnostic::new(format!(
            "Circular import detected while loading {}",
            canonical.display()
        ))
        .with_span(use_span));
    }

    let program = interp.loader.read_and_parse(&canonical, use_span)?;

    interp.loader.begin_loading(canonical.clone());
    let saved_scopes = std::mem::replace(&mut interp.scopes, ScopeStack::new());
    let saved_functions = std::mem::take(&mut interp.functions);

    let evaluation = execute_statement_list(interp, &program.statements);

    let exports = ModuleExports {
        functions: std::mem::take(&mut interp.functions),
        variables: interp.scopes.outermost_clone(),
    };

    interp.scopes = saved_scopes;
    interp.functions = saved_functions;
    interp.loader.end_loading(&canonical);

    match evaluation {
        Ok(ControlFlow::Continue) => {}
        Ok(ControlFlow::Return(_)) => {
            return Err(Diagnostic::new(format!(
                "Return outside of function in module {}",
                canonical.display()
            ))
            .with_span(use_span));
        }
        Err(diagnostic) => return Err(diagnostic),
    }

    interp.loader.cache_insert(canonical, exports.clone());
    merge_exports(interp, exports);
    Ok(())
}

fn merge_exports(interp: &mut Interpreter, exports: ModuleExports) {
    for (name, function) in exports.functions {
        interp.functions.insert(name, function);
    }
    for (name, value) in exports.variables {
        interp.scopes.set_outermost(&name, value);
    }
}
