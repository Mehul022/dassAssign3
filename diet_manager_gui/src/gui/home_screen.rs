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
        undo_manager: &mut UndoManager, // Add undo_manager parameter
    ) {
        ui.heading("Home Screen");

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
            *current_state = AppState::Login; // Return to the login screen
        }

        // Undo button
        if ui.button("Undo").clicked() {
            if let Some(previous_db_state) = undo_manager.undo() {
                *db = previous_db_state; // Restore the previous state
            }
        }

        // Display calorie information
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let (total_calories, target_calories, difference) = db.calculate_calories(&db.current_user, &date);
        ui.label(format!("Total Calories Consumed: {:.2}", total_calories));
        ui.label(format!("Target Calorie Intake: {:.2}", target_calories));
        ui.label(format!("Difference: {:.2}", difference));
    }
}