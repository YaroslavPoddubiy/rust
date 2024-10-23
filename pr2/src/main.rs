mod calculator;
use eframe::egui::{self, Button, CentralPanel, TextEdit};
use eframe::{App, Frame};
use egui::{Grid, Vec2};
use crate::calculator::Calculator;

const WINDOW_WIDTH: f32 = 300.0;
const WINDOW_HEIGHT: f32 = 300.0;

#[derive(Default)]
struct CalculatorApp {
    calculator: Calculator,
    display: String,
    current_input: String,
    expression: String,
    operator: Option<char>,
    result: Option<f32>,
}

impl CalculatorApp {

    fn handle_input(&mut self, input: &str) {
        if input == "C" {
            self.clear();
        }
        else if input == "<-" {
            if !self.expression.is_empty() {
                self.expression.pop();
                self.update_display();
            }
        }
        else if input == "=" {
            if !self.expression.is_empty() && self.expression.chars().last().unwrap().is_digit(10) {
                let result = self.calculator.calculate(self.expression.clone());
                match result{
                    Ok(res) => {self.expression = res.to_string(); self.update_display();},
                    Err(error) => {
                        self.expression = error.to_string();
                        self.update_display();
                        self.expression.clear();
                    }
                }
            }
        }
        else if input == " " {
            return;
        }
        else{
            if self.check_input(input.chars().next().unwrap()) {
                self.expression.push(input.chars().next().unwrap());
            }
            else {
                if !self.expression.is_empty() {
                    self.expression.pop();
                    self.expression.push(input.chars().next().unwrap());
                }
            }
            self.update_display();
        }

    }

    fn check_input(&mut self, input: char) -> bool {
        if "+-/*^".contains(input) {
            if self.expression.is_empty() || !self.expression.chars().last().unwrap().is_digit(10) {
                return false;
            }
        }
        if input == '.' && (self.expression.is_empty() || !self.expression.chars().last().unwrap().is_digit(10)){
            return false;
        }
        true
    }

    fn update_display(&mut self) {
        self.display = self.expression.clone();
    }

    fn clear(&mut self) {
        self.display.clear();
        self.current_input.clear();
        self.operator = None;
        self.result = None;
        self.expression.clear();
    }
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let buttons = [
                    ["7", "8", "9", "+"],
                    ["4", "5", "6", "-"],
                    ["1", "2", "3", "*"],
                    ["0", ".", "=", "/"],
                    ["C", "<-", " ", "^"]
                ];

                let font_size = 20.0;

                let rows = buttons.len() as f32;
                let columns = buttons[0].len() as f32;

                ui.add(TextEdit::singleline(
                    &mut self.display).font(
                    egui::FontId::monospace(font_size)).interactive(false).desired_width(WINDOW_WIDTH));

                Grid::new("calculator_grid").num_columns(columns as usize).spacing(Vec2::new(10.0, 10.0)).show(ui, |ui| {
                    let button_size = Vec2::new(
                        (WINDOW_WIDTH - 5.0 - 10.0 * columns) / columns,
                        (WINDOW_HEIGHT - 5.0 - font_size - 10.0 * rows) / rows);

                    for row in buttons{
                        for input in row {
                            if ui.add_sized(button_size, Button::new(input.to_string())).clicked() {
                                self.handle_input(input);
                            }
                        }
                        ui.end_row();
                    }

                });
            });
        });
    }
}

fn main() {
    let app = CalculatorApp::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native("Calculator", native_options, Box::new(|_cc|
        Ok(Box::new(app)))).expect("Помилка");
}