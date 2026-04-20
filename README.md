# dotenvy-derive

Proc macro that derives struct initialization from `.env` files at **compile time** via [`dotenvy_macro`](https://crates.io/crates/dotenvy_macro).

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
dotenvy-derive = "0.15"
dotenvy_macro = "0.15"
```

### `impl Default` (standard)

```rust
use dotenvy_derive::Bind;

#[derive(Bind)]
pub struct MailConfig {
    #[env("MAIL_HOST")]
    pub host: &'static str,
    #[env("MAIL_API_KEY")]
    pub api_key: &'static str,
}

let config = MailConfig::default();
```

Generates:

```rust
impl Default for MailConfig {
    fn default() -> Self {
        MailConfig {
            host: ::dotenvy_macro::dotenv!("MAIL_HOST"),
            api_key: ::dotenvy_macro::dotenv!("MAIL_API_KEY"),
        }
    }
}
```

### `pub const INSTANCE` (static)

Add `#[env_static]` to emit a compile-time constant instead:

```rust
use dotenvy_derive::Bind;

#[derive(Bind)]
#[env_static]
pub struct MailConfig {
    #[env("MAIL_HOST")]
    pub host: &'static str,
    #[env("MAIL_API_KEY")]
    pub api_key: &'static str,
}

// Use in const context:
pub const CONFIG: AppConfig = AppConfig {
    mail: MailConfig::INSTANCE,
};
```

Generates:

```rust
impl MailConfig {
    pub const INSTANCE: MailConfig = MailConfig {
        host: ::dotenvy_macro::dotenv!("MAIL_HOST"),
        api_key: ::dotenvy_macro::dotenv!("MAIL_API_KEY"),
    };
}
```

## Requirements

- All fields must be `&'static str` — `dotenv!` returns `&'static str`
- A `.env` file must exist at the crate root at build time
- Consumer crate must depend on `dotenvy_macro` directly

## Error handling

The macro emits a compile error for:

- Non-struct types (enums, unions)
- Tuple or unit structs
- Fields missing `#[env("VAR_NAME")]`
- Malformed `#[env(...)]` attribute

## License

MIT
