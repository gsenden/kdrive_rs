use common::domain::errors::ApplicationError;

#[derive(PartialEq, Clone, Debug)]
pub enum EngineEvent {
    AuthFlowCompleted,
    AuthFlowFailed { reason: ApplicationError },
}