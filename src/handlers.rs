use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use crate::evaluator;

pub struct AppState {
    pub degree_mode: bool,
    pub memory: f64,
    pub last_result: f64,
    pub entry_text: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            degree_mode: false,
            memory: 0.0,
            last_result: 0.0,
            entry_text: String::new(),
        }
    }
}

pub type SharedState = Rc<RefCell<AppState>>;

pub fn create_buttons(grid: &gtk::Grid, entry: &gtk::Entry, status: &gtk::Label, state: SharedState) {
    let buttons: Vec<(&str, i32, i32, i32, i32)> = vec![
        ("⏻", 1, 1, 0, 2), ("π", 1, 1, 1, 2), ("𝑒", 1, 1, 2, 2),
        ("°", 1, 1, 3, 2), ("C", 1, 1, 4, 2), ("⌫", 1, 1, 5, 2),

        ("x²", 1, 1, 0, 3), ("x³", 1, 1, 1, 3), ("xʸ", 1, 1, 2, 3),
        ("√", 1, 1, 3, 3), ("∛", 1, 1, 4, 3), ("%", 1, 1, 5, 3),

        ("10ˣ", 1, 1, 0, 4), ("eˣ", 1, 1, 1, 4), ("1/x", 1, 1, 2, 4),
        ("n!", 1, 1, 3, 4), ("(", 1, 1, 4, 4), (")", 1, 1, 5, 4),

        ("sin", 1, 1, 0, 5), ("cos", 1, 1, 1, 5), ("tan", 1, 1, 2, 5),
        ("ln", 1, 1, 3, 5), ("log", 1, 1, 4, 5), ("|x|", 1, 1, 5, 5),

        ("asin", 1, 1, 0, 6), ("acos", 1, 1, 1, 6), ("atan", 1, 1, 2, 6),
        ("sinh", 1, 1, 3, 6), ("cosh", 1, 1, 4, 6), ("tanh", 1, 1, 5, 6),

        ("asinh", 1, 1, 0, 7), ("acosh", 1, 1, 1, 7), ("atanh", 1, 1, 2, 7),
        ("log₂", 1, 1, 3, 7), ("Ans", 1, 1, 4, 7), ("EQN", 1, 1, 5, 7),

        ("MC", 1, 1, 0, 8), ("7", 1, 1, 1, 8), ("8", 1, 1, 2, 8),
        ("9", 1, 1, 3, 8), ("÷", 1, 1, 4, 8),

        ("MR", 1, 1, 0, 9), ("4", 1, 1, 1, 9), ("5", 1, 1, 2, 9),
        ("6", 1, 1, 3, 9), ("×", 1, 1, 4, 9),

        ("M+", 1, 1, 0, 10), ("1", 1, 1, 1, 10), ("2", 1, 1, 2, 10),
        ("3", 1, 1, 3, 10), ("+", 1, 1, 4, 10),

        ("M-", 1, 1, 0, 11), ("0", 1, 1, 1, 11), (".", 1, 1, 2, 11),
        ("±", 1, 1, 3, 11), ("-", 1, 1, 4, 11),
    ];

    for &(label, width, height, col, row) in &buttons {
        let button = gtk::Button::with_label(label);
        style_button(&button, label);
        let state_c = state.clone();
        let entry_c = entry.clone();
        let status_c = status.clone();
        let label_o = label.to_string();
        button.connect_clicked(move |_| {
            handle_button(&entry_c, &label_o, &state_c, &status_c);
        });
        button.set_size_request(55 * width, 46 * height);
        grid.attach(&button, col, row, width, height);
    }

    let eq_btn = gtk::Button::with_label("=");
    style_button(&eq_btn, "=");
    eq_btn.set_size_request(55, 184);
    let state_c = state.clone();
    let entry_c = entry.clone();
    let status_c = status.clone();
    eq_btn.connect_clicked(move |_| {
        handle_button(&entry_c, "=", &state_c, &status_c);
    });
    grid.attach(&eq_btn, 5, 8, 1, 4);
}

