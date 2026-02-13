use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceConfig {
    pub vpses: Vec<Vps>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Vps {
    pub ip: String,
    pub uuid: String,
    pub pbk: String,
    pub sid: String,
}

#[derive(Debug, Serialize)]
pub struct SingBoxConfig {
    pub log: LogConfig,
    pub dns: DnsConfig,
    pub inbounds: Vec<InboundConfig>,
    pub outbounds: Vec<OutboundConfig>,
    pub route: RouteConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub level: String,
    pub timestamp: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsConfig {
    pub servers: Vec<DnsServersConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsServersConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub detour: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InboundConfig {
    #[serde(rename = "type")]
    pub type_: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_independent_nat: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutboundConfig {
    #[serde(rename = "type")]
    pub type_: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbounds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub server_name: String,
    pub utls: UtlsConfig,
    pub reality: RealityConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UtlsConfig {
    pub enabled: bool,
    pub fingerprint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RealityConfig {
    pub enabled: bool,
    pub public_key: String,
    pub short_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteConfig {
    pub rules: Vec<RuleConfig>,
    pub auto_detect_interface: bool,
    #[serde(rename = "final")]
    pub final_: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    pub action: String,
}
