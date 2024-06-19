use evalexpr::{eval_with_context, Context, Function, HashMapContext, Value, EvalexprError};
use regex::{Captures, Regex};
use std::error::Error;
use std::fmt;

const PI: f64 = 3.14159265358979323846264338327950288;
const E: f64 = 2.71828182845904523536028747135266250;

pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError({})", self.0)
    }
}

impl Error for CustomError {}

pub fn evaluate_expression(expression: &str) -> String {
    let expression = expression
        .replace("x", "*")
        .replace("𝑙𝑛", "lnf")
        .replace("𝑙𝑜𝑔", "log10f")
        .replace(" ^ ", "pow")
        .replace("π", &PI.to_string())
        .replace("𝑒", &E.to_string());

    let expression = replace_percentage(&expression);
    println!("Evaluating expression: {}", expression);

    let mut context = HashMapContext::new();

    context.set_value("π".to_string(), Value::from(PI)).unwrap();
    context.set_value("𝑒".to_string(), Value::from(E)).unwrap();

    context
        .set_function("lnf".into(), Function::new(Some(1), Box::new(lnf_function)))
        .unwrap();

    context
        .set_function("log10f".into(), Function::new(Some(1), Box::new(log10f_function)))
        .unwrap();

    context
        .set_function("pow".into(), Function::new(Some(2), Box::new(pow_function)))
        .unwrap();
    context
        .set_function("%".into(), Function::new(Some(2), Box::new(percentage_function)))
        .unwrap();

    let result = eval_with_context(&expression, &context);
    match result {
        Ok(value) => value.to_string(),
        Err(err) => {
            eprintln!("Evaluation error: {}", err);
            "Error".to_string()
        }
    }
}

fn lnf_function(args: &[Value]) -> Result<Value, EvalexprError> {
    check_argument_count(args, 1)?;
    let value = args[0].as_number()?;
    Ok(Value::from(value.ln()))
}

fn log10f_function(args: &[Value]) -> Result<Value, EvalexprError> {
    check_argument_count(args, 1)?;
    let value = args[0].as_number()?;
    Ok(Value::from(value.log10()))
}

fn pow_function(args: &[Value]) -> Result<Value, EvalexprError> {
    check_argument_count(args, 2)?;
    let base = args[0].as_number()?;
    let exponent = args[1].as_number()?;
    Ok(Value::from(base.powf(exponent)))
}

fn percentage_function(args: &[Value]) -> Result<Value, EvalexprError> {
    check_argument_count(args, 2)?;
    let number = args[0].as_number()?;
    let percentage = args[1].as_number()?;
    Ok(Value::from(number * (percentage / 100.0)))
}

fn check_argument_count(args: &[Value], expected_count: usize) -> Result<(), EvalexprError> {
    if args.len() != expected_count {
        return Err(EvalexprError::WrongFunctionArgumentAmount { actual: args.len(), expected: expected_count });
    }
    Ok(())
}
fn replace_percentage(expression: &str) -> String {
    let re = Regex::new(r"(\d+(\.\d+)?)(\s*%)(\s*)(\d+(\.\d+)?)?").unwrap();
    let replaced_expression = re.replace_all(expression, |caps: &Captures| {
        let number = caps.get(1).unwrap().as_str();
        let percentage = if let Some(matched) = caps.get(5) {
            matched.as_str().parse::<f64>().unwrap() / 100.0
        } else {
            1.0
        };
        format!("({} * {})", number, percentage)
    });

    replaced_expression.to_string()
}
