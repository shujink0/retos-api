use crate::dto::CreateCredit;
use crate::dto::{
    AccountReq, AccountRes, ChangePasswordReq, Meta, SkinData, UploadEvent, UploadFile,
};
use actix_multipart::form::MultipartForm;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::Duration;
use serde_json::json;
use std::io::BufReader;
use std::io::Read;
use uuid::uuid;
use uuid::Uuid;

use deadpool_postgres::Pool;

pub async fn create_event(pool: &Pool, mut form: MultipartForm<UploadEvent>) -> Result<bool> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let mut di = String::new();
    let _ = &form.decoracion_skin.file.read_to_string(&mut di)?;

    let mut rb = String::new();
    let _ = &form.icono_skin.file.read_to_string(&mut rb)?;

    let data = SkinData {
        color: format!("#{}", *form.color_skin),
        decoration_img: di,
        reaction_button: rb,
    };

    let fecha_desde = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", *form.fecha_desde),
        "%Y-%m-%d %H:%M:%S",
    )?;
    let fecha_hasta = NaiveDateTime::parse_from_str(
        &format!("{} 23:59:59", *form.fecha_hasta),
        "%Y-%m-%d %H:%M:%S",
    )?;
    let fecha_hasta = fecha_hasta
        .checked_add_signed(Duration::hours(5))
        .ok_or_else(|| anyhow!("error al convertir fecha_hasta"))?;

    let rows = transaction.query(
            "INSERT INTO post_skin (name, data, \"startAt\", \"endAt\", enabled)  VALUES ($1, $2, $3, $4, $5) RETURNING ID", 
            &[&form.nombre_reto_etiqueta.to_string(), &json!(data), &fecha_desde, &fecha_hasta, &true],
        ).await?;

    let post_skin_id: Uuid = rows[0].get(0);

    transaction.execute(
            "INSERT INTO credit_rule(item_type, item_field_key, item_field_value, action, description, amount) VALUES ($1, $2, $3, $4, $5, $6)", 
            &[&"post", &"skin", &post_skin_id.to_string(), &form.tipo_reto.to_string(), &form.nombre_reto_etiqueta.to_string(), &form.cantidad_puntos_reto.to_owned()],
        ).await?;

    let meta = Meta::default();

    let mime_type = form
        .imagen_reto
        .content_type
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let file_name = form
        .imagen_reto
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let mut reader = BufReader::new(&form.imagen_reto.file);
    let mut imagen_reto_bytes = Vec::new();
    reader.read_to_end(&mut imagen_reto_bytes)?;

    let rows = transaction.query(
            "INSERT INTO file(filename, encoding, mimetype, data) VALUES ($1, $2, $3, $4) RETURNING ID", 
            &[&file_name, &"7bit".to_string(), &mime_type, &imagen_reto_bytes],
        ).await?;

    let file_id: Uuid = rows[0].get(0);

    let rows = transaction
        .query(
            "INSERT INTO attachment(file, account, enabled) VALUES ($1, $2, $3) RETURNING ID",
            &[
                &file_id,
                &uuid!("471e366a-293c-11ed-957b-eb3c8f5a17f5"),
                &true,
            ],
        )
        .await?;

    let attachment_id: Uuid = rows[0].get(0);

    let rows = transaction.query(
            "INSERT INTO event (name, \"start_at\", \"end_at\", skin, enabled, meta)  VALUES ($1, $2, $3, $4, $5, $6) RETURNING ID", 
            &[&form.nombre_reto_etiqueta.to_string(), &fecha_desde, &fecha_hasta, &post_skin_id, &true, &json!(meta)],
        ).await?;

    let event_id: Uuid = rows[0].get(0);

    transaction
        .execute(
            "INSERT INTO event_attachment (event_id, attachment_id)  VALUES ($1, $2)",
            &[&event_id, &attachment_id],
        )
        .await?;

    transaction.commit().await?;

    Ok(true)
}

pub async fn update_file(pool: &Pool, form: MultipartForm<UploadFile>) -> Result<bool> {
    let client = pool.get().await?;

    let mime_type = form
        .imagen
        .content_type
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let file_name = form
        .imagen
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let mut reader = BufReader::new(&form.imagen.file);
    let mut imagen_reto_bytes = Vec::new();
    reader.read_to_end(&mut imagen_reto_bytes)?;

    client
        .execute(
            "UPDATE file SET mimetype=$1, data=$2, filename=$3 WHERE id=$4",
            &[
                &mime_type,
                &imagen_reto_bytes,
                &file_name,
                &Uuid::parse_str(&form.id.to_string())?,
            ],
        )
        .await?;

    Ok(true)
}

pub async fn create_credit(pool: &Pool, body: CreateCredit) -> Result<bool> {
    let client = pool.get().await?;

    client.execute(
            "INSERT INTO credit(account, description, amount, subject_action) VALUES ($1, $2, $3, $4)", 
            &[&Uuid::parse_str(&body.account)?, &body.description, &body.amount, &"post"],
        ).await?;

    Ok(true)
}

pub async fn get_users(pool: &Pool, body: AccountReq) -> Result<Vec<AccountRes>> {
    let client = pool.get().await?;
    let mut result: Vec<AccountRes> = vec![];

    for row in client
        .query(
            "SELECT id, name, email, last_login FROM account WHERE LOWER(name) LIKE LOWER($1)",
            &[&format!("%{}%", body.name_like)],
        )
        .await?
    {
        let mut last_l = "".to_string();

        let _id: Uuid = row.get(0);
        let ll: Option<NaiveDateTime> = row.get(3);

        if let Some(_last_login) = ll {
            last_l = _last_login.to_string();
        }

        let acc = AccountRes {
            id: _id.to_string(),
            name: row.get(1),
            email: row.get(2),
            last_login: last_l,
        };

        result.push(acc);
    }

    Ok(result)
}

pub async fn change_password(pool: &Pool, body: &ChangePasswordReq) -> Result<Uuid> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let query_one = transaction
        .query(
            "SELECT start_password_recovery_text($1)",
            &[&body.email_data.email],
        )
        .await?;

    let recovery_hash: String = query_one[0].get(0);

    let query_two = transaction
        .query(
            "SELECT do_password_recovery_text($1,$2,$3)",
            &[&body.email_data.email, &recovery_hash, &body.password],
        )
        .await?;

    let result: Uuid = query_two[0].get(0);

    transaction.commit().await?;

    Ok(result)
}