pub fn style_button(button: &gtk::Button, label: &str) {
    let ops = ["÷", "×", "+", "-", "=", "(", ")", "^", "xʸ", "√", "∛", "%",
               "sin", "cos", "tan", "asin", "acos", "atan",
               "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
               "ln", "log", "log₂", "|x|",
               "x²", "x³", "10ˣ", "eˣ", "1/x", "n!", "±", "EQN"];
    let clears = ["C", "CE", "⌫"];
    let memory = ["MC", "MR", "M+", "M-"];
    let constants = ["π", "𝑒"];
    let mods = ["°", "Ans"];

    if ops.contains(&label) {
        button.style_context().add_class("operation");
    } else if clears.contains(&label) {
        button.style_context().add_class("clear");
    } else if memory.contains(&label) {
        button.style_context().add_class("memory");
    } else if constants.contains(&label) {
        button.style_context().add_class("constant");
    } else if mods.contains(&label) {
        button.style_context().add_class("mode");
    } else if label == "⏻" {
        button.style_context().add_class("power");
    } else {
        button.style_context().add_class("normal");
    }
}

fn update_status(state: &AppState, status: &gtk::Label) {
    let mode = if state.degree_mode { "DEG" } else { "RAD" };
    let mem = if state.memory != 0.0 { "  M" } else { "" };
    status.set_text(&format!("{}{}", mode, mem));
}

fn handle_button(entry: &gtk::Entry, label: &str, state: &SharedState, status: &gtk::Label) {
    let text = entry.text().to_string();
    let mut s = state.borrow_mut();

    match label {
        "⏻" => gtk::main_quit(),

        "=" => {
            s.entry_text = text.clone();
            let result = evaluator::evaluate_expression(&text);
            if let Ok(num) = result.parse::<f64>() {
                s.last_result = num;
            }
            entry.set_text(&result);
        }
        "C" => { s.entry_text.clear(); entry.set_text(""); }
        "CE" => entry.set_text(""),
        "⌫" => {
            let new: String = text.chars().take(text.len().saturating_sub(1)).collect();
            entry.set_text(&new);
        }
        "±" => {
            if text.starts_with('-') {
                entry.set_text(&text[1..]);
            } else if !text.is_empty() && text != "0" && text != "Error" {
                entry.set_text(&format!("-{}", text));
            }
        }
        "π" => entry.set_text(&format!("{}{}", text, std::f64::consts::PI)),
        "𝑒" => entry.set_text(&format!("{}{}", text, std::f64::consts::E)),
        "°" => {
            s.degree_mode = !s.degree_mode;
            evaluator::DEGREE_MODE.store(s.degree_mode, std::sync::atomic::Ordering::Relaxed);
            update_status(&s, status);
        }
        "Ans" => entry.set_text(&format!("{}{}", text, s.last_result)),

        // Memory
        "MC" => { s.memory = 0.0; update_status(&s, status); }
        "MR" => entry.set_text(&format!("{}{}", text, s.memory)),
        "M+" => {
            if let Ok(num) = evaluator::evaluate_expression(&text).parse::<f64>() {
                s.memory += num; update_status(&s, status);
            }
        }
        "M-" => {
            if let Ok(num) = evaluator::evaluate_expression(&text).parse::<f64>() {
                s.memory -= num; update_status(&s, status);
            }
        }

        // Unary function buttons — insert func( or wrap
        "sin" => insert_func(entry, &text, "sin"),
        "cos" => insert_func(entry, &text, "cos"),
        "tan" => insert_func(entry, &text, "tan"),
        "asin" => insert_func(entry, &text, "asin"),
        "acos" => insert_func(entry, &text, "acos"),
        "atan" => insert_func(entry, &text, "atan"),
        "sinh" => insert_func(entry, &text, "sinh"),
        "cosh" => insert_func(entry, &text, "cosh"),
        "tanh" => insert_func(entry, &text, "tanh"),
        "asinh" => insert_func(entry, &text, "asinh"),
        "acosh" => insert_func(entry, &text, "acosh"),
        "atanh" => insert_func(entry, &text, "atanh"),
        "ln" => insert_func(entry, &text, "ln"),
        "log" => insert_func(entry, &text, "log"),
        "log₂" => insert_func(entry, &text, "log2"),
        "√" => insert_func(entry, &text, "sqrt"),
        "∛" => insert_func(entry, &text, "cbrt"),
        "|x|" => insert_func(entry, &text, "abs"),
        "eˣ" => insert_func(entry, &text, "exp"),
        "n!" => insert_func(entry, &text, "fact"),

        // Postfix operators
        "x²" => insert_postfix(entry, &text, "^2"),
        "x³" => insert_postfix(entry, &text, "^3"),
        "10ˣ" => {
    if text.is_empty() || text.chars().last().map_or(false, |c| "+-*/(".contains(c)) {
        entry.set_text(&format!("{}10^(", text));
    } else {
        entry.set_text(&format!("10^({})", text));
    }
},

        // Special
        "1/x" => insert_inv(entry, &text),
        "xʸ" => entry.set_text(&format!("{}**", text)),

        // Percentage: divide current value by 100
        "%" => {
            if let Ok(num) = evaluator::evaluate_expression(&text).parse::<f64>() {
                entry.set_text(&format!("{}", num / 100.0));
            }
        }

        "EQN" => show_eqn_window(),

        _ => { entry.set_text(&format!("{}{}", text, label)); }
    }
}

