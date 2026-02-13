use anyhow::Result;
use crate::config::models::{DnsConfig, DnsServersConfig, InboundConfig};

pub fn platform_to_dns(platform: &str) -> Result<DnsConfig> {
    match platform {
        "android" | "container" => Ok(DnsConfig {
            servers: vec![DnsServersConfig {
                type_: Some("tls".to_string()),
                tag: "default".to_string(),
                server: Some("1.1.1.1".to_string()),
                detour: "auto-select".to_string(),
                address: None,
            }],
        }),
        "ios" => Ok(DnsConfig {
            servers: vec![DnsServersConfig {
                type_: None,
                tag: "default".to_string(),
                server: None,
                detour: "auto-select".to_string(),
                address: Some("tls://1.1.1.1".to_string()),
            }],
        }),
        _ => anyhow::bail!("unsupported platform type"),
    }
}

pub fn platform_to_inbound(platform: &str) -> Result<InboundConfig> {
    match platform {
        "android" | "ios" => Ok(InboundConfig {
            type_: "tun".to_string(),
            tag: "tun-in".to_string(),
            listen_port: None,
            listen: None,
            mtu: Some(1358),
            address: Some("172.19.0.1/30".to_string()),
            auto_route: Some(true),
            strict_route: Some(true),
            stack: Some("gvisor".to_string()),
            endpoint_independent_nat: Some(true),
        }),
        "container" => Ok(InboundConfig {
            type_: "http".to_string(),
            tag: "http-in".to_string(),
            listen_port: Some(2026),
            listen: Some("0.0.0.0".to_string()),
            mtu: None,
            address: None,
            auto_route: None,
            strict_route: None,
            stack: None,
            endpoint_independent_nat: None,
        }),
        _ => anyhow::bail!("unsupported platform type"),
    }
}
