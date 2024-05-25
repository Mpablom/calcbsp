use evalexpr::{eval_with_context, Context, Function, HashMapContext, Value};

pub fn evaluate_expression(expression: &str) -> String {
    let expression = expression
        .replace("x", "*")
        .replace("ln", "lnf")
        .replace("log", "log10f")
        .replace(" ^ ", "pow");

    let mut context = HashMapContext::new();
    context
        .set_function(
            "lnf".into(),
            Function::new(
                None,
                Box::new(|argument| {
                    let value = argument[0].as_number()?;
                    Ok(Value::from(value.ln()))
                }),
            ),
        )
        .unwrap();
    context
        .set_function(
            "log10f".into(),
            Function::new(
                None,
                Box::new(|argument| {
                    let value = argument[0].as_number()?;
                    Ok(Value::from(value.log10()))
                }),
            ),
        )
        .unwrap();
    context
        .set_function(
            "pow".into(),
            Function::new(
                Some(2),
                Box::new(|arguments| {
                    let base = arguments[0].as_number()?;
                    let exponent = arguments[1].as_number()?;
                    Ok(Value::from(base.powf(exponent)))
                }),
            ),
        )
        .unwrap();

    eval_with_context(&expression, &context)
        .map_or_else(|_| "Error".to_string(), |result| result.to_string())
}
