use anyhow::anyhow;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Deserialize, Default, Serialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub serve_port: u16,
    pub unsafe_deploy: bool,
    pub db_conncter_max_connctions: u32,
    pub debug_log: bool,
    /*---保留字段---*/
    pub base_host_name: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<String>,
    pub smtp_user_name: Option<String>,
    pub smtp_passwd: Option<String>,
    pub smtp_ratelimit: Option<i32>, /*同个IP日最大可发起发送邮件请求次数，保留字段*/
    /*smtp不使用ssl/tls合并到unsafe_deploy,在此不做另外选项*/
    /*---保留字段---*/
    pub meta: Option<MetaConfig>,
}

#[derive(Deserialize, Default, Serialize, Clone)]
pub struct MetaConfig {
    pub serve_meta_endpoint: bool,
    pub business_name: String,
    pub admin_contact: Option<Vec<AdminContact>>,
}

#[derive(Deserialize, Default, Serialize, Clone)]
pub struct AdminContact {
    pub phone: Option<String>,
    pub e_mail: Option<String>,
    pub role: Option<String>,
}

pub async fn create_or_load_config() -> anyhow::Result<Config> {
    match fs::read_to_string("config.toml").await {
        Ok(content) => Ok(toml::from_str(&content)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default_config = Config {
                database_url: "postgresql://user:pass@localhost/db".to_string(),
                serve_port: 9098,
                unsafe_deploy: false,
                db_conncter_max_connctions: 10,
                ..Default::default()
            };
            debug!("No profile was found, so a new profile has been created");
            let toml_str = toml::to_string(&default_config)?;
            fs::write("config.toml", toml_str).await?;
            Err(anyhow!(
                "file crated success! pls edit file and restart server"
            ))
        }
        Err(e) => Err(e.into()),
    }
}
