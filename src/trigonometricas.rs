const PI: f64 = 3.14159265358979323846264338327950288;

/// Calcula el seno de un ángulo en grados y redondea el resultado.
pub fn sin(x: f64) -> f64 {
    let radians = x * PI / 180.0;
    (radians.sin() * 1000000.0).round() / 1000000.0
}

/// Calcula el coseno de un ángulo en grados y redondea el resultado.
pub fn cos(x: f64) -> f64 {
    let radians = x * PI / 180.0;
    (radians.cos() * 1000000.0).round() / 1000000.0
}

/// Calcula la tangente de un ángulo en grados y redondea el resultado.
pub fn tan(x: f64) -> f64 {
    let radians = x * PI / 180.0;
    (radians.tan() * 1000000.0).round() / 1000000.0
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
