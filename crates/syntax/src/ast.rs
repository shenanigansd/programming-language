#[derive(Debug, Clone)]
pub enum ExpressionNode {
    NumberLiteral {
        value: i64,
    },
    IdentifierReference {
        name: String,
    },
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    ExpressionStatement { expression: ExpressionNode },
    VariableDeclaration { name: String, value: ExpressionNode },
}

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

pub trait AstDisplay {
    fn write_ast(&self, indent: usize, output: &mut String);

    fn indent(indent: usize) -> String {
        "  ".repeat(indent)
    }

    fn to_ast_string(&self) -> String {
        let mut output = String::new();
        self.write_ast(0, &mut output);
        output
    }
}

#[cfg(feature = "debug-ast")]
impl AstDisplay for ProgramNode {
    fn write_ast(&self, indent: usize, output: &mut String) {
        output.push_str(&format!("{}Program\n", Self::indent(indent)));

        for statement in &self.statements {
            statement.write_ast(indent + 1, output);
        }
    }
}

#[cfg(feature = "debug-ast")]
impl AstDisplay for StatementNode {
    fn write_ast(&self, indent: usize, output: &mut String) {
        match self {
            StatementNode::ExpressionStatement { expression } => {
                output.push_str(&format!("{}ExpressionStatement\n", Self::indent(indent)));
                expression.write_ast(indent + 1, output);
            }

            StatementNode::VariableDeclaration { name, value } => {
                output.push_str(&format!(
                    "{}VariableDeclaration(name={})\n",
                    Self::indent(indent),
                    name
                ));
                value.write_ast(indent + 1, output);
            }
        }
    }
}

#[cfg(feature = "debug-ast")]
impl AstDisplay for ExpressionNode {
    fn write_ast(&self, indent: usize, output: &mut String) {
        match self {
            ExpressionNode::NumberLiteral { value } => {
                output.push_str(&format!(
                    "{}NumberLiteral({})\n",
                    Self::indent(indent),
                    value
                ));
            }

            ExpressionNode::IdentifierReference { name } => {
                output.push_str(&format!(
                    "{}IdentifierReference({})\n",
                    Self::indent(indent),
                    name
                ));
            }

            ExpressionNode::BinaryOperation {
                operator,
                left,
                right,
            } => {
                output.push_str(&format!(
                    "{}BinaryOperation({:?})\n",
                    Self::indent(indent),
                    operator
                ));

                left.write_ast(indent + 1, output);
                right.write_ast(indent + 1, output);
            }
        }
    }
}
