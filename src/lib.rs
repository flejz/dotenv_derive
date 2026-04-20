mod codegen;
mod parse;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derives struct initialization from `.env` values at compile time.
///
/// Each field must carry `#[env("VAR_NAME")]` and be typed `&'static str`.
/// The consumer crate must also depend on `dotenvy_macro`.
///
/// # Modes
///
/// ## `impl Default` (default)
///
/// ```rust,ignore
/// #[derive(Bind)]
/// pub struct MailConfig {
///     #[env("MAIL_HOST")]
///     pub host: &'static str,
///     #[env("MAIL_API_KEY")]
///     pub api_key: &'static str,
/// }
///
/// let cfg = MailConfig::default();
/// ```
///
/// ## `pub const INSTANCE` (with `#[env_static]`)
///
/// Add `#[env_static]` to emit a compile-time constant usable in `const` contexts:
///
/// ```rust,ignore
/// #[derive(Bind)]
/// #[env_static]
/// pub struct MailConfig {
///     #[env("MAIL_HOST")]
///     pub host: &'static str,
///     #[env("MAIL_API_KEY")]
///     pub api_key: &'static str,
/// }
///
/// pub const CONFIG: AppConfig = AppConfig {
///     mail: MailConfig::INSTANCE,
/// };
/// ```
///
/// # Errors
///
/// Compile error if:
/// - Applied to an enum, union, tuple struct, or unit struct
/// - Any field is missing `#[env("VAR_NAME")]`
/// - `#[env(...)]` argument is not a string literal
#[proc_macro_derive(Bind, attributes(env, env_static))]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let is_static = input.attrs.iter().any(|a| a.path().is_ident("env_static"));

    let bindings = match parse::parse_derive_input(&input) {
        Ok(b) => b,
        Err(e) => return e.to_compile_error().into(),
    };

    if is_static {
        codegen::emit_static(&input.ident, &bindings).into()
    } else {
        codegen::emit_default(&input.ident, &bindings).into()
    }
}
