use std::sync::Arc;

use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItem {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completion: Option<bool>,
    pub creation_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItemDTO {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completion: Option<bool>,
    pub creation_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItemSchemaUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completion: Option<bool>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TodoItemListQueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TodoItemQueryOptions {
    pub id: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<TodoItem>>>,
}

impl TodoItem {
    pub fn to_dto(&self) -> TodoItemDTO {
        TodoItemDTO {
            id: self.id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            completion: self.completion.clone(),
            creation_time: self.creation_time.clone(),
            update_time: self.update_time.clone(),
        }
    }
}
