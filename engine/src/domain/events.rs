#[derive(Clone)]
pub enum EngineEvent {
    AuthFlowStarted { url: String },
    AuthCodeReceived,
    TokensStored,
}