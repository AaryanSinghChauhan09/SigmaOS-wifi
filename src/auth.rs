//! Authentication model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod { Open, Wep, WpaPsk, Wpa2Psk, Wpa3Sae }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthStatus { InProgress, Success, Failed }

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub ssid: String,
    pub method: AuthMethod,
    pub status: AuthStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_session() {
        let a = AuthSession { ssid: "Test".into(), method: AuthMethod::Wpa2Psk, status: AuthStatus::InProgress };
        assert_eq!(a.method, AuthMethod::Wpa2Psk);
    }
}
