use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

// Schema definition
mod schema {
    diesel::table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
            email -> Varchar,
        }
    }
}

use schema::users;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// User model
#[derive(Queryable, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user")]
async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    let pool= pool.into_inner();
    match pool.get() {
        Ok(mut con ) => {
            match diesel::insert_into(users::table)
                .values(&new_user.into_inner())
                .get_result::<User>(&mut con)
            {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    HttpResponse::InternalServerError().body("Could not create user")
                }
            }
        }
        Err(e) => {
            eprintln!("Connection error: {:?}", e);
            HttpResponse::InternalServerError().body("Database connection error")
        }
    }
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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create db connection pool
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(create_user)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
