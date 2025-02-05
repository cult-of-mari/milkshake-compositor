use super::{ClientState, SmithayAppRunnerState};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Client;
use smithay::wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState};
use smithay::xwayland::XWaylandClientData;

impl CompositorHandler for SmithayAppRunnerState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        println!("compost state");
        &mut self.smithay_state.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        println!("client compost state");

        if let Some(state) = client.get_data::<XWaylandClientData>() {
            return &state.compositor_state;
        }

        if let Some(state) = client.get_data::<ClientState>() {
            return &state.compositor_state;
        }

        unreachable!()
    }

    fn commit(&mut self, _surface: &WlSurface) {
        println!("compost commit");
    }
}

smithay::delegate_compositor!(SmithayAppRunnerState);
