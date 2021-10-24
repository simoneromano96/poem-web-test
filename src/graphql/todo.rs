use async_graphql::{Error, Object, Result};
use tracing::{debug, error};

use crate::models::todo::{self, CreateTodoInput, TodoError, UpdateTodoInput};

pub struct QueryRoot;

fn handle_error(error: TodoError) -> Error {
	error!("{}", error);
	Error::new(format!("{:?}", error))
}

#[Object]
impl QueryRoot {
	async fn todos(&self) -> Result<Vec<todo::Model>> {
		debug!("GraphQL find all todos handler");

		todo::Model::find_all().await.map_err(handle_error)
	}
	async fn todo(&self, id: String) -> Result<todo::Model> {
		debug!("GraphQL find todo handler");

		todo::Model::find_one(id).await.map_err(handle_error)
	}
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
	async fn create_todo(&self, input: CreateTodoInput) -> Result<todo::Model> {
		debug!("GraphQL create todo handler");

		todo::Model::create_one(input).await.map_err(handle_error)
	}

	async fn update_todo(&self, id: String, input: UpdateTodoInput) -> Result<todo::Model> {
		debug!("GraphQL update todo handler");

		todo::Model::update_one(id, input)
			.await
			.map_err(handle_error)
	}
  async fn delete_todo(&self, id: String) -> Result<u64> {
		debug!("GraphQL delete todo handler");

		todo::Model::delete_one(id).await.map_err(handle_error)
	}
}
