use eframe::egui;
use crate::models::{Database, BasicFood};
use crate::app_state::AppState;

pub struct AddBasicFoodScreen {
    new_food_id: String,
    new_food_keywords: String,
    new_food_calories: String,
}

impl AddBasicFoodScreen {
    pub fn new() -> Self {
        Self {
            new_food_id: String::new(),
            new_food_keywords: String::new(),
            new_food_calories: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.heading("Add Basic Food");

        ui.horizontal(|ui| {
            ui.label("Identifier:");
            ui.text_edit_singleline(&mut self.new_food_id);
        });

        ui.horizontal(|ui| {
            ui.label("Keywords (comma-separated):");
            ui.text_edit_singleline(&mut self.new_food_keywords);
        });

        ui.horizontal(|ui| {
            ui.label("Calories:");
            ui.text_edit_singleline(&mut self.new_food_calories);
        });

        if ui.button("Save").clicked() {
            let keywords = self.new_food_keywords.split(',').map(|s| s.trim().to_string()).collect();
            let calories = self.new_food_calories.parse().unwrap_or(0.0);

            let food = BasicFood {
                id: self.new_food_id.clone(),
                name: self.new_food_id.clone(), // Use identifier as name for simplicity
                keywords,
                calories_per_serving: calories,
            };

            db.basic_foods.insert(self.new_food_id.clone(), food);
            *current_state = AppState::Home;
        }

        if ui.button("Cancel").clicked() {
            *current_state = AppState::Home;
        }
    }
}