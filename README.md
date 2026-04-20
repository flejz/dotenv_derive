# dotenvy-derive

Proc macro that derives struct initialization from `.env` files at **compile time** via [`dotenv_codegen`](https://crates.io/crates/dotenv_codegen).

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
pub struct ZoomConfig {
    #[dotenv("ZOOM_APP_KEY")]
    pub app_key: &'static str,
    #[dotenv("ZOOM_APP_SECRET")]
    pub app_secret: &'static str,
}

let config = ZoomConfig::default();
```

Generates:

```rust
impl Default for ZoomConfig {
    fn default() -> Self {
        ZoomConfig {
            app_key: ::dotenv_codegen::dotenv!("ZOOM_APP_KEY"),
            app_secret: ::dotenv_codegen::dotenv!("ZOOM_APP_SECRET"),
        }
    }
}
```

### `pub const INSTANCE` (static)

Add `#[dotenv_static]` to emit a compile-time constant instead:

```rust
use dotenvy_derive::Bind;

#[derive(Bind)]
#[dotenv_static]
pub struct ZoomConfig {
    #[dotenv("ZOOM_APP_KEY")]
    pub app_key: &'static str,
    #[dotenv("ZOOM_APP_SECRET")]
    pub app_secret: &'static str,
}

// Use in const context:
pub const CONFIG: AppConfig = AppConfig {
    zoom: ZoomConfig::INSTANCE,
};
```

Generates:

```rust
impl ZoomConfig {
    pub const INSTANCE: ZoomConfig = ZoomConfig {
        app_key: ::dotenv_codegen::dotenv!("ZOOM_APP_KEY"),
        app_secret: ::dotenv_codegen::dotenv!("ZOOM_APP_SECRET"),
    };
}
```

## Requirements

- All fields must be `&'static str` — `dotenv!` returns `&'static str`
- A `.env` file must exist at the crate root at build time
- Consumer crate must depend on `dotenv_codegen` directly

## Error handling

The macro emits a compile error for:

- Non-struct types (enums, unions)
- Tuple or unit structs
- Fields missing `#[dotenv("VAR_NAME")]`
- Malformed `#[dotenv(...)]` attribute

## License

MIT
