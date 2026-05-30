/*
* iface.rs - Network interface discovery
*
* This module uses pnet to enumerate all network interfaces
* on the host and returns them in a format the GUI can display
*/

use pnet::datalink;

#[derive(Debug, Clone)]
pub struct IfaceInfo {
  pub name: String, // e.g. "eth0", "enp3s0"
  pub mac:  String, // a 6-byte value
  pub ipv4: String, // IPv4 address if assigned, else empty string
}

/*
list_interfaces()

Returns all usable network interface on the machine.
FIlters out loopback.
*/
pub fn list_interfaces() -> Vec<IfaceInfo> {
  datalink::interfaces()
    .into_iter()
    .filter( |iface| {
      // skip loopback and interfaces with no MAC
      !iface.is_loopback() && iface.mac.is_some()
    })
    .map( |iface| {
      // store mac address
      let mac = iface.mac.unwrap();

      // format MAC as human readable string
      let mac_str = format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac.0, mac.1, mac.2, mac.3, mac.4, mac.5
      );

      // grab ipv4 address if one exists.
      let ipv4_str = iface
        .ips
        .iter()
        .find_map( |ip| {
          if let std::net::IpAddr::V4(v4) = ip.ip() {
            Some(v4.to_string())
          } else {
            None
          }
        })
        .unwrap_or(String::from(""));

      IfaceInfo {
        name: iface.name,
        mac:  mac_str,
        ipv4: ipv4_str,
      }
    })
    .collect()
}
