use cranelift::codegen::ir::{self, InstBuilder};
use cranelift::codegen::settings;
use cranelift::codegen::settings::Configurable;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift::module::{Linkage, Module};
use cranelift::object::{ObjectBuilder, ObjectModule};
use syntax::ast::{BinaryOperator, ExpressionNode, ProgramNode, StatementNode};

use crate::error::CodegenError;

use std::collections::HashMap;

struct CodegenContext<'a, 'f> {
    function_builder: &'a mut FunctionBuilder<'f>,
    variables: HashMap<String, cranelift::codegen::ir::StackSlot>,
}

pub fn compile_program(program: &ProgramNode) -> Result<Vec<u8>, CodegenError> {
    // 1. Build flags and host instruction set
    let mut flag_builder = settings::builder();
    flag_builder
        .set("is_pic", "true")
        .map_err(|error| CodegenError::new(format!("Failed to set Cranelift flag: {}", error)))?;
    let flags = settings::Flags::new(flag_builder);

    let instruction_set_builder = cranelift::native::builder()
        .map_err(|error| CodegenError::new(format!("Host not supported: {}", error)))?;

    let instruction_set = instruction_set_builder.finish(flags).map_err(|error| {
        CodegenError::new(format!("Failed to build instruction set: {}", error))
    })?;

    // 2. Create an object module (object file in memory)
    let object_builder = ObjectBuilder::new(
        instruction_set,
        "wolf",
        cranelift::module::default_libcall_names(),
    )
    .map_err(|error| CodegenError::new(format!("Failed to build object module: {}", error)))?;

    let mut module = ObjectModule::new(object_builder);

    // 3. Create a function context for `main`
    let mut context = module.make_context();
    context
        .func
        .signature
        .returns
        .push(cranelift::codegen::ir::AbiParam::new(
            cranelift::codegen::ir::types::I64,
        ));

    let mut function_builder_context = FunctionBuilderContext::new();

    {
        use cranelift::codegen::ir::InstBuilder;

        let mut function_builder =
            FunctionBuilder::new(&mut context.func, &mut function_builder_context);

        let entry_block = function_builder.create_block();
        function_builder.switch_to_block(entry_block);
        function_builder.seal_block(entry_block);

        // For now: compile only the last expression statement, return its value.
        let return_value = compile_program_statements(program, &mut function_builder)?;

        function_builder.ins().return_(&[return_value]);
        function_builder.finalize();
    }

    // 4. Declare and define the function in the module
    let function_id = module
        .declare_function("main", Linkage::Export, &context.func.signature)
        .map_err(|error| CodegenError::new(format!("Failed to declare function: {}", error)))?;

    module
        .define_function(function_id, &mut context)
        .map_err(|error| CodegenError::new(format!("Failed to define function: {}", error)))?;

    module.clear_context(&mut context);

    // 5. Finish the module and emit the object file bytes
    let product = module.finish();

    let object_bytes = product
        .emit()
        .map_err(|error| CodegenError::new(format!("Failed to emit object: {}", error)))?;

    Ok(object_bytes)
}

fn compile_program_statements<'a, 'f>(
    program: &ProgramNode,
    function_builder: &'a mut FunctionBuilder<'f>,
) -> Result<ir::Value, CodegenError> {
    let mut context = CodegenContext {
        function_builder,
        variables: HashMap::new(),
    };

    let mut last_value = None;

    for statement in &program.statements {
        last_value = Some(compile_statement(statement, &mut context)?);
    }

    last_value.ok_or_else(|| CodegenError::new("Program had no statements"))
}

fn compile_statement<'a, 'f>(
    statement: &StatementNode,
    context: &mut CodegenContext<'a, 'f>,
) -> Result<ir::Value, CodegenError> {
    match statement {
        StatementNode::ExpressionStatement { expression } => {
            compile_expression(expression, context)
        }

        StatementNode::VariableDeclaration { name, value } => {
            let value_ir = compile_expression(value, context)?;

            use cranelift::codegen::ir::{StackSlotData, StackSlotKind};

            let slot = context
                .function_builder
                .create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 0));

            context
                .function_builder
                .ins()
                .stack_store(value_ir, slot, 0);
            context.variables.insert(name.clone(), slot);

            // the value of a declaration is the initializer value
            Ok(value_ir)
        }
    }
}

fn compile_expression<'a, 'f>(
    expression: &ExpressionNode,
    context: &mut CodegenContext<'a, 'f>,
) -> Result<ir::Value, CodegenError> {
    match expression {
        ExpressionNode::NumberLiteral { value } => {
            let immediate = *value;
            Ok(context
                .function_builder
                .ins()
                .iconst(ir::types::I64, immediate))
        }

        ExpressionNode::IdentifierReference { name } => {
            let slot = context
                .variables
                .get(name)
                .ok_or_else(|| CodegenError::new(format!("Undefined variable: {}", name)))?;

            Ok(context
                .function_builder
                .ins()
                .stack_load(ir::types::I64, *slot, 0))
        }

        ExpressionNode::BinaryOperation {
            operator,
            left,
            right,
        } => {
            let left_value = compile_expression(left, context)?;
            let right_value = compile_expression(right, context)?;

            let result = match operator {
                BinaryOperator::Add => context.function_builder.ins().iadd(left_value, right_value),
                BinaryOperator::Subtract => {
                    context.function_builder.ins().isub(left_value, right_value)
                }
                BinaryOperator::Multiply => {
                    context.function_builder.ins().imul(left_value, right_value)
                }
                BinaryOperator::Divide => {
                    context.function_builder.ins().sdiv(left_value, right_value)
                }
            };

            Ok(result)
        }
    }
}
