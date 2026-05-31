//! packet.rs — Packet configuration data model
//!
//! This module contains the data structures that represent a packet
//! configuration, the parsing and building logic.
//!
//! # Structure
//! - [`EtherType`] — EtherType field selector
//! - [`PacketConfig`] — complete packet configuration
//! - [`parse_mac`] — MAC address string parser
//! - [`build_packet`] — builds raw bytes from a [`PacketConfig`]

use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::util::MacAddr;

/// EtherType field.
///
/// Identifies the protocol encapsulated in the frame payload.
/// Values are defined in IEEE 802.3.
///
/// # Example
/// ```
/// let et = EtherType::Ipv4;
/// assert_eq!(et.value(), 0x0800);
/// assert_eq!(et.label(), "IPv4  (0x0800)");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum EtherType {
  /// No Ethertype - raw Ethernet frame
  None,
  /// Address Resolution Protocol - 0x0806
  Arp,
  /// Internet Protocol v4 - 0x0800
  Ipv4,
  /// Internet Protocol v6 - 0x86DD
  Ipv6,
  /// VLAN tagged frame IEEE 802.1Q - 0x8100
  Vlan,
}

impl EtherType {

  /// Returns the 16-bit EtherType value.
  /// [`EtherType::None`] returns `0x0000`.
  pub fn value(&self) -> u16 {
    match self {
      EtherType::None => 0x0000,
      EtherType::Ipv4 => 0x0800,
      EtherType::Arp  => 0x0806,
      EtherType::Ipv6 => 0x86DD,
      EtherType::Vlan => 0x8100,
    }
  }

  /// Returns a human readable label for display
  pub fn label(&self) -> &str {
    match self {
      EtherType::None => "None (0x0000)",
      EtherType::Ipv4 => "None (0x0800)",
      EtherType::Arp  => "None (0x0806)",
      EtherType::Ipv6 => "None (0x86DD)",
      EtherType::Vlan => "None (0x8100)",
    }
  }

  /// Returns all currently supported Ethertypes
  ///
  /// # Example
  /// ```
  /// for et in EtherType::all() {
  ///   println!("{}", et.label());
  /// }
  /// ```
  pub fn all() -> &'static [EtherType] {
    &[
      EtherType::None,
      EtherType::Ipv4,
      EtherType::Arp,
      EtherType::Ipv6,
      EtherType::Vlan,
    ]
  }
}

// enum for layer 4 protocol selection
#[derive(Debug, Clone, PartialEq)]
pub enum L4Proto {
  None, // raw Ethernet frame only
  Tcp,
  Udp,
  Icmp,
}

/// Ethernet frame configuration
///
/// # Example
/// ```
/// let config = PacketConfig::default();
/// assert_eq!(config.src_mac, "00:00:00:00:00:00");
/// assert_eq!(config.ethertype, EtherType::Ipv4;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PacketConfig {
  /// Source MAC address
  pub src_mac   : String,

  /// Destination MAC address
  pub dst_mac   : String,

  /// EtherType
  pub ether_type: EtherType,

  //// ARP fields
  //pub arp_src_ip: String,
  //pub arp_dst_ip: String,

  //// IPv4 fields
  //pub ipv4_src: String,
  //pub ipv4_dst: String,
  //pub ipv4_ttl: String,
  //pub l4_proto: L4Proto,

  //// TCP/UDP fields
  //pub src_port: String,
  //pub dst_port: String,

  //// Payload
  //pub payload_hex: String,
}

// Default values for PacketConfig
impl Default for PacketConfig {
  fn default() -> Self {
    Self {
      src_mac:     "00:00:00:00:00:00".to_string(),
      dst_mac:     "FF:FF:FF:FF:FF:FF".to_string(),
      ether_type:  EtherType::None,
      //ether_type:  EtherType::Ipv4,
      //arp_src_ip:  "192.168.1.1".to_string(),
      //arp_dst_ip:  "192.168.1.2".to_string(),
      //ipv4_src:    "192.168.1.1".to_string(),
      //ipv4_dst:    "192.168.1.2".to_string(),
      //ipv4_ttl:    "64".to_string(),
      //l4_proto:    L4Proto::Udp,
      //src_port:    "12345".to_string(),
      //dst_port:    "80".to_string(),
      //payload_hex: "48656C6C6F".to_string(), // "Hello" in hex
    }
  }
}



/// Parse a MAC address string into a [`MacAddr`]
///
/// # Arguments
/// * `s` - MAC Address in `"AA:BB:CC:DD:EE:FF"` format (hex, colon separated)
///
/// # Returns
/// * `Ok(MacAddr)` on success
/// * `Err(String)` with a description if the format is invalid
///
/// # Example
/// ```
/// let mac = parse_mac("AA:BB:CC:DD:EE:FF").unwrap();
/// let bad = parse_mac("ZZZZ");
/// assert!(bad.is_err());
/// ```
pub fn parse_mac(s: &str) -> Result<MacAddr, String> {

  let octets_str: Vec<&str> = s.split(':').collect();

  if octets_str.len() != 6 {
    return Err(format!("Expected 6 octets, got {}", octets_str.len()));
  }

  let mut octets = [0u8; 6];
  for (i, p) in octets_str.iter().enumerate() {
    octets[i] = u8::from_str_radix(p, 16)
      .map_err(|_| format!("Invalid hex octet: {}", p))?;
  }

  Ok(MacAddr::new(
    octets[0], octets[1], octets[2],
    octets[3], octets[4], octets[5]
  ))
}

/// Build a raw Ethernet frame from a [`PacketConfig`].
///
/// Constructs the 14-byte header of the Ethernet frame from
/// the source MAC,
/// the destination MAC,
/// the EtherType.
///
/// # Returns
/// * `Ok(Vec<u8>)` - raw frame bytes for transmission
/// * `Err(String)` - description of the error
///
/// # Exampe
/// ```
/// let config = PacketConfig::default();
/// let bytes = build_packet(&config).unwrap();
/// assert_eq!(bytes.len(), 14);
/// ```
pub fn build_packet(cfg: &PacketConfig) -> Result<Vec<u8>, String> {

  let src_mac_addr = parse_mac(&cfg.src_mac)?;
  let dst_mac_addr = parse_mac(&cfg.dst_mac)?;

  // create buffer for the Ethernet header
  // 6 bytes for the dst MAC
  // 6 bytes for the src MAC
  // 2 bytes for the EtherType
  // Total: 14 bytes
  let mut buf = vec![0u8; 14];

  let mut eth = MutableEthernetPacket::new(&mut buf)
    .ok_or("Failed to create Ethernet packet buffer")?;

  eth.set_source(src_mac_addr);
  eth.set_destination(dst_mac_addr);
  eth.set_ethertype(pnet::packet::ethernet::EtherType(cfg.ether_type.value()));

  Ok(buf)
}
