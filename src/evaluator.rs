use evalexpr::*;
use std::f64::consts;
use std::sync::atomic::{AtomicBool, Ordering};

pub static DEGREE_MODE: AtomicBool = AtomicBool::new(false);

fn reg(c: &mut HashMapContext, name: &str, arity: usize, f: fn(f64) -> f64) {
    c.set_function(name.into(), Function::new(Some(arity), Box::new(move |args| {
        Ok(Value::from(f(args[0].as_number().unwrap_or(f64::NAN))))
    }))).unwrap();
}

fn deg_on() -> bool {
    DEGREE_MODE.load(Ordering::Relaxed)
}

fn context() -> HashMapContext {
    let mut c = HashMapContext::new();

    c.set_value("π".into(), Value::from(consts::PI)).unwrap();
    c.set_value("𝑒".into(), Value::from(consts::E)).unwrap();

    // Trig with degree/radian support
    c.set_function("sin".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        let r = if deg_on() { x.to_radians().sin() } else { x.sin() };
        Ok(Value::from(if r.is_nan() || r.is_infinite() { f64::NAN } else { r }))
    }))).unwrap();

    c.set_function("cos".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        let r = if deg_on() { x.to_radians().cos() } else { x.cos() };
        Ok(Value::from(if r.is_nan() || r.is_infinite() { f64::NAN } else { r }))
    }))).unwrap();

    c.set_function("tan".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        let r = if deg_on() { x.to_radians().tan() } else { x.tan() };
        Ok(Value::from(if r.is_nan() || r.is_infinite() || r.abs() > 1e15 { f64::NAN } else { r }))
    }))).unwrap();

    // Inverse trig — output depends on mode
    c.set_function("asin".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        if x < -1.0 || x > 1.0 { return Ok(Value::from(f64::NAN)); }
        let r = x.asin();
        let r = if deg_on() { r.to_degrees() } else { r };
        Ok(Value::from(if r.is_nan() { f64::NAN } else { r }))
    }))).unwrap();

    c.set_function("acos".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        if x < -1.0 || x > 1.0 { return Ok(Value::from(f64::NAN)); }
        let r = x.acos();
        let r = if deg_on() { r.to_degrees() } else { r };
        Ok(Value::from(if r.is_nan() { f64::NAN } else { r }))
    }))).unwrap();

    c.set_function("atan".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        let r = x.atan();
        let r = if deg_on() { r.to_degrees() } else { r };
        Ok(Value::from(if r.is_nan() { f64::NAN } else { r }))
    }))).unwrap();

    // Simple unary functions (just f(x))
    reg(&mut c, "sinh", 1, |x| x.sinh());
    reg(&mut c, "cosh", 1, |x| x.cosh());
    reg(&mut c, "tanh", 1, |x| x.tanh());
    reg(&mut c, "asinh", 1, |x| x.asinh());
    reg(&mut c, "acosh", 1, |x| if x < 1.0 { f64::NAN } else { x.acosh() });
    reg(&mut c, "atanh", 1, |x| if x <= -1.0 || x >= 1.0 { f64::NAN } else { x.atanh() });
    reg(&mut c, "ln", 1, |x| if x <= 0.0 { f64::NAN } else { x.ln() });
    reg(&mut c, "log", 1, |x| if x <= 0.0 { f64::NAN } else { x.log10() });
    reg(&mut c, "log2", 1, |x| if x <= 0.0 { f64::NAN } else { x.log2() });
    reg(&mut c, "exp", 1, |x| x.exp());
    reg(&mut c, "sqrt", 1, |x| if x < 0.0 { f64::NAN } else { x.sqrt() });
    reg(&mut c, "cbrt", 1, |x| x.cbrt());
    reg(&mut c, "abs", 1, |x| x.abs());

    // Factorial
    c.set_function("fact".into(), Function::new(Some(1), Box::new(|args| {
        let x = args[0].as_number().unwrap_or(f64::NAN);
        if x < 0.0 || x > 170.0 || (x - x.round()).abs() > 1e-10 {
            return Ok(Value::from(f64::NAN));
        }
        let n = x.round() as u64;
        let mut r = 1.0;
        for i in 2..=n { r *= i as f64; if r.is_infinite() { return Ok(Value::from(f64::NAN)); } }
        Ok(Value::from(r))
    }))).unwrap();

    c
}

pub fn evaluate_expression(expression: &str) -> String {
    let expr = expression
        .replace("×", "*")
        .replace("÷", "/")
        .replace("π", &consts::PI.to_string())
        .replace("𝑒", &consts::E.to_string());

    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let ctx = context();
    match eval_with_context(trimmed, &ctx) {
        Ok(value) => {
            let num: f64 = value.as_number().unwrap_or(f64::NAN);
            if num.is_nan() || num.is_infinite() {
                "Error".to_string()
            } else {
                let rounded = (num * 1_000_000_000.0).round() / 1_000_000_000.0;
                rounded.to_string()
            }
        }
        Err(_) => "Error".to_string(),
    }
}
