use actix_multipart::form::MultipartForm;
use actix_web::web;
use actix_web::HttpResponse;
use anyhow::Result;
use common::CustomError;

use crate::db::{change_password, create_credit, create_event, get_users, update_file};
use crate::dto::{AccountReq, ChangePasswordReq, CreateCredit, EmailData, UploadEvent, UploadFile};

use deadpool_postgres::Pool;

pub async fn create_event_route(
    pool: web::Data<Pool>,
    form: MultipartForm<UploadEvent>,
) -> Result<HttpResponse, CustomError> {
    let result = create_event(pool.get_ref(), form).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_file_route(
    pool: web::Data<Pool>,
    form: MultipartForm<UploadFile>,
) -> Result<HttpResponse, CustomError> {
    let result = update_file(pool.get_ref(), form).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn create_credit_route(
    pool: web::Data<Pool>,
    body: web::Json<CreateCredit>,
) -> Result<HttpResponse, CustomError> {
    let inner_body = body.into_inner();
    let result = create_credit(pool.get_ref(), inner_body).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_users_route(
    pool: web::Data<Pool>,
    query: web::Query<AccountReq>,
) -> Result<HttpResponse, CustomError> {
    let inner_query = query.into_inner();
    let result = get_users(pool.get_ref(), inner_query).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn change_password_route(
    pool: web::Data<Pool>,
    body: web::Json<ChangePasswordReq>,
) -> Result<HttpResponse, CustomError> {
    let inner_body = body.into_inner();
    let result = change_password(pool.get_ref(), &inner_body).await?;

    let EmailData {
        email,
        message,
        subject,
    } = inner_body.email_data;

    if !result.is_nil() {
        common::send_email(email, subject, message).await?;
    }

    Ok(HttpResponse::Ok().json(result.to_string()))
}
