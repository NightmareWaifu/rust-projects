use std::fmt::format;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

pub mod schema;
pub mod models;
use actix_web::{HttpServer, App, web, HttpResponse, Responder, cookie::Cookie};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};


// pub mod models;

use models::{
    User,
    NewUser,
    LoginUser
};



//all models are moved into models directory
#[derive(Serialize)]
struct Post{
    title: String,
    link: String,
    author: String
}

// #[derive(Debug, Deserialize)]
// struct LoginUser{
//     username: String,
//     email: String,
//     password: String
// }



#[derive(Debug, Deserialize)]
struct Submission{
    title: String,
    link: String
}

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    const PORT: &str = "3000";
    let address: String = format!("127.0.0.1:{}",PORT);
    println!("App running at http://localhost:{}",PORT);
    HttpServer::new(|| {

        let tera = Tera::new("templates/**/*").unwrap(); // "/**/*" means to get all the files inside of the templates folder no matter how many sub dirs there are

        App::new()
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0;32])
            .name("auth-cookie")
            .secure(false) //false - HTTP, true - HTTPS
        )
        )
        .app_data(web::Data::new(tera.clone())) //app_data() doesnt work for some reason
        //add all routes below
        .route("/", web::get().to(index))
        .route("/signup", web::get().to(get_signup))
        .route("/signup", web::post().to(post_signup))
        .route("/login", web::get().to(get_login))
        .route("/login", web::post().to(post_login))
        .route("/submission", web::get().to(get_submission))
        .route("/submission", web::post().to(post_submission))
        .route("/logout", web::to(logout))
    })
    .bind(address)?
    .run()
    .await
}

async fn index(tera: web::Data<Tera>) -> impl Responder{
    let mut data = Context::new();
    
    let posts = [
        Post{
            title: String::from("This is the first link"),
            link: String::from("https://1example.com"),
            author: String::from("Bobbers")
        },
        Post{
            title: String::from("This is the second link"),
            link: String::from("https://2example.com"),
            author: String::from("Sloobby")
        }
    ];

    data.insert("title", "Web App using Tera");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn get_signup(tera: web::Data<Tera>) -> impl Responder{
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn post_signup(data: web::Form<NewUser>) -> impl Responder{
    use schema::users;

    let mut connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&mut connection)
        .expect("Failed to register user");


    println!("{:?}",data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}",data.username))
}

async fn get_login(tera: web::Data<Tera>, id: Identity) -> impl Responder{
    let mut data = Context::new();
    data.insert("title", "Login");

    if let Some(id) = id.identity(){
        return HttpResponse::Ok().body("Already logged in.")
    }
    
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn post_login(data: web::Form<LoginUser>, id: Identity) -> impl Responder{
    use schema::users::dsl::{username, users};

    let mut connection = establish_connection();
    let user = users.filter(username.eq(&data.username)).first::<User>(&mut connection);

    match user{
        Ok(u) =>{
            if u.password == data.password{
                let session_token = String::from(u.username);
                id.remember(session_token);
                HttpResponse::Ok().body(format!("Logged in as: {}", data.username))
            } else{ 
                HttpResponse::Ok().body("Password is incorrect.")
            }
        }
        Err(e) =>{
            println!("{:?}", e);
            HttpResponse::Ok().body(format!("User: {} does not exist.", data.username))
        }
    }
        
}

async fn get_submission(tera: web::Data<Tera>) -> impl Responder{
    let mut data = Context::new();
    data.insert("title", "Submit a post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn post_submission(data: web::Form<Submission>) -> impl Responder{
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}

fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
    .expect(&format!("Error connection to {}", database_url))
}

async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}