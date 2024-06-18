extern crate evalexpr;
extern crate gdk;
extern crate gtk;

mod ui;
mod handlers;
mod evaluator;
mod styles;
mod trigonometricas;
mod calculator_state;
fn main() {
    ui::run();
}