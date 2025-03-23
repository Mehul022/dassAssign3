use eframe::egui;
use chrono::NaiveDate;
use crate::models::{Database, UserProfile};
use crate::app_state::AppState;
use crate::models::Gender;
use crate::models::ActivityLevel;

pub struct ViewDailyLogScreen {
    selected_date: NaiveDate,
}

impl ViewDailyLogScreen {
    pub fn new() -> Self {
        Self {
            selected_date: chrono::Local::now().date_naive(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.heading("Daily Log");

        ui.horizontal(|ui| {
            ui.label("Select Date:");
            if ui.button("◄").clicked() {
                self.selected_date -= chrono::Duration::days(1);
            }
            ui.label(self.selected_date.format("%Y-%m-%d").to_string());
            if ui.button("►").clicked() {
                self.selected_date += chrono::Duration::days(1);
            }
        });

        // Calculate total calories for the selected date
        let selected_date_str = self.selected_date.format("%Y-%m-%d").to_string();
        let total_calories: f32 = db.food_logs
            .get(&selected_date_str)
            .map_or(0.0, |entries| {
                entries.iter()
                    .map(|entry| db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings)
                    .sum()
            });

        // Calculate daily calorie goal
        let calories_goal = if let Some(user) = db.users.values().next() {
            self.calculate_daily_goal(&user.profile)
        } else {
            0.0
        };

        let calories_remaining = calories_goal - total_calories;

        // Display calorie information
        ui.label(format!("Calories Consumed: {:.1}", total_calories));
        ui.label(format!("Calories Goal: {:.1}", calories_goal));
        ui.label(format!("Calories Remaining: {:.1}", calories_remaining));

        // Display log entries for the selected date
        if let Some(entries) = db.food_logs.get(&selected_date_str) {
            let mut entries_to_delete = Vec::new();

            for (i, entry) in entries.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}. {} ({} servings)", i + 1, entry.food_id, entry.servings));
                    if ui.button("Delete").clicked() {
                        entries_to_delete.push(i);
                    }
                });
            }

            // Delete entries after the loop
            if !entries_to_delete.is_empty() {
                let entries = db.food_logs.get_mut(&selected_date_str).unwrap();
                for &index in entries_to_delete.iter().rev() {
                    entries.remove(index);
                }
            }
        } else {
            ui.label("No entries for this date.");
        }

        if ui.button("Back to Home").clicked() {
            *current_state = AppState::Home;
        }
    }

    fn calculate_daily_goal(&self, profile: &UserProfile) -> f32 {
        // Example calculation using the Harris-Benedict equation
        let bmr = match profile.gender {
            Gender::Male => 88.362 + (13.397 * profile.weight_kg) + (4.799 * profile.height_cm) - (5.677 * profile.age as f32),
            Gender::Female => 447.593 + (9.247 * profile.weight_kg) + (3.098 * profile.height_cm) - (4.330 * profile.age as f32),
        };

        let activity_multiplier = match profile.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9,
        };

        bmr * activity_multiplier
    }
}