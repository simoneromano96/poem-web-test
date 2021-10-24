pub mod todo;

use async_graphql::{extensions::Tracing, EmptySubscription, Schema};

use self::todo::{MutationRoot, QueryRoot};

// GraphQL API
pub type GraphQlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> GraphQlSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(Tracing)
        .finish()
}
