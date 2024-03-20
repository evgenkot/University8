use crate::model::{TodoItem, TodoItemDTO};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericRsponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TodoItemData {
    pub todo_item: TodoItemDTO,
}

#[derive(Serialize, Debug)]
pub struct TodoItemDataResponse {
    pub status: String,
    pub todo_item_data: TodoItemData,
}

#[derive(Serialize, Debug)]

pub struct TodoItemDataSecret {
    pub todo_item: TodoItem,
}

#[derive(Serialize, Debug)]
pub struct TodoItemDataResponseSecret {
    pub status: String,
    pub todo_item_data: TodoItemDataSecret,
}

#[derive(Serialize, Debug)]
pub struct TodoItemDataListResponse {
    pub status: String,
    pub page: usize,
    pub result_count: usize,
    pub todo_item_list: Vec<TodoItemDTO>,
}
