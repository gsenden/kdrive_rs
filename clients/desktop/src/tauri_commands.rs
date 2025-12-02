use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::frontend_errors::FrontendError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub struct TauriCommands {}

impl TauriCommands {
    pub async fn greet(name: &str) -> Result<String, FrontendError> {

        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name })?;

        let result = invoke("greet", args)
            .await
            .as_string()
            .ok_or(FrontendError::WasmInvokeError)?;

        Ok(result)
    }
}