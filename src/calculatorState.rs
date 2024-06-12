use std::collections::VecDeque;

use crate::evaluator::evaluate_expression;

struct CalculatorState {
    entry_text: String,
    parenthesis_stack: VecDeque<char>,
}

impl CalculatorState {
    pub fn new() -> Self {
        CalculatorState {
            entry_text: String::new(),
            parenthesis_stack: VecDeque::new(),
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        match input {
            "sin" | "cos" | "tan" => {
                self.entry_text.push_str(input);
                self.entry_text.push('(');
                self.parenthesis_stack.push_back(')');
            },
            ")" => {
                if let Some(')') = self.parenthesis_stack.pop_back() {
                    self.entry_text.push(')');
                }
            },
            "√" => {
                if let Ok(number) = self.entry_text.parse::<f64>() {
                    let result = number.sqrt();
                    if result.is_nan() || result.is_infinite() {
                        self.entry_text = String::from("Error");
                    } else {
                        self.entry_text = result.to_string();
                    }
                }
            },
            "( - )" => {
                if self.entry_text.starts_with('-') {
                    self.entry_text.remove(0);
                } else {
                    self.entry_text.insert(0, '-');
                }
            },
            "=" => {
                while let Some(')') = self.parenthesis_stack.pop_back() {
                    self.entry_text.push(')');
                }
                self.entry_text = evaluate_expression(&self.entry_text);
            },
            _ if input.ends_with('%') => {
                let number = &input[..input.len() - 1];
                match number.parse::<f64>() {
                    Ok(n) => self.entry_text.push_str(&(n / 100.0).to_string()),
                    Err(_) => self.entry_text.push_str("Error: entrada no válida"),
                }
            },
            _ => {
                if !self.parenthesis_stack.is_empty() && input.chars().all(|c| c.is_digit(10) || c == '.' || "+-*/".contains(c)) {
                    self.entry_text.push_str(input);
                    if let Some(&next_char) = self.parenthesis_stack.back() {
                        self.entry_text.push(next_char);
                    }
                } else {
                    self.entry_text.push_str(input);
                }
            },
        }
    }
}