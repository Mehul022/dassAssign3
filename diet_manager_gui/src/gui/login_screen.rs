// src/gui/login_screen.rs
use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;
use std::fs;

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

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
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
                    // Store user_id instead of username
                    db.current_user = user.user_id.clone();
                    
                    // Save the database to JSON file
                    if let Err(e) = self.save_database(db) {
                        self.error_message = Some(format!("Failed to save login state: {}", e));
                        return;
                    }
                    
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

    fn save_database(&self, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(db)?;
        fs::write("database.json", json)?;
        Ok(())
    }
}