fn show_eqn_window() {
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    win.set_title("Sistema de Ecuaciones");
    win.set_resizable(false);
    win.set_position(gtk::WindowPosition::Center);
    win.set_modal(true);

    // Close on Escape
    win.connect_key_press_event(|w, key| {
        if key.keyval() == gdk::keys::constants::Escape {
            w.close();
            gtk::Inhibit(true)
        } else {
            gtk::Inhibit(false)
        }
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // Top bar: title + X close button
    let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let title_lbl = gtk::Label::new(Some("Sistema de Ecuaciones"));
    let close_btn = gtk::Button::with_label("✕");
    close_btn.set_size_request(30, 24);
    title_box.pack_start(&title_lbl, true, true, 0);
    title_box.pack_end(&close_btn, false, false, 0);
    vbox.add(&title_box);

    // Close button
    {
        let win = win.clone();
        close_btn.connect_clicked(move |_| {
            win.close();
        });
    }

    // Mode selection
    let radio_2 = gtk::RadioButton::with_label("2 incógnitas");
    let radio_3 = gtk::RadioButton::with_label_from_widget(&radio_2, "3 incógnitas");
    let mode_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    mode_box.add(&radio_2);
    mode_box.add(&radio_3);
    vbox.add(&mode_box);

    // Entry grid: 3 rows × 4 cols  (max for 3 unknowns: a1x+b1y+c1z=d1)
    let mut entries: Vec<Vec<gtk::Entry>> = Vec::new();
    let grid = gtk::Grid::new();
    grid.set_row_spacing(6);
    grid.set_column_spacing(4);
    grid.set_halign(gtk::Align::Center);

    let col_labels: Vec<gtk::Label> = (0..4).map(|c| {
        let lbl = gtk::Label::new(Some(["a", "b", "c", "d"][c]));
        lbl.set_halign(gtk::Align::Center);
        grid.attach(&lbl, c as i32, 0, 1, 1);
        lbl
    }).collect();

    for row in 0..3 {
        let mut row_entries = Vec::new();
        for col in 0..4 {
            let e = gtk::Entry::new();
            e.set_width_chars(7);
            e.set_halign(gtk::Align::Center);
            e.set_xalign(0.5);
            e.set_text("0");
            grid.attach(&e, col as i32, (row + 1) as i32, 1, 1);
            row_entries.push(e);
        }
        entries.push(row_entries);
    }
    vbox.add(&grid);

    // Solve button
    let solve_btn = gtk::Button::with_label("Resolver");
    solve_btn.set_margin_top(8);
    vbox.add(&solve_btn);

    // Result label
    let result = gtk::Label::new(Some("x = \ny = "));
    result.set_margin_top(8);
    result.set_halign(gtk::Align::Start);
    result.set_selectable(true);
    vbox.add(&result);

    win.add(&vbox);

    // Toggle visibility + update labels and result placeholder
    {
        let grid = grid.clone();
        let result = result.clone();
        let col_labels = col_labels.clone();
        radio_2.connect_toggled(move |r| {
            let active = r.is_active();
            for row in 1..=3 {
                if let Some(w) = grid.child_at(2, row) {
                    w.set_visible(!active);
                }
            }
            for c in 0..4 {
                if let Some(w) = grid.child_at(c, 3) {
                    w.set_visible(!active);
                }
            }
            // Toggle column 2 header label visibility
            col_labels[2].set_visible(!active);
            if active {
                col_labels[3].set_text("c");
                result.set_text("x = \ny = ");
            } else {
                col_labels[3].set_text("d");
                result.set_text("x = \ny = \nz = ");
            }
        });
    }

    // Solve
    let entries_c = entries.clone();
    let result_c = result.clone();
    let radio_3_c = radio_3.clone();
    solve_btn.connect_clicked(move |_| {
        let is_3d = radio_3_c.is_active();
        let mut vals: Vec<Vec<f64>> = Vec::new();
        for row in &entries_c {
            let mut r = Vec::new();
            for e in row {
                let t = e.text().to_string().replace(',', ".");
                match t.trim().parse::<f64>() {
                    Ok(v) => r.push(v),
                    Err(_) => {
                        result_c.set_text(&format!("Error en coeficiente: {}", t));
                        return;
                    }
                }
            }
            vals.push(r);
        }

        let res = if is_3d {
            crate::equations::solve_3x3(
                vals[0][0], vals[0][1], vals[0][2], vals[0][3],
                vals[1][0], vals[1][1], vals[1][2], vals[1][3],
                vals[2][0], vals[2][1], vals[2][2], vals[2][3],
            )
        } else {
            crate::equations::solve_2x2(
                vals[0][0], vals[0][1], vals[0][3],
                vals[1][0], vals[1][1], vals[1][3],
            )
        };

        match res {
            Ok(sol) => {
                result_c.set_text(&crate::equations::format_solution(&sol, is_3d));
            }
            Err(e) => {
                result_c.set_text(&e);
            }
        }
    });

    win.show_all();
    // Apply initial visibility for 2 unknowns mode (default):
    // hide column 2 (entries + label) for rows 1–3, hide row 3, relabel col 3 as "c"
    for row in 0..=3 {
        if let Some(w) = grid.child_at(2, row) {
            w.set_visible(false);
        }
    }
    for c in 0..4 {
        if let Some(w) = grid.child_at(c, 3) {
            w.set_visible(false);
        }
    }
    col_labels[3].set_text("c");
}

fn is_op_or_paren(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '(')
}

fn insert_func(entry: &gtk::Entry, text: &str, name: &str) {
    if text.is_empty() || text.chars().last().map_or(false, is_op_or_paren) {
        entry.set_text(&format!("{}{}(", text, name));
    } else {
        entry.set_text(&format!("{}({})", name, text));
    }
}

fn insert_postfix(entry: &gtk::Entry, text: &str, op: &str) {
    if !text.is_empty() && !text.chars().last().map_or(false, is_op_or_paren) && text != "Error" {
        entry.set_text(&format!("({}){}", text, op));
    }
}

fn insert_inv(entry: &gtk::Entry, text: &str) {
    if text.is_empty() || text.chars().last().map_or(false, is_op_or_paren) {
        entry.set_text(&format!("{}1/(", text));
    } else {
        entry.set_text(&format!("1/({})", text));
    }
}

pub fn handle_key_press(key: &gdk::EventKey, entry: &gtk::Entry, state: &SharedState, status: &gtk::Label) {
    let keyval = key.keyval();
    let text = entry.text().to_string();

    match keyval {
        gdk::keys::constants::Return | gdk::keys::constants::KP_Enter => {
            handle_button(entry, "=", state, status);
        }
        gdk::keys::constants::Escape => {
            handle_button(entry, "C", state, status);
        }
        gdk::keys::constants::BackSpace => {
            handle_button(entry, "⌫", state, status);
        }
        gdk::keys::constants::Delete => {
            handle_button(entry, "CE", state, status);
        }
        _ => {
            if let Some(c) = keyval.to_unicode() {
                if c.is_digit(10) || "+-*/().^%".contains(c) {
                    entry.set_text(&format!("{}{}", text, c));
                }
            }
        }
    }
}
