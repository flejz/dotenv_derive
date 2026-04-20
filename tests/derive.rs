use dotenvy_derive::Bind;

#[derive(Bind)]
pub struct DefaultConfig {
    #[env("TEST_APP_KEY")]
    pub app_key: &'static str,
    #[env("TEST_APP_SECRET")]
    pub app_secret: &'static str,
}

#[derive(Bind)]
#[env_static]
pub struct StaticConfig {
    #[env("TEST_APP_KEY")]
    pub app_key: &'static str,
    #[env("TEST_APP_SECRET")]
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
