extern crate gtk;
extern crate gdk;
use gtk::prelude::*;
use evalexpr::eval;

fn main() {
    // Inicializa GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crea una ventana principal
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Calculadora");

    // Crea un layout de cuadrícula para organizar los botones
    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    window.add(&grid);

    // Crea un entry para mostrar la expresión y el resultado
    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_width_chars(30);
    entry.set_size_request(0, 80); // Doble de alto
    grid.attach(&entry, 0, 0, 4, 1);

    // Distribución de botones según especificaciones
    let buttons = [
        ("(", 1, 1, 0, 1), (")", 1, 1, 1, 1), ("C", 1, 1, 2, 1), ("/", 1, 1, 3, 1),
        ("7", 1, 1, 0, 2), ("8", 1, 1, 1, 2), ("9", 1, 1, 2, 2), ("x", 1, 1, 3, 2),
        ("4", 1, 1, 0, 3), ("5", 1, 1, 1, 3), ("6", 1, 1, 2, 3), ("-", 1, 1, 3, 3),
        ("1", 1, 1, 0, 4), ("2", 1, 1, 1, 4), ("3", 1, 1, 2, 4), ("+", 1, 2, 3, 4),
        ("0", 1, 1, 0, 5), (".", 1, 1, 1, 5), ("=", 1, 1, 2, 5)
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

        // Establecer tamaño y agregar botón a la cuadrícula
        button.set_size_request(50 * width, 50 * height);
        grid.attach(&button, col, row, width, height);
    }

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

    // Ajustar el tamaño de la ventana al contenido
    window.set_resizable(false);
    window.show_all();
    gtk::main();
}

fn evaluate_expression(expression: &str) -> String {
    // Evaluar la expresión y devolver el resultado
    if let Ok(result) = eval(expression) {
        result.to_string()
    } else {
        "Error".to_string()
    }
}
