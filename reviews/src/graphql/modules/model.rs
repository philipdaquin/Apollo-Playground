use crate::schema::review;
use super::schema::{ReviewType, User, Product};
use async_graphql::*;
#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "review"]
pub struct Review { 
    pub id: i32, 
    pub body: String,
    pub author_id: i32, 
    pub product_id: i32
}

impl From<&Review> for ReviewType { 
    fn from(g: &Review) -> Self {
        ReviewType { 
            id: g.id.into(),
            body: g.body.clone(),
            author: User::new(g.author_id.clone()),
            product: Product::new(g.product_id.clone()),
        }
    }
}

impl User { 
    fn new(id: i32) -> Self { 
        Self { 
            id: id.into()
        }
    }
}
impl Product { 
    fn new(id: i32) -> Self { 
        Self { 
            id: id.into()
        }
    }
}