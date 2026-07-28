use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// 企业空间核心表
#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Enterprise {
    /// 企业ID
    pub id: i64,
    /// 企业名称
    pub name: String,
    /// 统一社会信用代码
    pub unified_social_credit_code: String,
    /// 法人代表
    pub legal_person: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 联系邮箱
    pub contact_email: Option<String>,
    /// 企业地址
    pub address: Option<String>,
    /// 企业简介（公开可见）
    pub bio: Option<String>,
    /// 企业Logo地址
    pub logo_url: Option<String>,
    /// 所属行业
    pub industry: Option<String>,
    /// 标签数组
    pub tags: Option<Vec<String>>,
    /// 企业空间是否公开可见
    pub public_visible: bool,
    /// 企业信誉分
    pub credit_score: i32,
    /// 套餐等级：free/pro/enterprise
    pub tier: String,
    /// 最大子账号数
    pub max_users: i32,
    /// 每日API调用限额
    pub max_api_calls_per_day: i32,
    /// 最大存储空间(MB)
    pub max_storage_mb: i32,
    /// 企业认证通过时间
    pub verified_at: Option<DateTime<Utc>>,
    /// 认证审核人ID
    pub verified_by: Option<i64>,
    /// 企业状态：active/suspended/closed
    pub status: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
}
