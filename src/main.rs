//#![windows_subsystem = "windows"]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
extern crate web_view;

use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::config::{Config, Environment};
use rust_embed::RustEmbed;

use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;
use std::{thread, time};

use web_view::{Content};

mod utils;
use utils::get_rnd_port;

#[derive(RustEmbed)]
#[folder = "gui\\dist"]
struct Asset;

#[get("/")]
fn index<'r>() -> response::Result<'r> {
  Asset::get("index.html").map_or_else(
    || Err(Status::NotFound),
    |d| response::Response::build().header(ContentType::HTML).sized_body(Cursor::new(d)).ok(),
  )
}

#[get("/<file..>")]
fn dist<'r>(file: PathBuf) -> response::Result<'r> {
  let filename = file.display().to_string();
  Asset::get(&filename).map_or_else(
    || Err(Status::NotFound),
    |d| {
      let ext = file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
      let content_type = ContentType::from_extension(ext).ok_or_else(|| Status::new(400, "Could not get file content type"))?;
      response::Response::build().header(content_type).sized_body(Cursor::new(d)).ok()
    },
  )
}

fn main() {

  #[cfg(debug_assertions)]
  let mut port = 8080;

  #[cfg(not(debug_assertions))]
  let mut port = 60123;

  #[cfg(not(debug_assertions))]
  let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port));

  #[cfg(not(debug_assertions))]
  match listener {
    Err(_e) => port = get_rnd_port(),
    Ok(v) => println!("error: {:?}", v )
  }

  #[cfg(not(debug_assertions))]
  thread::spawn(move || {
    let config = Config::build(Environment::Development)
      .address("localhost")
      .port(port)
      .finalize();
  
    let config: Option<rocket::config::Config> = match config {
      Ok(r) => Some(r),
      Err(e) => { 
        println!("Error here: {:?}", e);
        None
      }
    };
  
    rocket::custom(config.unwrap()).mount("/", routes![index, dist]).launch();
  });

  let webview = web_view::builder()
  .title("Promodoro")
  .content(Content::Url(format!("http://localhost:{}/", port)))
  .size(400,600)
  .resizable(true)
  .min_size(400, 600)
  .user_data(())
  .invoke_handler(|_webview, _arg| {
    Ok(())
  })
  .build().unwrap();

  webview.run().unwrap();




}