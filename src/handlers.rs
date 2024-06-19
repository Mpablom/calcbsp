use gtk::prelude::*;
use crate::evaluator::evaluate_expression;
use crate::trigonometricas;

pub fn create_buttons(grid: &gtk::Grid, entry: &gtk::Entry) {
    let buttons = [
        ("C", 1, 1, 3, 1),("⏻", 1, 1, 4, 1),
        ("𝑙𝑛", 1, 1, 0, 3),("𝑙𝑜𝑔", 1, 1, 1, 3),("𝑒", 1,1,2,3),( "π", 1, 1, 3, 3),("%",1,1,4,3),
        ("^", 1, 1, 0, 4),("√", 1, 1, 1, 4),("𝑠𝑖𝑛", 1, 1, 2, 4),("𝑐𝑜𝑠", 1, 1, 3, 4),("𝑡𝑎𝑛",1,1,4,4),
        ("7", 1, 1, 0, 5),("8", 1, 1, 1, 5),("9", 1, 1, 2, 5),("(", 1, 1, 3, 5),(")",1,1,4,5),
        ("4", 1, 1, 0, 6),("5", 1, 1, 1, 6),("6", 1, 1, 2, 6),("x", 1, 1, 3, 6),("/", 1, 1, 4, 6),
        ("1", 1, 1, 0, 7),("2", 1, 1, 1, 7),("3", 1, 1, 2, 7),("+", 1, 1, 3, 7),("-",1,1,4,7),
        ("0", 1, 1, 0, 8),(".", 1, 1, 1, 8),("( - )", 1, 1, 2, 8),("=",2,1,3,8)
        
    ];

    for &(label, width, height, col, row) in &buttons {
        let button = if label == "^" {
            let label = gtk::Label::new(None);
            label.set_markup("𝑥<sup>𝑦</sup>"); 
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
    if ["/", "x", "-", "+", "=", "(", ")", "^", "√", "𝑙𝑛", "𝑙𝑜𝑔", "𝑠𝑖𝑛", "𝑐𝑜𝑠", "𝑡𝑎𝑛", "𝑒", "π", "%"].contains(&label) {
        button.style_context().add_class("operation");
    } else if label == "C" || label == "( - )" {
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
            "𝑠𝑖𝑛" => {
                let result = evaluate_expression(&text).parse::<f64>().ok()
                    .map_or_else(|| "Error".to_string(), |value| trigonometricas::sin(&value.to_string()));
                result
            },
            "𝑐𝑜𝑠" => {
                let result = evaluate_expression(&text).parse::<f64>().ok()
                    .map_or_else(|| "Error".to_string(), |value| trigonometricas::cos(&value.to_string()));
                result
            },
            "𝑡𝑎𝑛" => {
                let result = evaluate_expression(&text).parse::<f64>().ok()
                    .map_or_else(|| "Error".to_string(), |value| trigonometricas::tan(&value.to_string()));
                result
            },
             "√" => {
                if let Ok(number) = evaluate_expression(&text).parse::<f64>() {
                    let result = number.sqrt();
                    if result.is_nan() || result.is_infinite() {
                        "Error".to_string()
                    } else {
                        result.to_string()
                    }
                } else {
                    "Error".to_string()
                }
            },
            "%" => {
               let formatted_text = format!("{}%", text.trim());
                entry_clone.set_text(&formatted_text);
                return;
            },
            "( - )" => {
                let current_text = entry_clone.text().to_string();
                if current_text.starts_with('-') {
                    entry_clone.set_text(&current_text[1..]);
                } else {
                    entry_clone.set_text(&format!("-{}", current_text));
                }
                entry_clone.text().to_string()
            },
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
        gdk::keys::constants::Return => {
            entry.set_text(&evaluate_expression(&entry_text));
        },
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
                if character.is_digit(10) || "+-*/().𝑙𝑛𝑙𝑜𝑔^⏻%".contains(character_str.as_str()) {
                    entry.set_text(&format!("{}{}", entry_text, character_str));
                }
            }
        }
    }
}

