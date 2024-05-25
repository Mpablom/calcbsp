extern crate evalexpr;
extern crate gdk;
extern crate gtk;

mod ui;
mod handlers;
mod evaluator;
mod styles;

use gtk::prelude::*;

fn main() {
    ui::run();
}