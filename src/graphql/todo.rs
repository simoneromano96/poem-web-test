use async_graphql::{Error, Object, Result};
use tracing::debug;

use crate::models::todo::{get_todos, TodoDAO};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> String {
        String::from("pong")
    }

    async fn todos(&self) -> Result<Vec<TodoDAO>> {
        debug!("GraphQL todos handler");
        match get_todos().await {
            Ok(todos) => Ok(todos),
            Err(error) => Err(Error::from(format!("{}", error))),
        }
    }
}
