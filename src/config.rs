use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct DwellingConfig {
    #[envconfig(from = "PORT", default = "8080")]
    pub app_port: u16,

    #[envconfig(from = "DB_HOST", default = "localhost")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,

    #[envconfig(from = "DB_USER", default = "root")]
    pub db_user: String,

    #[envconfig(from = "DB_PASSWORD", default = "")]
    pub db_password: String,

    #[envconfig(from = "DB_NAME", default = "dwelling")]
    pub db_name: String,
}

pub fn load() -> DwellingConfig {
    DwellingConfig::init_from_env().unwrap()
}
