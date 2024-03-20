use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use uuid::Uuid;

use crate::{
    model::{AppState, TodoItem, TodoItemDTO, TodoItemListQueryOptions, TodoItemSchemaUpdate},
    response::{
        TodoItemData, TodoItemDataListResponse, TodoItemDataResponse, TodoItemDataResponseSecret,
        TodoItemDataSecret,
    },
};

pub async fn get_health() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
        "message": "all good"
    });
    Json(json_response)
}

pub async fn get_index() -> Html<&'static str> {
    Html(std::include_str!("../index.html"))
}

pub async fn get_todo_items(
    options: Option<Query<TodoItemListQueryOptions>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let todo_item_list = app_state.db.lock().await;

    let Query(options) = options.unwrap_or_default();

    let limit = options.limit.unwrap_or(10);
    let page = options.page.unwrap_or(0);
    let offset = page * limit;

    let todo_item_list: Vec<TodoItemDTO> = todo_item_list
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|item| item.to_dto())
        .collect();

    let json_response = TodoItemDataListResponse {
        status: "success".to_string(),
        page: page,
        result_count: todo_item_list.len(),
        todo_item_list,
    };

    Json(json_response)
}

pub async fn post_todo_item(
    State(app_state): State<AppState>,
    Json(mut body): Json<TodoItem>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut todo_item_list = app_state.db.lock().await;

    if let Some(todo) = todo_item_list.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let datetime = chrono::Utc::now();

    body.id = Some(Uuid::new_v4().to_string());
    body.completion = Some(false);
    body.creation_time = Some(datetime);
    body.update_time = Some(datetime);

    let todo_item = body.to_owned();

    todo_item_list.push(body);

    let json_response = TodoItemDataResponse {
        status: "success".to_string(),
        todo_item_data: TodoItemData {
            todo_item: todo_item.to_dto(),
        },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_todo_item_path(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let todo_item_list = app_state.db.lock().await;
    let id = id.to_string();

    if let Some(todo_item) = todo_item_list
        .iter()
        .find(|todo| todo.id == Some(id.to_owned()))
    {
        let json_response = TodoItemDataResponse {
            status: "success".to_string(),
            todo_item_data: TodoItemData {
                todo_item: todo_item.clone().to_dto(),
            },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn patch_todo_item_path(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
    Json(body): Json<TodoItemSchemaUpdate>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut todo_item_list = app_state.db.lock().await;
    let id = id.to_string();

    if let Some(todo) = todo_item_list
        .iter_mut()
        .find(|todo| todo.id == Some(id.clone()))
    {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| todo.title.to_owned());
        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| todo.content.to_owned());
        let completed = body.completion.unwrap_or(todo.completion.unwrap());
        let secret = body.secret.to_owned();
        let payload = TodoItem {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completion: Some(completed),
            creation_time: todo.creation_time,
            update_time: Some(datetime),
            secret: secret,
        };
        *todo = payload;

        let json_response = TodoItemDataResponse {
            status: "success".to_string(),
            todo_item_data: TodoItemData {
                todo_item: todo.clone().to_dto(),
            },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_todo_item_path(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut todo_item_list = app_state.db.lock().await;
    let id = id.to_string();

    if let Some(pos) = todo_item_list
        .iter()
        .position(|todo| todo.id == Some(id.clone()))
    {
        todo_item_list.remove(pos);
        let json_response = serde_json::json!({
            "status": "success",
            "message": format!("Todo with ID: {} deleted", id)
        });

        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn get_todo_item_secret_path(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let todo_item_list = app_state.db.lock().await;
    let id = id.to_string();

    if let Some(todo_item) = todo_item_list
        .iter()
        .find(|todo| todo.id == Some(id.to_owned()))
    {
        let json_response = TodoItemDataResponseSecret {
            status: "success".to_string(),
            todo_item_data: TodoItemDataSecret {
                todo_item: todo_item.clone(),
            },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
