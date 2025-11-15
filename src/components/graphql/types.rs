use juniper::{GraphQLObject, GraphQLInputObject};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Quiz GraphQL Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub difficulty: String,
    pub question_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Question GraphQL Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub quiz_id: String,
    pub text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub points: i32,
    pub created_at: String,
}

/// Answer Option GraphQL Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct AnswerOption {
    pub id: String,
    pub question_id: String,
    pub text: String,
    pub is_correct: bool,
}

/// Quiz Session GraphQL Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct QuizSession {
    pub id: String,
    pub user_id: String,
    pub quiz_id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub score: Option<i32>,
    pub status: String,
}

/// User GraphQL Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub created_at: String,
}

/// Quiz Input Type for creating quizzes
#[derive(GraphQLInputObject, Debug, Clone)]
pub struct CreateQuizInput {
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub difficulty: String,
}

/// Question Input Type for creating questions
#[derive(GraphQLInputObject, Debug, Clone)]
pub struct CreateQuestionInput {
    pub quiz_id: String,
    pub text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub points: i32,
}

/// Answer Option Input Type
#[derive(GraphQLInputObject, Debug, Clone)]
pub struct CreateAnswerOptionInput {
    pub question_id: String,
    pub text: String,
    pub is_correct: bool,
}

/// Quiz Session Input Type
#[derive(GraphQLInputObject, Debug, Clone)]
pub struct CreateQuizSessionInput {
    pub user_id: String,
    pub quiz_id: String,
}

/// Submit Answer Input Type
#[derive(GraphQLInputObject, Debug, Clone)]
pub struct SubmitAnswerInput {
    pub session_id: String,
    pub question_id: String,
    pub answer: String,
}

/// Quiz Result Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub session_id: String,
    pub total_questions: i32,
    pub correct_answers: i32,
    pub score: i32,
    pub percentage: f64,
    pub time_spent_seconds: i32,
}

/// Generic Response Type
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
}
