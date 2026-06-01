use amarok_interpreter::{Environment, Value};

#[test]
fn a_child_sees_a_variable_from_its_parent() {
    let global = Environment::new_global();
    global.borrow_mut().define("x", Value::Number(5.0));
    let child = Environment::new_child(&global);
    assert_eq!(child.borrow().get("x"), Some(Value::Number(5.0)));
}

#[test]
fn a_child_can_shadow_a_parent_variable() {
    let global = Environment::new_global();
    global.borrow_mut().define("x", Value::Number(5.0));
    let child = Environment::new_child(&global);
    child.borrow_mut().define("x", Value::Number(99.0));
    assert_eq!(child.borrow().get("x"), Some(Value::Number(99.0))); // child's own
    assert_eq!(global.borrow().get("x"), Some(Value::Number(5.0))); // parent untouched
}

#[test]
fn a_missing_name_is_none_after_walking_all_parents() {
    let global = Environment::new_global();
    let child = Environment::new_child(&global);
    assert_eq!(child.borrow().get("absent"), None);
}
