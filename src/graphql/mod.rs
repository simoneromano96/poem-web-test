pub mod todo;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use self::todo::QueryRoot;

// GraphQL API
pub type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
