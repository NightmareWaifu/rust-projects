//mod = modules
pub mod api; //looks for api.rs if not found -> api/mod.rs
pub mod repository;
pub mod model;

use api::task::{
    get_task,
    submit_task,
    start_task,
    complete_task,
    pause_task,
    fail_task,
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger, http::uri::Port};
use repository::ddb::DDBRepository;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug");
    std::env::set_var("RUST_BACKTRACE","1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    
    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"),
            config.clone()
        );
        let ddb_data = Data::new(
            ddb_repo
        );
        const ADDRESS: &str = "127.0.0.1";
        const PORT: u32 = 3000;
        let base_url = (ADDRESS,PORT);
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .app_data(ddb_data)
        .service(get_task)
        .service(submit_task)
        .service(start_task)
        .service(complete_task)
        .service(pause_task)
        .service(fail_task)
    })
    .bind(("127.0.0.1",3000))?
    .run()
    .await
}
