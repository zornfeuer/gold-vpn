use std::path::Path;
use axum::{routing::get, Router, Json};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct SourceConfig {
    vpses: Vec<Vps>
}

#[derive(Debug, Deserialize, Serialize)]
struct Vps {
    ip: String,
    uuid: String,
    pbk: String,
    sid: String
}

#[derive(Debug, Serialize)]
struct SingBoxConfig {
    log: LogConfig,
    dns: DnsConfig,
    inbounds: Vec<InboundConfig>,
    outbounds: Vec<OutboundConfig>,
    route: RouteConfig
}

#[derive(Debug, Deserialize, Serialize)]
struct LogConfig {
    level: String,
    timestamp: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct DnsConfig {
    servers: Vec<DnsServersConfig>
}

#[derive(Debug, Deserialize, Serialize)]
struct DnsServersConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    detour: String
}

#[derive(Debug, Deserialize, Serialize)]
struct InboundConfig {
    #[serde(rename = "type")]
    type_: String,
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    listen_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_independent_nat: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
struct OutboundConfig {
    #[serde(rename = "type")]
    type_: String,
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbounds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tolerance: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct TlsConfig {
    enabled: bool,
    server_name: String,
    utls: UtlsConfig,
    reality: RealityConfig
}

#[derive(Debug, Deserialize, Serialize)]
struct UtlsConfig {
    enabled: bool,
    fingerprint: String
}

#[derive(Debug, Deserialize, Serialize)]
struct RealityConfig {
    enabled: bool,
    public_key: String,
    short_id: String
}

#[derive(Debug, Deserialize, Serialize)]
struct RouteConfig {
    rules: Vec<RuleConfig>,
    auto_detect_interface: bool,
    #[serde(rename = "final")]
    final_: String
}

#[derive(Debug, Deserialize, Serialize)]
struct RuleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<String>,
    action: String,
}

fn parse_vpn_config<P: AsRef<Path>>(path: P) -> Result<Vec<Vps>> {
    let raw = std::fs::read_to_string(path)?;
    let config: SourceConfig = serde_json::from_str(&raw)?;
    Ok(config.vpses)
}

fn vps_to_outbound(vps: &Vps, index: usize) -> OutboundConfig {
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
                fingerprint: "chrome".to_string()
            },
            reality: RealityConfig {
                enabled: true,
                public_key: vps.pbk.clone(),
                short_id: vps.sid.clone()
            }
        }),
        outbounds: None,
        interval: None,
        tolerance: None,
        url: None,
        default: None
    }
}

fn platform_to_dns(platform: &str) -> Result<DnsConfig> {
    match platform {
        "android" | "container" => Ok(DnsConfig {
            servers: vec![DnsServersConfig {
                type_: Some("tls".to_string()),
                tag: "default".to_string(),
                server: Some("1.1.1.1".to_string()),
                detour: "auto-select".to_string(),
                address: None
            }]
        }),
        "ios" => Ok(DnsConfig {
            servers: vec![DnsServersConfig {
                type_: None,
                tag: "default".to_string(),
                server: None,
                detour: "auto-select".to_string(),
                address: Some("tls://1.1.1.1".to_string())
            }]
        }),
        _ => anyhow::bail!("unsupported platfrom type")
    }
}

fn platform_to_inbound(platform: &str) -> Result<InboundConfig> {
    match platform {
        "android" | "ios" => {Ok(InboundConfig {
            type_: "tun".to_string(),
            tag: "tun-in".to_string(),
            listen_port: None,
            listen: None,
            mtu: Some(1358),
            address: Some("172.19.0.1/30".to_string()),
            auto_route: Some(true),
            strict_route: Some(true),
            stack: Some("gvisor".to_string()),
            endpoint_independent_nat: Some(true)
        })}
        "container" => {Ok(InboundConfig {
            type_: "http".to_string(),
            tag: "http-in".to_string(),
            listen_port: Some(2026),
            listen: Some("0.0.0.0".to_string()),
            mtu: None,
            address: None,
            auto_route: None,
            strict_route: None,
            stack: None,
            endpoint_independent_nat: None
        })}
        _ => anyhow::bail!("unsupported platfrom type")
    }
    
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let app = Router::new()
        .route("/subscribe/{platform}", get(subscribe));

    let addr = "0.0.0.0:8080";
    info!("🚀 Panel listening on 0.0.0.0:8080");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn subscribe(
    axum::extract::Path(platform): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    let vpses = parse_vpn_config("/home/zornfeuer/doc/creds.json").unwrap();
    
    let mut outbounds: Vec<OutboundConfig> = vpses
        .iter()
        .enumerate()
        .map(|(i, vps)| vps_to_outbound(vps, i))
        .collect();

    let number = outbounds.len();
    let mut outbounds_for_auto: Vec<String> = (0..number)
                .map(|i| format!("vps-{}", i))
                .collect();

    let auto_best = OutboundConfig {
            type_: "urltest".to_string(),
            tag: "auto-select".to_string(),
            outbounds: Some(outbounds_for_auto.clone()),
            interval: Some("5m".to_string()),
            tolerance: Some(100),
            url: Some("https://www.youtube.com/".to_string()),
            server: None,
            server_port: None,
            flow: None,
            uuid: None,
            packet_encoding: None,
            tls: None,
            default: None
        };

    outbounds.push(auto_best);

    outbounds.push(OutboundConfig {
            type_: "direct".to_string(),
            tag: "direct".to_string(),
            outbounds: None,
            interval: None,
            tolerance: None,
            url: None,
            server: None,
            server_port: None,
            flow: None,
            uuid: None,
            packet_encoding: None,
            tls: None,
            default: None
        });

    outbounds_for_auto.push("auto-select".to_string());

    outbounds.push(OutboundConfig {
        tag: "proxy-selector".to_string(),
        type_: "selector".to_string(),
        outbounds: Some(outbounds_for_auto),
        default: Some("auto-select".to_string()),
        url: None,
        flow: None,
        interval: None,
        packet_encoding: None,
        server: None,
        tolerance: None,
        server_port: None,
        tls: None,
        uuid: None
    });

    let dns: DnsConfig = platform_to_dns(&platform).unwrap();

    let inbounds: Vec<InboundConfig> = vec![platform_to_inbound(&platform).unwrap()];

    let config = SingBoxConfig {
        log: LogConfig {
            level: "error".to_string(),
            timestamp: true
        },
        dns,
        inbounds,
        outbounds,
        route: RouteConfig {
            rules: vec![
                RuleConfig {
                    protocol: None,
                    action: "sniff".to_string()
                },
                RuleConfig {
                    protocol: Some("dns".to_string()),
                    action: "hijack-dns".to_string()
                },
                RuleConfig {
                    protocol: None,
                    action: "resolve".to_string()
                },
            ],
            auto_detect_interface: true,
            final_: "proxy-selector".to_string()
        }
    };
    json!(config).into()
}
