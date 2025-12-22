#[derive(PartialEq, Clone, Debug)]
pub enum EngineEvent {
    AuthFlowCompleted,
    AuthFlowFailed { reason: String },
}