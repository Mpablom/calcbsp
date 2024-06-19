
/// Calcula el seno de un ángulo en grados y redondea el resultado.
pub fn sin(input: &str) -> String {
    match input.parse::<f64>() {
        Ok(radians) => {
            let result = (radians.sin() * 1000000.0).round() / 1000000.0;
            result.to_string()
        },
        Err(_) => "Error".to_string(),
    }
}

/// Calcula el coseno de un ángulo en grados y redondea el resultado.
pub fn cos(input: &str) -> String {
    match input.parse::<f64>() {
        Ok(radians) => {
            let result = (radians.cos() * 1000000.0).round() / 1000000.0;
            result.to_string()
        },
        Err(_) => "Error".to_string(),
    }
}

/// Calcula la tangente de un ángulo en grados y redondea el resultado.
pub fn tan(input: &str) -> String {
    match input.parse::<f64>() {
        Ok(radians) => {
            let result = (radians.tan() * 1000000.0).round() / 1000000.0;
            result.to_string()
        },
        Err(_) => "Error".to_string(),
    }
}


/* /// Calcula la cosecante de un ángulo en radianes.
pub fn csc(x: f64) -> f64 {
    1.0 / sin(x)
}

/// Calcula la secante de un ángulo en radianes.
pub fn sec(x: f64) -> f64 {
    1.0 / cos(x)
}

/// Calcula la cotangente de un ángulo en radianes.
pub fn cot(x: f64) -> f64 {
    1.0 / tan(x)
} */
