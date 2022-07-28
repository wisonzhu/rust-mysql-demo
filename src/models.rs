use super::schema::posts;
use serde::{Serialize, Deserialize};

#[derive(Queryable,Debug,Serialize,Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

table! {
    sequences(id) {
        id -> BigInt,
    }
}

// 用于获取id
#[derive(QueryableByName)]
#[table_name="sequences"]
pub struct Sequence {
    pub id: i64,
}