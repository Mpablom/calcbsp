use gtk::prelude::*;
use crate::evaluator::evaluate_expression;

pub fn create_buttons(grid: &gtk::Grid, entry: &gtk::Entry) {
    let buttons = [
        ("ln", 1, 1, 0, 1),("log", 1, 1, 1, 1),("C", 1, 1, 2, 1),("⏻", 1, 1, 3, 1),
        ("^", 1, 1, 0, 2),("(", 1, 1, 1, 2),(")", 1, 1, 2, 2),("/", 1, 1, 3, 2),
        ("7", 1, 1, 0, 3),("8", 1, 1, 1, 3),("9", 1, 1, 2, 3),("x", 1, 1, 3, 3),
        ("4", 1, 1, 0, 4),("5", 1, 1, 1, 4),("6", 1, 1, 2, 4),("-", 1, 1, 3, 4),
        ("1", 1, 1, 0, 5),("2", 1, 1, 1, 5),("3", 1, 1, 2, 5),("+", 1, 2, 3, 5),
        ("0", 1, 1, 0, 6),(".", 1, 1, 1, 6),("=", 1, 1, 2, 6),
    ];

    for &(label, width, height, col, row) in &buttons {
        let button = if label == "^" {
            let label = gtk::Label::new(None);
            label.set_markup("x<sup>y</sup>"); 
            let button = gtk::Button::new();
            button.add(&label);
            button
        } else {
            gtk::Button::with_label(label)
        };
        style_button(&button, label);
        attach_button(grid, &button, entry, label, width, height, col, row);
    } 
}

pub fn style_button(button: &gtk::Button, label: &str) {
    if ["/", "x", "-", "+", "=", "(", ")", "^", "ln", "log"].contains(&label) {
        button.style_context().add_class("operation");
    } else if label == "C" {
        button.style_context().add_class("clear");
    } else if label == "⏻" {
        button.style_context().add_class("power");
    } else {
        button.style_context().add_class("normal");
    }
}

fn attach_button(
    grid: &gtk::Grid,
    button: &gtk::Button,
    entry: &gtk::Entry,
    label: &str,
    width: i32,
    height: i32,
    col: i32,
    row: i32,
) {
    let entry_clone = entry.clone();
    let label_clone = label.to_string();
    button.connect_clicked(move |_| {
        let text = entry_clone.text().to_string();
        let new_text = match label_clone.as_str() {
            "=" => evaluate_expression(&text),
            "C" => String::new(),
            _ => format!("{}{}", text, label_clone),
        };
        entry_clone.set_text(&new_text);
    });
    button.set_size_request(50 * width, 50 * height);
    grid.attach(button, col, row, width, height);
}

pub fn handle_key_press(key: &gdk::EventKey, entry: &gtk::Entry) {
    let keyval = key.keyval();
    let entry_text = entry.text().to_string();
    match keyval {
        gdk::keys::constants::Return => entry.set_text(&evaluate_expression(&entry_text)),
        gdk::keys::constants::Escape => entry.set_text(""),
        gdk::keys::constants::BackSpace => {
            let new_text = entry_text
                .chars()
                .take(entry_text.len().saturating_sub(1))
                .collect::<String>();
            entry.set_text(&new_text);
        }
        _ => {
            if let Some(character) = keyval.to_unicode() {
                let character_str = character.to_string();
                if character.is_digit(10) || "+-*/().lnlog^⏻".contains(character_str.as_str()) {
                    entry.set_text(&format!("{}{}", entry_text, character_str));
                }
            }
        }
    }
}
