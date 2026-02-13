use anyhow::Result;
use crate::config::database::fetch_active_vps;
use crate::db::DbPool;
use crate::config::models::*;
use crate::config::parser::vps_to_outbound;
use crate::utils::platform::{platform_to_dns, platform_to_inbound};

pub async fn generate_config(pool: &DbPool, platform: &str) -> Result<SingBoxConfig> {
    let vpses = fetch_active_vps(pool).await?;
    
    let mut outbounds: Vec<OutboundConfig> = vpses
        .iter()
        .enumerate()
        .map(|(i, vps)| vps_to_outbound(vps, i))
        .collect();

    let number = outbounds.len();
    let mut outbounds_for_auto: Vec<String> = (0..number)
        .map(|i| format!("vps-{i}"))
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
        default: None,
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
        default: None,
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
        uuid: None,
    });

    let dns = platform_to_dns(platform)?;
    let inbounds = vec![platform_to_inbound(platform)?];

    Ok(SingBoxConfig {
        log: LogConfig {
            level: "error".to_string(),
            timestamp: true,
        },
        dns,
        inbounds,
        outbounds,
        route: RouteConfig {
            rules: vec![
                RuleConfig {
                    protocol: None,
                    action: "sniff".to_string(),
                },
                RuleConfig {
                    protocol: Some("dns".to_string()),
                    action: "hijack-dns".to_string(),
                },
                RuleConfig {
                    protocol: None,
                    action: "resolve".to_string(),
                },
            ],
            auto_detect_interface: true,
            final_: "proxy-selector".to_string(),
        },
    })
}
