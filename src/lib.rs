use std::{sync::Arc, time::Instant};

use quinn_proto::congestion::{Controller, ControllerFactory};

struct UdpPragueWrapper {}
impl UdpPragueWrapper {
    fn new(arg: Arc<UdpPragueWrapperConfig>, now: Instant, current_mtu: u16) -> Self {
        Self {}
    }
}

impl Controller for UdpPragueWrapper {
    fn on_congestion_event(
        &mut self,
        now: std::time::Instant,
        sent: std::time::Instant,
        is_persistent_congestion: bool,
        is_ecn: bool,
        lost_bytes: u64,
        diff: quinn_proto::EcnCounts,
    ) {
        todo!()
    }

    fn on_mtu_update(&mut self, new_mtu: u16) {
        todo!()
    }

    fn set_window(&mut self, size: u64) {
        todo!()
    }

    fn window(&self) -> u64 {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Controller> {
        todo!()
    }

    fn initial_window(&self) -> u64 {
        todo!()
    }

    fn enable_ect1(&mut self) -> bool {
        true
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct UdpPragueWrapperConfig {}

impl UdpPragueWrapperConfig {}

impl Default for UdpPragueWrapperConfig {
    fn default() -> Self {
        Self {}
    }
}

impl ControllerFactory for UdpPragueWrapperConfig {
    fn build(self: Arc<Self>, now: Instant, current_mtu: u16) -> Box<dyn Controller> {
        Box::new(UdpPragueWrapper::new(self, now, current_mtu))
    }
}
