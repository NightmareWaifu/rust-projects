use actix_web::{HttpServer, App, web, HttpResponse, Responder};

async fn hello_world() -> impl Responder{
    HttpResponse::Ok().body("<h1>Test Application</h1>")
}

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    const PORT: &str = "3000";
    let address: String = format!("127.0.0.1:{}",PORT);
    println!("App running at http://localhost:{}",PORT);
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(hello_world))
    })
    .bind(address)?
    .run()
    .await
}
