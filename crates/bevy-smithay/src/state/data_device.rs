use super::SmithayAppRunnerState;
use smithay::input::Seat;
use smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::selection::data_device::{
    ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
};

impl ClientDndGrabHandler for SmithayAppRunnerState {
    fn dropped(&mut self, _target: Option<WlSurface>, _validated: bool, _seat: Seat<Self>) {}

    fn started(
        &mut self,
        _source: Option<WlDataSource>,
        _icon: Option<WlSurface>,
        _seat: Seat<Self>,
    ) {
    }
}

impl ServerDndGrabHandler for SmithayAppRunnerState {}

impl DataDeviceHandler for SmithayAppRunnerState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.smithay_state.data_device_state
    }
}

smithay::delegate_data_device!(SmithayAppRunnerState);
