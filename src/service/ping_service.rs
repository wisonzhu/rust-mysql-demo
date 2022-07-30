use actix_web::{web};
use diesel::RunQueryDsl;
use crate::config::database::DbPool;
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;


pub async fn testok(pool: &web::Data<DbPool>){
    use diesel_demo::schema::posts::dsl::*;
    let conn = pool.get().unwrap();
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&conn)
    .expect("Error loading posts");
    println!("{}",results.len());
    let generated_id = diesel::sql_query("select LAST_INSERT_ID() as id")
    .load::<Sequence>(&conn).expect("get_id_error").first().unwrap().id;
    println!("this is ok {:?}",generated_id)

}