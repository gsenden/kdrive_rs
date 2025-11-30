#[derive(Clone)]
#[derive(PartialEq)]
pub enum EngineEvent {
    AuthFlowCompleted,
    AuthFlowFailed { reason: String },
}