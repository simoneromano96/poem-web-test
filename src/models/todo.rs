//! SeaORM Entity. Generated by sea-orm-codegen 0.2.6

use async_graphql::SimpleObject;
use poem_openapi::Object;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Object)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject, Object)]
pub struct TodoDAO {
    id: i32,
    name: String,
    description: Option<String>,
}

impl From<Model> for TodoDAO {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
        }
    }
}

// impl From<Vec<Model>> for Vec<TodoDAO> {
//     fn from(model: Model) -> Vec<TodoDAO> {
//     }
// }