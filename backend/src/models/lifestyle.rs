use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct JournalEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub frequency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct HabitLog {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub date: DateTime<Utc>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LearningProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub topic: String,
    pub progress: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LearningSession {
    pub id: Uuid,
    pub learning_progress_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VisionBoard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VisionItem {
    pub id: Uuid,
    pub vision_board_id: Uuid,
    pub description: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MindMap {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MindMapNode {
    pub id: Uuid,
    pub mind_map_id: Uuid,
    pub parent_node_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExerciseLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub r#type: String,
    pub duration_minutes: i32,
    pub calories_burned: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NutritionLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub calories: f64,
    pub protein: f64,
    pub carbs: f64,
    pub fats: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FoodItem {
    pub id: Uuid,
    pub nutrition_log_id: Uuid,
    pub name: String,
    pub quantity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SleepLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub quality: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WaterIntakeLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub amount_liters: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MeditationLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub duration_minutes: i32,
    pub r#type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SymptomLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub symptom: String,
    pub severity: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub preferences: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Anniversary {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub r#type: String,
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ShoppingList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ShoppingItem {
    pub id: Uuid,
    pub shopping_list_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub purchased: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventoryCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub expiry_date: Option<DateTime<Utc>>,
    pub category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TravelPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub destination: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TravelItinerary {
    pub id: Uuid,
    pub travel_plan_id: Uuid,
    pub time: DateTime<Utc>,
    pub activity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub r#type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PetCareLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pet_id: Uuid,
    pub date: DateTime<Utc>,
    pub activity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SmartHomeIntegration {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_name: String,
    pub r#type: String,
    pub config: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CareerTracker {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub start_date: DateTime<Utc>,
    pub achievements: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CareerMilestone {
    pub id: Uuid,
    pub career_tracker_id: Uuid,
    pub description: String,
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NetworkReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub contact_id: Uuid,
    pub message: String,
    pub trigger_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreativeNote {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sketch {
    pub id: Uuid,
    pub creative_note_id: Uuid,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
