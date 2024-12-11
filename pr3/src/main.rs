use eframe::egui::{self, TextEdit, Label};
use crate::todolist::{authenticate, User, Task, registration};

mod todolist;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;


#[derive(Default)]
struct MyApp {
    is_authenticated: bool,
    username: String,
    password: String,
    error_message: String,
    new_task_title: String,
    new_task_deadline: String,
    user: User
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_authenticated {
                self.show_task_list(ui);
            } else {
                self.show_login(ui);
            }
        });
    }
}

impl MyApp {
    fn show_login(&mut self, ui: &mut egui::Ui) {
        ui.heading("Авторизація");
        ui.horizontal(|ui| {
            ui.label("Логін:");
            ui.text_edit_singleline(&mut self.username);
        });
        ui.horizontal(|ui| {
            ui.label("Пароль:");
            ui.add(TextEdit::singleline(&mut self.password).password(true));
        });
        ui.horizontal(|ui| {
            if ui.button("Увійти").clicked() {
                match authenticate(self.username.clone(), self.password.clone()) {
                    Ok(user) => {
                        self.user = user;
                        self.is_authenticated = true;
                        self.error_message.clear();
                    }
                    Err(error_text) => {
                        self.error_message = error_text;
                    }
                }
            }

            if ui.button("Зареєструватися").clicked() {
                match registration(self.username.clone(), self.password.clone()) {
                    Ok(user) => {
                        self.user = user;
                        self.is_authenticated = true;
                        self.error_message.clear();
                    }
                    Err(error_text) => {
                        self.error_message = error_text;
                    }
                }
            }
        });
        ui.label(self.error_message.clone());

    }

    fn show_task_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Список задач");

        egui::Grid::new("task_table").striped(true).num_columns(4).show(ui, |ui| {
            ui.add_sized([WINDOW_WIDTH * 0.1, 20.0], Label::new("Статус"));
            ui.add_sized([WINDOW_WIDTH * 0.5, 20.0], Label::new("Задача"));
            ui.add_sized([WINDOW_WIDTH * 0.2, 20.0], Label::new("Дедлайн"));
            ui.add_sized([WINDOW_WIDTH * 0.2, 20.0], Label::new(" "));
            ui.end_row();

            for mut task in self.user.get_tasks() {
                ui.checkbox(&mut task.done, "").changed().then(|| {
                    self.user.update_task(task.clone());
                });
                let task_title = ui.add(TextEdit::singleline(&mut task.title));
                if task_title.changed() {
                    self.user.update_task(task.clone());
                }
                let task_deadline = ui.add(TextEdit::singleline(&mut task.deadline));
                if task_deadline.changed() {
                    self.user.update_task(task.clone());
                }
                if ui.button("Видалити").clicked() {
                    self.user.delete_task(task);
                }
                ui.end_row();
            }

            ui.label(" ");
            ui.add(TextEdit::singleline(&mut self.new_task_title));
            ui.add(TextEdit::singleline(&mut self.new_task_deadline));
            if ui.button("Додати").clicked() {
                if !self.new_task_title.trim().is_empty() {
                    self.user.add_task(Task::new(self.new_task_title.clone(),
                                                 self.new_task_deadline.clone(),
                                                 false));
                    self.new_task_title.clear();
                    self.new_task_deadline.clear();
                }
            }
            ui.end_row();
        });

        if ui.button("Вийти").clicked() {
            self.is_authenticated = false;
            self.username.clear();
            self.password.clear();
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "ToDo List",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).expect("TODO: panic message");
}