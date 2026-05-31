use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use crate::handlers::{self, SharedState, AppState};
use crate::styles::apply_css;

pub fn run() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Calculadora Científica");
    window.set_resizable(false);
    window.set_position(gtk::WindowPosition::Center);

    let grid = gtk::Grid::new();
    grid.set_row_spacing(4);
    grid.set_column_spacing(4);
    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);
    grid.style_context().add_class("grid");
    window.add(&grid);

    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_size_request(360, 55);
    entry.style_context().add_class("screen");
    entry.set_halign(gtk::Align::Fill);
    entry.set_valign(gtk::Align::Center);
    entry.set_has_frame(false);
    entry.set_xalign(1.0);
    grid.attach(&entry, 0, 0, 6, 1);

    let status = gtk::Label::new(Some("RAD"));
    status.set_halign(gtk::Align::Start);
    status.set_valign(gtk::Align::Center);
    status.style_context().add_class("status");
    grid.attach(&status, 0, 1, 6, 1);

    let state: SharedState = Rc::new(RefCell::new(AppState::new()));

    handlers::create_buttons(&grid, &entry, &status, state.clone());

    let entry_c = entry.clone();
    let state_c = state.clone();
    let status_c = status.clone();
    window.connect_key_press_event(move |_, key| {
        handlers::handle_key_press(key, &entry_c, &state_c, &status_c);
        Inhibit(false)
    });

    apply_css();

    window.show_all();
    gtk::main();
}
