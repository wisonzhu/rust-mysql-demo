use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate diesel_demo;
extern crate diesel;


use self::diesel_demo::*;
use self::diesel::prelude::*;
use self::models::{NewPost, Post};

use serde::{Serialize,Deserialize};

use crate::config::database::DbPool;
pub mod config;
pub mod route;
pub mod service;

#[derive(Deserialize,Serialize)]
struct UserPost {
    title: String,
     body: String
}


use log::info;

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}


#[get("/")]
async fn hello() -> impl Responder {
    info!("this is test");
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


#[get("/test")]
async fn testpool(pool: web::Data<DbPool>) -> impl Responder {
    info!("this is test");
    use diesel_demo::schema::posts::dsl::*;
    let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&pool.get().unwrap())
    .expect("Error loading posts");
    HttpResponse::Ok().json(results)
    
}    

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



#[post("/data")]
async fn data1(jsondata: web::Json<UserPost>) -> impl Responder {
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
    init_logger();
    info!("hello world");
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let pool = config::database::establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .service(data1)
            .service(testpool)
            .service(route::test_route::testhello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}