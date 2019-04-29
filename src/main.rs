#[macro_use]
extern crate json;

use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};

/// This handler manually load request payload and parse json-rust
fn index_mjsonrust(pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(|body| {
        // body is loaded, now we can deserialize json-rust
        let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        let myjson = injson.dump();
        println!("{}", myjson);
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(myjson))
    })
}


fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::post().to_async(index_mjsonrust)),
            )
    })
    .bind("0.0.0.0:8181")?
    .run()
}