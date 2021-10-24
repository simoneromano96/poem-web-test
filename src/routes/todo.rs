use poem_openapi::{
  payload::{Json, PlainText},
  ApiResponse, OpenApi, Tags,
};
use tracing::debug;

use crate::models::todo::{self, get_todos};

// REST API

#[derive(Tags)]
enum ApiTags {
    /// Operations about todo
    Todo,
}

#[derive(ApiResponse)]
enum GetTodosResponse {
    /// Got all todos.
    #[oai(status = 200)]
    Ok(Json<Vec<todo::Model>>),
    /// Had database error.
    #[oai(status = 500)]
    DbError,
}

#[derive(ApiResponse)]
enum GetTodoResponse {
    /// Got all todos.
    #[oai(status = 200)]
    Ok(Json<todo::Model>),
    /// Had database error.
    #[oai(status = 500)]
    DbError,
}

pub struct TodoAPI;

#[OpenApi]
impl TodoAPI {
    #[oai(path = "/ping", method = "get")]
    async fn index(&self) -> PlainText<String> {
        debug!("REST ping handler");
        PlainText(String::from("pong"))
    }
    #[oai(path = "/todo", method = "get", tag = "ApiTags::Todo")]
    async fn todos(&self) -> GetTodosResponse {
        debug!("REST todos handler");
        match get_todos().await {
            Ok(todos) => GetTodosResponse::Ok(Json(todos)),
            Err(_error) => GetTodosResponse::DbError,
        }
    }
    #[oai(path = "/todo-test", method = "get", tag = "ApiTags::Todo")]
    async fn todo(&self) -> GetTodoResponse {
        debug!("REST todos handler");
        GetTodoResponse::Ok(Json(todo::Model {
            id: 1,
            name: "test".to_string(),
            description: None,
        }))
    }
}
