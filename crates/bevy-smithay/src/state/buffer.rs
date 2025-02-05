use super::SmithayAppRunnerState;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::wayland::buffer::BufferHandler;

impl BufferHandler for SmithayAppRunnerState {
    fn buffer_destroyed(&mut self, _buffer: &WlBuffer) {}
}
