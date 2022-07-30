use actix_web::{get, HttpResponse, Responder, web};
use crate::config::database::DbPool;
use crate::service::ping_service::testok;
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

#[get("/testhello")]
pub async fn testhello(pool: web::Data<DbPool>) -> impl Responder {
    use diesel_demo::schema::posts::dsl::*;
    let conn = pool.get().unwrap();
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&conn)
    .expect("Error loading posts");
    println!("Displaying {:?} posts", results.len());
    // for post in results {
    //     println!("{:?}", post);
    //     println!("{:?}", post.title);
    // }
    testok(&pool).await;
    HttpResponse::Ok().body(format!("Hello world! Succesfully connected to Database! Query Results: {}", "testad"))
}