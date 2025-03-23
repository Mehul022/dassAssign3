// src/gui/login_screen.rs
use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;

pub struct LoginScreen {
    username: String,
    password: String,
    error_message: Option<String>,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &Database, current_state: &mut AppState) {
        ui.heading("Login");

        if let Some(error) = &self.error_message {
            ui.label(error);
        }

        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.text_edit_singleline(&mut self.password);
        });

        if ui.button("Login").clicked() {
            if let Some(user) = db.users.get(&self.username) {
                if user.password == self.password {
                    *current_state = AppState::Home;
                    self.error_message = None;
                } else {
                    self.error_message = Some("Incorrect password.".to_string());
                }
            } else {
                self.error_message = Some("User not found.".to_string());
            }
        }

        if ui.button("Register").clicked() {
            *current_state = AppState::Register;
        }
    }
}