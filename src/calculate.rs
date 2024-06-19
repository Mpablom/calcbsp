fn calculate(expression: &str) -> Result<String, Box<dyn Error>> {
    let result = evaluate_expression(expression);
    if result == "Error" {
        Err(Box::new(CustomError("Error evaluating expression".to_string())))
    } else {
        Ok(result)
    }
}
