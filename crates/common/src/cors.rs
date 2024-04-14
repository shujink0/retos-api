use actix_web::{middleware, options, HttpResponse, Responder};

pub fn cors_headers() -> middleware::DefaultHeaders {
    middleware::DefaultHeaders::new()
        .add(("Access-Control-Allow-Origin", "*"))
        .add(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .add((
            "Access-Control-Allow-Headers",
            "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range",
        ))
        .add((
            "Access-Control-Expose-Headers",
            "Content-Length,Content-Range",
        ))
}

#[options("/update_file")]
async fn cors1() -> impl Responder {
    HttpResponse::Ok()
}

#[options("/change_password")]
async fn cors2() -> impl Responder {
    HttpResponse::Ok()
}
