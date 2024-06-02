extern crate evalexpr;
extern crate gdk;
extern crate gtk;

mod ui;
mod handlers;
mod evaluator;
mod styles;
mod trigonometricas;
mod calculatorState;


fn main() {
    ui::run();
}