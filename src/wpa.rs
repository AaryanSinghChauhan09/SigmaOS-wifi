//! WPA supplicant model.

#[derive(Debug, Clone)]
pub struct WpaConfig {
    pub ssid: String,
    pub psk: String,
    pub proto: String,
}

#[derive(Debug, Default)]
pub struct WpaSupplicant {
    pub config: Option<WpaConfig>,
}

impl WpaSupplicant {
    pub fn new() -> Self { Self::default() }
    pub fn set_config(&mut self, c: WpaConfig) { self.config = Some(c); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wpa_supplicant() {
        let mut w = WpaSupplicant::new();
        w.set_config(WpaConfig { ssid: "Test".into(), psk: "secret".into(), proto: "WPA2".into() });
        assert!(w.config.is_some());
    }
}
