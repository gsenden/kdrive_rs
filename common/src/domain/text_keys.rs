use strum_macros::{Display, EnumIter, VariantNames};

#[derive(EnumIter, Display, VariantNames, Debug)]
pub enum TextKeys {
    AuthenticateBtn,
    InvalidRedirectUrl,
}