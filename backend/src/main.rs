mod controllers;
mod middleware;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret".to_string());

    let pool = PgPool::connect(&database_url).await?;

    // Run database migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations ran successfully");

    let shared_state = Arc::new(AppState {
        db: pool,
        jwt_secret,
    });

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/auth/register", post(controllers::auth::register))
        .route("/auth/login", post(controllers::auth::login))
        .route("/auth/profile", get(controllers::auth::get_profile))
        
        // Finance Routes
        .route("/categories", get(controllers::finance::categories::list_categories).post(controllers::finance::categories::create_category))
        .route("/categories/{id}", get(controllers::finance::categories::get_category).put(controllers::finance::categories::update_category).delete(controllers::finance::categories::delete_category))
        
        .route("/accounts", get(controllers::finance::accounts::list_accounts).post(controllers::finance::accounts::create_account))
        .route("/accounts/{id}", get(controllers::finance::accounts::get_account).put(controllers::finance::accounts::update_account).delete(controllers::finance::accounts::delete_account))
        
        .route("/transactions", get(controllers::finance::transactions::list_transactions).post(controllers::finance::transactions::create_transaction))
        .route("/transactions/{id}", get(controllers::finance::transactions::get_transaction).put(controllers::finance::transactions::update_transaction).delete(controllers::finance::transactions::delete_transaction))
        
        .route("/budgets", get(controllers::finance::budgets::list_budgets).post(controllers::finance::budgets::create_budget))
        .route("/budgets/{id}", get(controllers::finance::budgets::get_budget).put(controllers::finance::budgets::update_budget).delete(controllers::finance::budgets::delete_budget))
        
        // Productivity Routes
        .route("/goals", get(controllers::productivity::goals::list_goals).post(controllers::productivity::goals::create_goal))
        .route("/goals/{id}", get(controllers::productivity::goals::get_goal).put(controllers::productivity::goals::update_goal).delete(controllers::productivity::goals::delete_goal))
        
        .route("/tasks", get(controllers::productivity::tasks::list_tasks).post(controllers::productivity::tasks::create_task))
        .route("/tasks/{id}", get(controllers::productivity::tasks::get_task).put(controllers::productivity::tasks::update_task).delete(controllers::productivity::tasks::delete_task))
        
        // Lifestyle Routes
        .route("/journals", get(controllers::lifestyle::journals::list_journal_entries).post(controllers::lifestyle::journals::create_journal_entry))
        .route("/journals/{id}", get(controllers::lifestyle::journals::get_journal_entry).put(controllers::lifestyle::journals::update_journal_entry).delete(controllers::lifestyle::journals::delete_journal_entry))
        
        .route("/habits", get(controllers::lifestyle::habits::list_habits).post(controllers::lifestyle::habits::create_habit))
        .route("/habits/{id}", get(controllers::lifestyle::habits::get_habit).put(controllers::lifestyle::habits::update_habit).delete(controllers::lifestyle::habits::delete_habit))
        
        .route("/health/exercise", get(controllers::lifestyle::health::list_exercise_logs).post(controllers::lifestyle::health::create_exercise_log))
        .route("/health/exercise/{id}", get(controllers::lifestyle::health::get_exercise_log).put(controllers::lifestyle::health::update_exercise_log).delete(controllers::lifestyle::health::delete_exercise_log))
        
        .layer(cors)
        .with_state(shared_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}
