use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::diesel::prelude::*;
use self::models::{NewPost, Post};

use serde::{Serialize,Deserialize};

#[derive(Deserialize,Serialize)]
struct UserPost {
    title: String,
     body: String
}

#[get("/")]
async fn hello() -> impl Responder {

    use diesel_demo::schema::posts::dsl::*;
    let conn = establish_connection();
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&conn)
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



#[post("/data")]
async fn data(jsondata: web::Json<UserPost>) -> impl Responder {
    let conn = establish_connection();
    use schema::posts::dsl::posts;
    let str1 = jsondata.title.as_str();
    let str2 = jsondata.body.as_str();
    let new_post = NewPost{title: str1 ,body: str2};
    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");

    HttpResponse::Ok().json(jsondata)
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
            .service(data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}