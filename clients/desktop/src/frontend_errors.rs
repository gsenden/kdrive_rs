use thiserror::Error;
#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("WASM Bindgen error: {0}")]
    WasmBindgenError(String),
    #[error("WASM invoke error")]
    WasmInvokeError,
}

impl From<serde_wasm_bindgen::Error> for FrontendError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        FrontendError::WasmBindgenError(err.to_string())
    }
}