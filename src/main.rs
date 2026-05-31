extern crate gtk;
extern crate gdk;

mod ui;
mod handlers;
mod evaluator;
mod styles;
mod equations;

fn main() {
    ui::run();
}
