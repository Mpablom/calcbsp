extern crate gdk;
extern crate gtk;
use evalexpr::eval;
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
    window.add(&grid);

    // Crear un entry para mostrar la expresión y el resultado
    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_width_chars(30);
    entry.set_size_request(0, 150);
    grid.attach(&entry, 0, 0, 4, 1);

    // Especificaciones de botones
    let buttons = [
        ("C", 1, 1, 0, 1),("(", 1, 1, 1, 1),(")", 1, 1, 2, 1),("/", 1, 1, 3, 1),
        ("7", 1, 1, 0, 2),("8", 1, 1, 1, 2),("9", 1, 1, 2, 2),("x", 1, 1, 3, 2),
        ("4", 1, 1, 0, 3),("5", 1, 1, 1, 3),("6", 1, 1, 2, 3),("-", 1, 1, 3, 3),
        ("1", 1, 1, 0, 4),("2", 1, 1, 1, 4),("3", 1, 1, 2, 4),("+", 1, 2, 3, 4),
        ("0", 1, 1, 0, 5),(".", 1, 1, 1, 5),("=", 1, 1, 2, 5),
    ];

    for &(label, width, height, col, row) in &buttons {
        let button = gtk::Button::with_label(label);

        // Estiliza los botones de las operaciones, paréntesis y botón "C"
        if ["/", "x", "-", "+", "=", "(", ")"].contains(&label) {
            button.style_context().add_class("operation");
        } else if label == "C" {
            button.style_context().add_class("clear");
        }

        // Conectar señal de clic al botón
        let entry_clone = entry.clone();
        button.connect_clicked(move |_| {
            let text = entry_clone.text().to_string();
            let new_text = match label {
                "=" => evaluate_expression(&text),
                "C" => String::new(),
                _ => format!("{}{}", text, label),
            };
            entry_clone.set_text(&new_text);
        });

        // Establecer tamaño y agregar botón a la cuadrícula
        button.set_size_request(50 * width, 50 * height);
        grid.attach(&button, col, row, width, height);
    }

    // Conectar evento de presionar tecla
    window.connect_key_press_event(move |_, key| {
        let keyval = key.keyval();
        let _keystate = key.state();
        let entry_text = entry.text().to_string();

        // Compara el valor numérico de la tecla presionada
        match keyval {
            gdk::keys::constants::Return => entry.set_text(&evaluate_expression(&entry_text)),
            gdk::keys::constants::Escape => entry.set_text(""),
            gdk::keys::constants::BackSpace => {
                let mut chars = entry_text.chars();
                chars.next_back();
                entry.set_text(&chars.as_str())
            }
            _ => {
                if let Some(character) = keyval.to_unicode() {
                    let character_str = character.to_string();
                    if character_str
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .is_digit(10)
                        || "+-*/().".contains(character_str.as_str())
                    {
                        entry.set_text(&format!("{}{}", entry_text, character_str));
                    }
                }
            }
        }

        Inhibit(false)
    });

     // Aplicar estilo a los botones
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(
            "
        .operation {
            background-color: #FF5733;
            color: white;
            font-weight: bold;
        }
        .clear {
            background-color: #33B5FF;
            color: white;
            font-weight: bold;
        }
    "
            .as_bytes(),
        )
        .expect("Failed to load CSS");

    // Aplicar el proveedor CSS a la pantalla principal
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Ajustar tamaño de la ventana al contenido
    window.set_resizable(false);
    window.show_all();
    gtk::main();
}

fn evaluate_expression(expression: &str) -> String {
    // Evaluar la expresión y devolver el resultado
    let expression = expression.replace("x", "*");
    if let Ok(result) = eval(&expression) {
        result.to_string()
    } else {
        "Error".to_string()
    }
}
