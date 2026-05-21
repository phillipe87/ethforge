 /*
 packet.rs - Packet configuration data model

 This module owns the PacketConfig struct which represents everything the user
 configures for a packet.

*/

// enum for ethertype selection
#[derive(Debug, Clone, PartialEq)]
pub enum EtherType {
  None,
  Arp,
  Ipv4,
}

// enum for layer 4 protocol selection
#[derive(Debug, Clone, PartialEq)]
pub enum L4Proto {
  None,
  Tcp,
  Udp,
  Icmp,
}

// PacketConfig
#[derive(Debug, Clone, PartialEq)]
pub struct PacketConfig {
  // Ethernet header
  pub src_mac: String,
  pub dst_mac: String,
  pub ether_type: EtherType,

  // ARP fields
  pub arp_src_ip: String,
  pub arp_dst_ip: String,

  // IPv4 fields
  pub ipv4_src: String,
  pub ipv4_dst: String,
  pub ipv4_ttl: String,
  pub l4_proto: L4Proto,

  // TCP/UDP fields
  pub src_port: String,
  pub dst_port: String,

  // Payload
  pub payload_hex: String,
}

// Default values for PacketConfig
impl Default for PacketConfig {
  fn default() -> Self {
    Self {
      src_mac:     "00:00:00:00:00:00".to_string(),
      dst_mac:     "FF:FF:FF:FF:FF:FF".to_string(),
      ether_type:  EtherType::Ipv4,
      arp_src_ip:  "192.168.1.1".to_string(),
      arp_dst_ip:  "192.168.1.2".to_string(),
      ipv4_src:    "192.168.1.1".to_string(),
      ipv4_dst:    "192.168.1.2".to_string(),
      ipv4_ttl:    "64".to_string(),
      l4_proto:    L4Proto::Udp,
      src_port:    "12345".to_string(),
      dst_port:    "80".to_string(),
      payload_hex: "48656C6C6F".to_string(), // "Hello" in hex
    }
  }
}

