use crate::config::Config;
use crate::resp::{
    ApiResponse, CreditInfo, CursorPageData, CursorParams, EnterpriseDetail, EnterpriseSummary,
};
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder, get, web};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[get("/enterprises")]
pub async fn list_enterprises(
    pool: Data<PgPool>,
    query: web::Query<CursorParams>,
) -> impl Responder {
    let params = query.into_inner();
    let limit = params.limit();
    let cursor = params.cursor;

    let total: i64 = sqlx::query("SELECT COUNT(*) FROM enterprises")
        .fetch_one(pool.get_ref())
        .await
        .map(|row| row.get(0))
        .unwrap_or(0);

    let rows = if let Some(c) = cursor {
        sqlx::query(
			"SELECT id, uuid, name, credit_score, tier, industry FROM enterprises WHERE id > $1 ORDER BY id LIMIT $2"
		)
			.bind(c)
			.bind(limit)
			.fetch_all(pool.get_ref())
			.await
			.unwrap_or_default()
    } else {
        sqlx::query(
			"SELECT id, uuid, name, credit_score, tier, industry FROM enterprises ORDER BY id LIMIT $1"
		)
			.bind(limit)
			.fetch_all(pool.get_ref())
			.await
			.unwrap_or_default()
    };

    let items: Vec<EnterpriseSummary> = rows
        .iter()
        .map(|row| EnterpriseSummary {
            uuid: row.get("uuid"),
            name: row.get("name"),
            credit_score: row.get("credit_score"),
            tier: row.get("tier"),
            industry: row.get("industry"),
        })
        .collect();

    let next_cursor = rows.last().map(|row| row.get::<i64, _>("id"));
    let page_data = CursorPageData::new(items, total, limit, next_cursor);

    ApiResponse::success(page_data).respond()
}

#[get("/enterprises/{uuid}")]
pub async fn get_enterprise(pool: Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let uuis = path.into_inner();
    let row = sqlx::query(
        r#"SELECT uuid, name, credit_score, tier, industry, unified_social_credit_code, bio,
        logo_url, is_verified
        FROM enterprises
        WHERE uuid = $1;"#,
    )
    .bind(uuis)
    .fetch_optional(pool.get_ref())
    .await;

    match row {
        Ok(Some(row)) => {
            let detail = EnterpriseDetail {
                uuid: row.get("uuid"),
                name: row.get("name"),
                credit_score: row.get("credit_score"),
                tier: row.get("tier"),
                industry: row.get("industry"),
                unified_social_credit_code: row.get("unified_social_credit_code"),
                bio: row.get("bio"),
                logo_url: row.get("logo_url"),
                is_verified: row.get("is_verified"),
            };
            ApiResponse::success(detail).respond()
        }
        Ok(None) => ApiResponse::<()>::error(3002, "企业不存在").respond(),
        Err(e) => ApiResponse::<()>::error(
            4001,
            format!("查询失败 {} ,请将此错误报告给服务器管理员", e),
        )
        .respond(),
    }
}

#[get("/server/meta")]
pub async fn server_meta(conf: Data<Config>) -> impl Responder {
    let meta = conf.get_ref();
    if let Some(meta) = meta.meta.clone()
        && meta.serve_meta_endpoint
    {
        ApiResponse::success(meta).respond()
    } else {
        ApiResponse::<()>::error(10404, "Sever NOT serve own Meta Data").respond()
    }
}
#[get("/enterprises/{uuid}/credit")]
pub async fn get_enterprise_credit(pool: Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let uuid = path.into_inner();

    let row = sqlx::query(
        "SELECT name, credit_score FROM enterprises WHERE uuid = $1 AND public_visible = true",
    )
    .bind(uuid)
    .fetch_optional(pool.get_ref())
    .await;

    match row {
        Ok(Some(row)) => {
            let credit_info = CreditInfo {
                name: row.get("name"),
                credit_score: row.get("credit_score"),
                level: match row.get("credit_score") {
                    80..=100 => "优秀",
                    60..=79 => "良好",
                    40..=59 => "一般",
                    _ => "待提升",
                },
            };
            ApiResponse::success(credit_info).respond()
        }
        Ok(None) => ApiResponse::<()>::error(3002, "企业不存在或未公开").respond(),
        Err(_) => ApiResponse::<()>::error(4001, "查询失败").respond(),
    }
}
