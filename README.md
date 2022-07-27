cargo new --lib diesel_demo
cd diesel_demo

[dependencies]
diesel = { version = "1.4.4", features = ["postgres"] }
dotenv = "0.15.0"


diesel migration generate create_posts

diesel migration redo
  
cargo run --bin show_posts
cargo run --bin write_post

 cargo run --bin publish_post 1
 
  cargo run --bin delete_post demo
