extern crate evalexpr;
extern crate gdk;
extern crate gtk;

use evalexpr::{eval_with_context, Context, Function, HashMapContext, Value};
use gtk::prelude::*;

fn main() {
    // Inicializar GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crear ventana principal
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Calculadora");

    // Crear cuadrícula para organizar botones
    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);
    grid.style_context().add_class("grid");
    window.add(&grid);

    // Crear un entry para mostrar la expresión y el resultado
    let entry = gtk::Entry::new();
    entry.set_editable(false);
entry.set_size_request(223, 150);
    entry.style_context().add_class("screen");
    entry.set_halign(gtk::Align::Center);
    entry.set_valign(gtk::Align::Center);
    grid.attach(&entry, 0, 0, 5, 1);

    entry.set_hexpand(true);
    entry.set_widget_name("screen");

    //Crear boton power
    let power_label = gtk::Label::new(None);
    power_label.set_text("⏻");
    let power_button = gtk::Button::new();
    power_button.add(&power_label);
    style_button(&power_button, "⏻");
    power_button.set_size_request(50, 50);

    // Agregar el botón de apagado a la cuadrícula
    grid.attach(&power_button, 3, 1, 1, 1);
    power_button.connect_clicked(|_| {
        gtk::main_quit();
    });

    // Añadir botones a la cuadrícula
    create_buttons(&grid, &entry);

    // Conectar evento de presionar tecla
    {
        let entry_clone = entry.clone();
        window.connect_key_press_event(move |_, key| {
            handle_key_press(key, &entry_clone);
            Inhibit(false)
        });
    }

    // Aplicar estilo a los botones
    apply_css();

    // Ajustar tamaño de la ventana al contenido
    window.set_resizable(false);
    window.show_all();
    gtk::main();
}

/// Crea los botones de la calculadora y los añade a la cuadrícula.
fn create_buttons(grid: &gtk::Grid, entry: &gtk::Entry) {
    let buttons = [
        ("ln", 1, 1, 0, 1),
        ("log", 1, 1, 1, 1),
        ("C", 1, 1, 2, 1),
        ("⏻", 1, 1, 3, 1),
        ("(", 1, 1, 1, 2),
        (")", 1, 1, 2, 2),
        ("/", 1, 1, 3, 2),
        ("7", 1, 1, 0, 3),
        ("8", 1, 1, 1, 3),
        ("9", 1, 1, 2, 3),
        ("x", 1, 1, 3, 3),
        ("4", 1, 1, 0, 4),
        ("5", 1, 1, 1, 4),
        ("6", 1, 1, 2, 4),
        ("-", 1, 1, 3, 4),
        ("1", 1, 1, 0, 5),
        ("2", 1, 1, 1, 5),
        ("3", 1, 1, 2, 5),
        ("+", 1, 2, 3, 5),
        ("0", 1, 1, 0, 6),
        (".", 1, 1, 1, 6),
        ("=", 1, 1, 2, 6),
    ];

    for &(label, width, height, col, row) in &buttons {
        let button = gtk::Button::with_label(label);
        style_button(&button, label);
        attach_button(grid, &button, entry, label, width, height, col, row);
    }
}

/// Aplica estilos a los botones según su etiqueta.
fn style_button(button: &gtk::Button, label: &str) {
    if ["/", "x", "-", "+", "=", "(", ")", "ln", "log"].contains(&label) {
        button.style_context().add_class("operation");
    } else if label == "C" {
        button.style_context().add_class("clear");
    } else if label == "⏻" {
        button.style_context().add_class("power");
    } else {
        button.style_context().add_class("normal");
    }
}

/// Añade el botón a la cuadrícula y conecta la señal de clic.
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

/// Maneja los eventos de tecla presionada para actualizar el entry de la calculadora.
fn handle_key_press(key: &gdk::EventKey, entry: &gtk::Entry) {
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
                if character.is_digit(10) || "+-*/().lnlog⏻".contains(character_str.as_str()) {
                    entry.set_text(&format!("{}{}", entry_text, character_str));
                }
            }
        }
    }
}

/// Aplica los estilos CSS a los elementos de la interfaz de usuario.
fn apply_css() {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Evalúa la expresión matemática y devuelve el resultado como cadena.
fn evaluate_expression(expression: &str) -> String {
    let expression = expression
        .replace("x", "*")
        .replace("ln", "lnf") // Para evitar conflicto con funciones de eval
        .replace("log", "log10f");

    // Definimos las funciones ln y log10
    let mut context = HashMapContext::new();
    context
        .set_function(
            "lnf".into(),
            Function::new(
                None,
                Box::new(|argument| {
                    let value = argument[0].as_number()?;
                    Ok(Value::from(value.ln()))
                }),
            ),
        )
        .unwrap();
    context
        .set_function(
            "log10f".into(),
            Function::new(
                None,
                Box::new(|argument| {
                    let value = argument[0].as_number()?;
                    Ok(Value::from(value.log10()))
                }),
            ),
        )
        .unwrap();

    eval_with_context(&expression, &context)
        .map_or("Error".to_string(), |result| result.to_string())
}
