use super::SmithayAppRunnerState;
use smithay::wayland::security_context::{
    SecurityContext, SecurityContextHandler, SecurityContextListenerSource,
};

impl SecurityContextHandler for SmithayAppRunnerState {
    fn context_created(
        &mut self,
        _source: SecurityContextListenerSource,
        _security_context: SecurityContext,
    ) {
    }
}

smithay::delegate_security_context!(SmithayAppRunnerState);
