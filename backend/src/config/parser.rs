use crate::config::models::{OutboundConfig, TlsConfig, UtlsConfig, RealityConfig};

pub fn vps_to_outbound(
    vps: &crate::config::database::VpsCredential,
    index: usize
) -> OutboundConfig {
    OutboundConfig {
        type_: "vless".to_string(),
        tag: format!("vps-{index}"),
        server: Some(vps.ip.clone()),
        server_port: Some(443),
        uuid: Some(vps.uuid.clone()),
        flow: Some("xtls-rprx-vision".to_string()),
        packet_encoding: Some("xudp".to_string()),
        tls: Some(TlsConfig {
            enabled: true,
            server_name: "www.google.com".to_string(),
            utls: UtlsConfig {
                enabled: true,
                fingerprint: "chrome".to_string(),
            },
            reality: RealityConfig {
                enabled: true,
                public_key: vps.pbk.clone(),
                short_id: vps.sid.clone(),
            },
        }),
        outbounds: None,
        interval: None,
        tolerance: None,
        url: None,
        default: None,
    }
}
