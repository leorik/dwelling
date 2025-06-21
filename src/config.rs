use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct DwellingConfig {
    #[envconfig(from = "PORT", default = "8080")]
    pub app_port: u16,
}

pub fn load_config() -> DwellingConfig {
    DwellingConfig::init_from_env().unwrap()
}
