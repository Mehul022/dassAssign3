use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;
use crate::gui::undo_manager::UndoManager;

pub struct HomeScreen;

impl HomeScreen {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
        undo_manager: &mut UndoManager,
    ) {
        ui.heading("Home Screen");

        // Show current user's username if available
        if let Some(user) = db.users.values().find(|u| u.user_id == db.current_user) {
            ui.label(format!("Welcome, {}!", user.username));
        }

        // Navigation buttons
        if ui.button("Add Basic Food").clicked() {
            *current_state = AppState::AddBasicFood;
        }
        if ui.button("Add Composite Food").clicked() {
            *current_state = AppState::AddCompositeFood;
        }
        if ui.button("View Daily Log").clicked() {
            *current_state = AppState::ViewDailyLog;
        }
        if ui.button("Add Food to Log").clicked() {
            *current_state = AppState::AddFoodToLog;
        }
        if ui.button("Edit Food Log").clicked() {
            *current_state = AppState::EditFoodLog;
        }
        if ui.button("Update Profile").clicked() {
            *current_state = AppState::UpdateProfile;
        }

        // Logout button
        if ui.button("Logout").clicked() {
            db.current_user.clear();
            *current_state = AppState::Login;
        }

        // Undo button
        if ui.button("Undo").clicked() {
            if let Some(previous_db_state) = undo_manager.undo() {
                *db = previous_db_state;
            }
        }

        // Calculate and display calorie information for current user
        if !db.current_user.is_empty() {
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
            
            // Calculate calories using user_id directly
            let (total_calories, target_calories, difference) = self.calculate_user_calories(db, &db.current_user, &date);
            
            ui.separator();
            ui.heading("Daily Nutrition Summary");
            ui.label(format!("Total Calories Consumed: {:.2}", total_calories));
            ui.label(format!("Calories Goal: {:.2}", target_calories));
            
            if difference >= 0.0 {
                ui.label(egui::RichText::new(format!("Remaining: {:.2}", difference)).color(egui::Color32::GREEN));
            } else {
                ui.label(egui::RichText::new(format!("Over by: {:.2}", difference.abs())).color(egui::Color32::RED));
            }

            if target_calories > 0.0 {
                let progress = total_calories / target_calories;
                ui.add(egui::ProgressBar::new(progress.clamp(0.0, 1.0)).text(format!(
                    "{:.1}% of daily goal", 
                    progress * 100.0
                )));
            }
        } else {
            ui.label("No user logged in");
        }
    }

    fn calculate_user_calories(
        &self,
        db: &Database,
        user_id: &str,
        date: &str
    ) -> (f32, f32, f32) {
        // Calculate total calories consumed from food logs
        let total_calories = db.food_logs
            .get(user_id)
            .map_or(0.0, |entries| {
                entries.iter()
                    .filter(|entry| entry.date == date)
                    .map(|entry| db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings)
                    .sum()
            });

        // Get target calories from user profile
        let target_calories = db.users.values()
            .find(|u| u.user_id == user_id)
            .map(|user| user.profile.calculate_target_calories())
            .unwrap_or(0.0);

        let difference = target_calories - total_calories;
        (total_calories, target_calories, difference)
    }
}