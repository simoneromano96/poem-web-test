use poem_openapi::{
	payload::{Json, PlainText},
	ApiResponse, OpenApi, Tags,
};
use tracing::{debug, error};

use crate::models::todo::{self, CreateTodoInput, TodoError};

// REST API

#[derive(Tags)]
enum ApiTags {
	/// Operations about todo
	Todo,
}

#[derive(ApiResponse)]
enum CreateTodoResponse {
	/// Created todo.
	#[oai(status = 201)]
	Ok(Json<todo::Model>),
	/// Had database error.
	#[oai(status = 500)]
	DbError,
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
	/// Could not find todo.
	#[oai(status = 404)]
	NotFound,
	/// Had database error.
	#[oai(status = 500)]
	DbError,
}

#[derive(ApiResponse)]
enum UpdateTodoResponse {
	/// Updated todo.
	#[oai(status = 200)]
	Ok(Json<todo::Model>),
	/// Could not find todo.
	#[oai(status = 404)]
	NotFound,
	/// Had database error.
	#[oai(status = 500)]
	DbError,
}

#[derive(ApiResponse)]
enum DeleteTodoResponse {
	/// Deleted.
	#[oai(status = 204)]
	Ok,
	/// Could not find todo.
	#[oai(status = 404)]
	NotFound,
	/// Had database error.
	#[oai(status = 500)]
	DbError,
}

pub struct TodoAPI;

#[OpenApi]
impl TodoAPI {
	// #[oai(path = "/ping", method = "get")]
	// async fn index(&self) -> PlainText<String> {
	//     debug!("REST ping handler");
	//     PlainText(String::from("pong"))
	// }
	#[oai(path = "/todo", method = "post", tag = "ApiTags::Todo")]
	async fn create_todo(&self, todo: Json<CreateTodoInput>) -> CreateTodoResponse {
		debug!("REST create todo handler");

		match todo::Model::create_one(todo.0).await {
			Ok(todo) => CreateTodoResponse::Ok(Json(todo)),
			Err(err) => {
				error!("{}", err);
				CreateTodoResponse::DbError
			}
		}
	}
	#[oai(path = "/todo", method = "get", tag = "ApiTags::Todo")]
	async fn find_all_todos(&self) -> GetTodosResponse {
		debug!("REST find all todos handler");

		match todo::Model::find_all().await {
			Ok(todos) => GetTodosResponse::Ok(Json(todos)),
			Err(_err) => GetTodosResponse::DbError,
		}
	}
	#[oai(path = "/todo/:id", method = "get", tag = "ApiTags::Todo")]
	async fn find_todo(&self, #[oai(name = "id", in = "path")] id: String) -> GetTodoResponse {
		debug!("REST find todo handler");

		match todo::Model::find_one(id).await {
			Ok(todo) => GetTodoResponse::Ok(Json(todo)),
			Err(err) => match err {
				TodoError::NotFound(_id) => GetTodoResponse::NotFound,
				err => {
					error!("{}", err);
					GetTodoResponse::DbError
				}
			},
		}
	}
	#[oai(path = "/todo/:id", method = "patch", tag = "ApiTags::Todo")]
	async fn update_todo(
		&self,
		#[oai(name = "id", in = "path")] id: String,
		todo: Json<todo::UpdateTodoInput>,
	) -> UpdateTodoResponse {
		debug!("REST update todo handler");

		match todo::Model::update_one(id, todo.0).await {
			Ok(todo) => UpdateTodoResponse::Ok(Json(todo)),
			Err(err) => match err {
				TodoError::NotFound(_id) => UpdateTodoResponse::NotFound,
				err => {
					error!("{}", err);
					UpdateTodoResponse::DbError
				}
			},
		}
	}
	#[oai(path = "/todo/:id", method = "delete", tag = "ApiTags::Todo")]
	async fn delete_todo(&self, #[oai(name = "id", in = "path")] id: String) -> DeleteTodoResponse {
		debug!("REST delete todo handler");

		match todo::Model::delete_one(id).await {
			Ok(_) => DeleteTodoResponse::Ok,
			Err(err) => match err {
				TodoError::NotFound(_id) => DeleteTodoResponse::NotFound,
				err => {
					error!("{}", err);
					DeleteTodoResponse::DbError
				}
			},
		}
	}
}
