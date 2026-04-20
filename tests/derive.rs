use dotenv_derive::Bind;

#[derive(Bind)]
pub struct DefaultConfig {
    #[dotenv("TEST_APP_KEY")]
    pub app_key: &'static str,
    #[dotenv("TEST_APP_SECRET")]
    pub app_secret: &'static str,
}

#[derive(Bind)]
#[dotenv_static]
pub struct StaticConfig {
    #[dotenv("TEST_APP_KEY")]
    pub app_key: &'static str,
    #[dotenv("TEST_APP_SECRET")]
    pub app_secret: &'static str,
}

#[test]
fn default_impl_has_correct_values() {
    let cfg = DefaultConfig::default();
    assert_eq!(cfg.app_key, "test_key_value");
    assert_eq!(cfg.app_secret, "test_secret_value");
}

#[test]
fn static_instance_has_correct_values() {
    assert_eq!(StaticConfig::INSTANCE.app_key, "test_key_value");
    assert_eq!(StaticConfig::INSTANCE.app_secret, "test_secret_value");
}
