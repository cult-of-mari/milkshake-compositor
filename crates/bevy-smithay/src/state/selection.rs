use super::SmithayAppRunnerState;
use smithay::wayland::selection::SelectionHandler;

impl SelectionHandler for SmithayAppRunnerState {
    type SelectionUserData = ();
}
