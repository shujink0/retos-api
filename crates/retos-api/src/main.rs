use actix_web::HttpServer;
use actix_web::{web, web::scope, App};
use anyhow::Result;
use dotenv::dotenv;

mod db;
mod dto;
mod routes;

use common::{cors1, cors2, cors_headers, TokenMiddleware};
use routes::{
    change_password_route, create_credit_route, create_event_route, get_users_route,
    update_file_route,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let server_address = common::get_server_address();
    let environment = common::get_env();
    let db_pool = common::create_pool(environment).await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .wrap(TokenMiddleware)
            .wrap(cors_headers())
            .service(
                scope("/retos-api")
                    .service(cors1)
                    .service(cors2)
                    .service(
                        web::resource("/create_event").route(web::post().to(create_event_route)),
                    )
                    .service(web::resource("/update_file").route(web::post().to(update_file_route)))
                    .service(
                        web::resource("/create_credit").route(web::post().to(create_credit_route)),
                    )
                    .service(web::resource("/get_users").route(web::get().to(get_users_route)))
                    .service(
                        web::resource("/change_password")
                            .route(web::post().to(change_password_route)),
                    ),
            )
    })
    .bind(server_address)?
    .run();
    println!("Server running at http://{}/", server_address);

    server.await?;
    Ok(())
}
