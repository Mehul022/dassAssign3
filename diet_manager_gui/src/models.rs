use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicFood {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub calories_per_serving: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeFood {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub components: Vec<FoodComponent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoodComponent {
    pub food_id: String,
    pub servings: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoodLogEntry {
    pub date: String, // ISO 8601 date format (e.g., "2023-10-01")
    pub food_id: String,
    pub servings: f32,
    pub user_id: String, // Add user_id to associate with a user
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    VeryActive,
    ExtraActive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CalorieCalculationMethod {
    HarrisBenedict,
    MifflinStJeor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub gender: Gender,
    pub height_cm: f32,
    pub age: u32,
    pub calorie_method: CalorieCalculationMethod,
    pub weight_kg: f32,
    pub activity_level: ActivityLevel,
}

impl UserProfile {
    pub fn calculate_target_calories(&self) -> f32 {
        let bmr = match self.gender {
            Gender::Male => 88.362 + (13.397 * self.weight_kg) + (4.799 * self.height_cm) - (5.677 * self.age as f32),
            Gender::Female => 447.593 + (9.247 * self.weight_kg) + (3.098 * self.height_cm) - (4.330 * self.age as f32),
        };
        bmr * match self.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String, // In a real application, this should be hashed
    pub profile: UserProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub users: HashMap<String, User>, // Key: username, Value: User
    pub basic_foods: HashMap<String, BasicFood>,
    pub composite_foods: HashMap<String, CompositeFood>,
    pub food_logs: HashMap<String, Vec<FoodLogEntry>>, // Key: username, Value: logs
    pub current_user: String, // Track the currently logged-in user
}

impl Database {
    pub fn get_food_calories(&self, food_id: &str) -> Option<f32> {
        if let Some(basic_food) = self.basic_foods.get(food_id) {
            Some(basic_food.calories_per_serving)
        } else if let Some(composite_food) = self.composite_foods.get(food_id) {
            let mut total_calories = 0.0;
            for component in &composite_food.components {
                if let Some(calories) = self.get_food_calories(&component.food_id) {
                    total_calories += calories * component.servings;
                }
            }
            Some(total_calories)
        } else {
            None
        }
    }

    pub fn calculate_calories(&self, username: &str, date: &str) -> (f32, f32, f32) {
        let mut total_calories = 0.0;
        if let Some(entries) = self.food_logs.get(username) {
            for entry in entries {
                if entry.date == date {
                    if let Some(calories) = self.get_food_calories(&entry.food_id) {
                        total_calories += calories * entry.servings;
                    }
                }
            }
        }

        let target_calories = if let Some(user) = self.users.get(username) {
            user.profile.calculate_target_calories()
        } else {
            0.0
        };

        let difference = target_calories - total_calories;
        (total_calories, target_calories, difference)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            users: HashMap::new(),
            basic_foods: HashMap::new(),
            composite_foods: HashMap::new(),
            food_logs: HashMap::new(),
            current_user: String::new(), // Initialize current_user as empty
        }
    }
}