use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------- 统一响应 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_empty() -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: None,
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn respond(self) -> actix_web::HttpResponse {
        let status = if self.code == 0 {
            actix_web::http::StatusCode::OK
        } else {
            match self.code {
                1000..=1999 => actix_web::http::StatusCode::BAD_REQUEST,
                2000..=2999 => actix_web::http::StatusCode::UNAUTHORIZED,
                3000..=3999 => actix_web::http::StatusCode::FORBIDDEN,
                4000..=4999 => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            }
        };
        actix_web::HttpResponse::build(status).json(self)
    }
}

// ---------- 分页 ----------
#[derive(Debug, Deserialize, Default)]
pub struct CursorParams {
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

impl CursorParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }
}

#[derive(Debug, Serialize)]
pub struct CursorPageData<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub next_cursor: Option<i64>,
    pub has_more: bool,
}

impl<T> CursorPageData<T> {
    pub fn new(items: Vec<T>, total: i64, limit: i64, next_cursor: Option<i64>) -> Self {
        let has_more = items.len() as i64 >= limit;
        Self {
            items,
            total,
            limit,
            next_cursor,
            has_more,
        }
    }
}

// ---------- 企业相关的对外结构体 ----------
#[derive(Debug, Serialize)]
pub struct EnterpriseSummary {
    pub uuid: Uuid,
    pub name: String,
    pub credit_score: i32,
    pub tier: String,
    pub industry: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EnterpriseDetail {
    pub uuid: Uuid,
    pub name: String,
    pub credit_score: i32,
    pub tier: String,
    pub industry: Option<String>,
    pub unified_social_credit_code: String,
    pub bio: Option<String>,
    pub logo_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct CreditInfo {
    pub name: String,
    pub credit_score: i32,
    pub level: &'static str,
}

// ---------- 错误码 ----------
pub mod error_code {
    pub const SUCCESS: i32 = 0;
    pub const INVALID_PARAM: i32 = 1001;
    pub const MISSING_PARAM: i32 = 1002;
    pub const UNAUTHORIZED: i32 = 2001;
    pub const INVALID_TOKEN: i32 = 2002;
    pub const PERMISSION_DENIED: i32 = 2004;
    pub const USER_NOT_FOUND: i32 = 3001;
    pub const ENTERPRISE_NOT_FOUND: i32 = 3002;
    pub const EMAIL_ALREADY_EXISTS: i32 = 3003;
    pub const CREDIT_INSUFFICIENT: i32 = 3004;
    pub const ACCOUNT_FROZEN: i32 = 3005;
    pub const OPERATION_NOT_ALLOWED: i32 = 3006;
    pub const DB_ERROR: i32 = 4001;
    pub const DB_CONNECTION_ERROR: i32 = 4002;
    pub const DB_QUERY_ERROR: i32 = 4003;
    pub const DB_CONFLICT: i32 = 4004;
    pub const INTERNAL_ERROR: i32 = 5001;
    pub const SERVICE_UNAVAILABLE: i32 = 5002;
}
