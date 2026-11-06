//! WiFi interface model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiState { Disconnected, Scanning, Associating, Connected }

#[derive(Debug, Clone)]
pub struct WifiInterface {
    pub name: String,
    pub mac: [u8; 6],
    pub state: WifiState,
    pub channel: u32,
}

impl WifiInterface {
    pub fn new(name: String, mac: [u8; 6]) -> Self {
        Self { name, mac, state: WifiState::Disconnected, channel: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wifi_interface() {
        let w = WifiInterface::new("wlan0".into(), [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(w.state, WifiState::Disconnected);
    }
}
