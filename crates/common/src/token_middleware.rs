use anyhow::Result;

use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixWebError,
};

use futures_util::future::LocalBoxFuture;

use crate::AuthError;
use crate::TOKEN;

pub struct TokenMiddleware;

impl<S: 'static, B> Transform<S, ServiceRequest> for TokenMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = TokenMiddleware1<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenMiddleware1 {
            service: Rc::new(service),
        }))
    }
}

pub struct TokenMiddleware1<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TokenMiddleware1<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            if !req.headers().contains_key("token") {
                return Err(AuthError::NoToken.into());
            }

            let value = req
                .headers()
                .get("token")
                .ok_or(AuthError::TokenValidation)?
                .as_bytes();
            let token = std::str::from_utf8(value).expect("no se pudo decodificar el token");

            if token != TOKEN {
                return Err(AuthError::TokenValidation.into());
            }

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
