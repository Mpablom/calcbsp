use gtk::prelude::*;
use crate::handlers::{handle_key_press, create_buttons};
use crate::styles::apply_css;

pub fn run() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Calculadora");

    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);
    grid.style_context().add_class("grid");
    window.add(&grid);

    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_size_request(350, 150);
    entry.style_context().add_class("screen");
    entry.set_halign(gtk::Align::Center);
    entry.set_valign(gtk::Align::Center);
    entry.set_has_frame(false);
    grid.attach(&entry, 0, 0, 5, 1);

    entry.set_hexpand(true);
    entry.set_widget_name("screen");

    let power_label = gtk::Label::new(None);
    power_label.set_text("⏻");
    let power_button = gtk::Button::new();
    power_button.add(&power_label);
    crate::handlers::style_button(&power_button, "⏻");
    power_button.set_size_request(50, 50);

    grid.attach(&power_button, 4, 1, 1, 1);
    power_button.connect_clicked(|_| {
        gtk::main_quit();
    });

    create_buttons(&grid, &entry);

    let entry_clone = entry.clone();
    window.connect_key_press_event(move |_, key| {
        handle_key_press(key, &entry_clone);
        Inhibit(false)
    });

    window.connect_focus_in_event(move |_, _| {
        power_button.grab_focus();
        Inhibit(false)
    });


    apply_css();

    window.set_resizable(false);
    window.show_all();
    gtk::main();
}
