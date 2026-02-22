use amarok_syntax::*;

#[test]
fn can_construct_program() {
    let program = Program {
        statements: vec![
            Statement::Assignment {
                name: "x".to_string(),
                value: Expression::Binary {
                    left: Box::new(Expression::Integer(2).into()),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Integer(3).into()),
                }
                .into(),
            }
            .into(),
        ],
    };

    assert_eq!(program.statements.len(), 1);
}
