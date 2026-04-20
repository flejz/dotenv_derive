use syn::{DeriveInput, Error, Field, Fields, LitStr, Result};

pub struct FieldBinding {
    pub ident: syn::Ident,
    pub env_key: String,
}

pub fn parse_derive_input(input: &DeriveInput) -> Result<Vec<FieldBinding>> {
    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(Error::new_spanned(
                    &input.ident,
                    "Bind: only named-field structs are supported",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "Bind: only structs are supported",
            ));
        }
    };

    fields.iter().map(parse_field).collect()
}

fn parse_field(field: &Field) -> Result<FieldBinding> {
    let ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new_spanned(field, "Bind: unnamed fields not supported"))?;

    let env_attr = field
        .attrs
        .iter()
        .find(|a| a.path().is_ident("env"))
        .ok_or_else(|| {
            Error::new_spanned(
                &ident,
                format!(
                    "Bind: field `{}` missing #[env(\"VAR\")] attribute",
                    ident
                ),
            )
        })?;

    let lit: LitStr = env_attr.parse_args().map_err(|_| {
        Error::new_spanned(
            env_attr,
            "Bind: #[env(...)] expects a string literal, e.g. #[env(\"VAR_NAME\")]",
        )
    })?;

    Ok(FieldBinding {
        ident,
        env_key: lit.value(),
    })
}
