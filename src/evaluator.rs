use evalexpr::{eval_with_context, Context, Function, HashMapContext, Value};
use evalexpr::{eval_with_context, Context, Function, HashMapContext, Value};
use bigdecimal::BigDecimal;


const PI: f64 = 3.14159265358979323846264338327950288;
const E: f64 = 2.71828182845904523536028747135266250;


pub fn evaluate_expression(expression: &str) -> String {
    let expression = expression
        .replace("x", "*")
        .replace("𝑙𝑛", "lnf")
        .replace("𝑙𝑜𝑔", "log10f")
        .replace(" ^ ", "pow")
        .replace("π", &PI.to_string())
        .replace("𝑒", &E.to_string());
 
        let expression = replace_percentage(&expression);

    let mut context = HashMapContext::new();
    context.set_value("π".to_string(), Value::from(PI)).unwrap();
    context.set_value("𝑒".to_string(), Value::from(E)).unwrap();
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

    let result = eval_with_context(&expression, &context);
    
    match result {
        Ok(value) => {
            // Convertimos el valor a BigDecimal
            let decimal_value = BigDecimal::from_f64(value.as_float().unwrap()).unwrap();

            // Redondeamos el resultado a 20 decimales
            let rounded_result = decimal_value.round_dp_with_strategy(20, bigdecimal::RoundingStrategy::RoundHalfUp);

            // Convertimos el resultado a una cadena
            rounded_result.to_string()
        }
        Err(_) => "Error".to_string(),
    }
}

fn replace_percentage(expression: &str) -> String {
    let re = Regex::new(r"(\d+(\.\d+)?)%").unwrap();
    re.replace_all(expression, |caps: &regex::Captures| {
        format!("({} * 0.5)", &caps[1])
    }).to_string()
}