use std::f64;

#[derive(Debug, Clone)]
pub struct Solution {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

pub fn solve_2x2(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Result<Solution, String> {
    let det = a1 * b2 - a2 * b1;
    if det.abs() < 1e-12 {
        return Err("Sistema sin solución única (det = 0)".to_string());
    }
    let det_x = c1 * b2 - c2 * b1;
    let det_y = a1 * c2 - a2 * c1;
    Ok(Solution {
        x: det_x / det,
        y: det_y / det,
        z: None,
    })
}

pub fn det_3x3(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64,
    a3: f64, b3: f64, c3: f64,
) -> f64 {
    a1 * (b2 * c3 - b3 * c2)
    - b1 * (a2 * c3 - a3 * c2)
    + c1 * (a2 * b3 - a3 * b2)
}

pub fn solve_3x3(
    a1: f64, b1: f64, c1: f64, d1: f64,
    a2: f64, b2: f64, c2: f64, d2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64,
) -> Result<Solution, String> {
    let det = det_3x3(a1, b1, c1, a2, b2, c2, a3, b3, c3);
    if det.abs() < 1e-12 {
        return Err("Sistema sin solución única (det = 0)".to_string());
    }
    let det_x = det_3x3(d1, b1, c1, d2, b2, c2, d3, b3, c3);
    let det_y = det_3x3(a1, d1, c1, a2, d2, c2, a3, d3, c3);
    let det_z = det_3x3(a1, b1, d1, a2, b2, d2, a3, b3, d3);
    Ok(Solution {
        x: det_x / det,
        y: det_y / det,
        z: Some(det_z / det),
    })
}

pub fn format_solution(s: &Solution, is_3d: bool) -> String {
    let fmt = |v: f64| -> String {
        let r = (v * 1_000_000_000.0).round() / 1_000_000_000.0;
        if r == r { r.to_string() } else { "Error".to_string() }
    };
    if is_3d {
        format!("x = {}\ny = {}\nz = {}", fmt(s.x), fmt(s.y), fmt(s.z.unwrap_or(f64::NAN)))
    } else {
        format!("x = {}\ny = {}", fmt(s.x), fmt(s.y))
    }
}
