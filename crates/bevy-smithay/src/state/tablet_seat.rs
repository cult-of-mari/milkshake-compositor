use super::SmithayAppRunnerState;
use smithay::backend::input::TabletToolDescriptor;
use smithay::input::pointer::CursorImageStatus;
use smithay::wayland::tablet_manager::TabletSeatHandler;

impl TabletSeatHandler for SmithayAppRunnerState {
    fn tablet_tool_image(&mut self, _tool: &TabletToolDescriptor, _image: CursorImageStatus) {}
}

smithay::delegate_tablet_manager!(SmithayAppRunnerState);
