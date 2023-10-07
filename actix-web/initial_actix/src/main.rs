#![feature(proc_macro_hygiene)]

pub mod views;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use render::{rsx, Render};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
async fn test() -> impl Responder {
    //test_html.render();
    HttpResponse::Ok().body(views::views::test_html())
}

#[get("/route")]
async fn my_route() -> impl Responder {
    HttpResponse::Ok().body("This is my created route!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


//manual call instead of using #get or #post
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 3000;
    HttpServer::new(|| {
        App::new() //add your routes here
            .service(hello)
            .service(echo)
            .service(test)
            .service(my_route)
            .route("/hey", web::get().to(manual_hello)) //instead of using get, you can force call the path using .route, where url is "/hey" and it calls the "manual_hello()" function
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}