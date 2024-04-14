use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ChangePasswordReq {
    pub password: String,
    pub email_data: EmailData,
}

#[derive(Deserialize, Debug)]
pub struct EmailData {
    pub message: String,
    pub subject: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountReq {
    #[serde(rename = "nameLike")]
    pub name_like: String,
}

#[derive(Serialize, Debug)]
pub struct AccountRes {
    pub id: String,
    pub name: String,
    pub email: String,
    pub last_login: String,
}

#[derive(Deserialize)]
pub struct CreateCredit {
    pub amount: i32,
    pub description: String,
    pub account: String,
}

#[derive(Serialize, Default)]
pub struct Meta<'a> {
    pub location: &'a str,
    pub description: &'a str,
    pub external_url: &'a str,
    pub call_to_action: &'a str,
}

#[derive(Serialize)]
pub struct SkinData {
    pub color: String,
    #[serde(rename = "decorationImg")]
    pub decoration_img: String,
    #[serde(rename = "reactionButton")]
    pub reaction_button: String,
}

#[derive(MultipartForm)]
pub struct UploadEvent {
    pub nombre_reto_etiqueta: Text<String>,
    pub decoracion_skin: TempFile,
    pub icono_skin: TempFile,
    pub imagen_reto: TempFile,
    pub color_skin: Text<String>,
    pub fecha_desde: Text<String>,
    pub fecha_hasta: Text<String>,
    pub tipo_reto: Text<String>,
    pub cantidad_puntos_reto: Text<i32>,
}

#[derive(MultipartForm)]
pub struct UploadFile {
    pub imagen: TempFile,
    pub id: Text<String>,
}
