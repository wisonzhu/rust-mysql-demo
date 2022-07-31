use actix_web::{web};
use diesel::RunQueryDsl;
use diesel::update;
use diesel_demo::schema::posts::published;
use crate::config::database::DbPool;
use diesel::result::Error;
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;


pub async fn testok(pool: &web::Data<DbPool>)->Result<Vec<Post>,Error>{
    use diesel_demo::schema::posts::dsl::*;
    let conn = pool.get().unwrap();
    let generated_id = diesel::sql_query("select LAST_INSERT_ID() as id")
    .load::<Sequence>(&conn).expect("get_id_error").first().unwrap().id;
    println!("this is ok {:?}",generated_id);
    let results = posts.filter(published.eq(true))
    .limit(10)
    .load::<Post>(&conn)
    .expect("Error loading posts");
    Ok(results)
}

pub async fn updatedemo(pool: &web::Data<DbPool>){
    use diesel_demo::schema::posts::dsl::*;
    let conn = pool.get().unwrap();
    let urow = diesel::update(posts.find(16))
    .set(published.eq(true))
    .execute(&conn).unwrap();
    println!("test 123,{:?}",urow);
}