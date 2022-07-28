use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

#[get("/")]
async fn hello() -> impl Responder {

    use diesel_demo::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

    // println!("Displaying {:?} posts", results.len());
    // for post in results {
    //     println!("{:?}", post);
    //     println!("{:?}", post.title);
    // }
        HttpResponse::Ok().json(results)
    }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}