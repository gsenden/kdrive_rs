use strum_macros::{Display, EnumIter, VariantNames};

#[derive(EnumIter, Display, VariantNames)]
pub enum TextKeys {
    AuthenticateBtn,
}