#[cfg(feature = "pcap-capture")]
use crate::buffer::RingBuffer;
#[cfg(feature = "pcap-capture")]
use crate::config::TrafficCollectorConfig;
#[cfg(feature = "pcap-capture")]
use anyhow::{Context, Result};
#[cfg(feature = "pcap-capture")]
use monitoring_common::{Event, Protocol, TrafficEvent};
#[cfg(feature = "pcap-capture")]
use pcap::{Capture, Device};
#[cfg(feature = "pcap-capture")]
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
#[cfg(feature = "pcap-capture")]
use pnet::packet::ip::IpNextHeaderProtocols;
#[cfg(feature = "pcap-capture")]
use pnet::packet::ipv4::Ipv4Packet;
#[cfg(feature = "pcap-capture")]
use pnet::packet::tcp::TcpPacket;
#[cfg(feature = "pcap-capture")]
use pnet::packet::udp::UdpPacket;
#[cfg(feature = "pcap-capture")]
use pnet::packet::Packet;
#[cfg(feature = "pcap-capture")]
use std::collections::HashMap;
#[cfg(feature = "pcap-capture")]
use std::sync::Arc;
#[cfg(feature = "pcap-capture")]
use tracing::{debug, info, warn};

#[cfg(feature = "pcap-capture")]
pub struct PcapCollector {
    config: TrafficCollectorConfig,
    buffer: Arc<RingBuffer>,
    capture: Capture<pcap::Active>,
}

#[cfg(feature = "pcap-capture")]
impl PcapCollector {
    pub fn new(config: TrafficCollectorConfig, buffer: Arc<RingBuffer>) -> Result<Self> {
        // Get interface
        let interface_name = config.interface.clone()
            .unwrap_or_else(|| {
                Device::lookup()
                    .ok()
                    .flatten()
                    .map(|d| d.name)
                    .unwrap_or_else(|| "any".to_string())
            });

        info!("Opening capture on interface: {}", interface_name);

        let device = Device::list()?
            .into_iter()
            .find(|d| d.name == interface_name)
            .with_context(|| format!("Interface {} not found", interface_name))?;

        let capture = Capture::from_device(device)?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()?;

        Ok(Self {
            config,
            buffer,
            capture,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        info!("Starting packet capture");

        loop {
            match self.capture.next_packet() {
                Ok(packet) => {
                    // Sample based on configured rate
                    if rand::random::<f64>() > self.config.sample_rate {
                        continue;
                    }

                    self.process_packet(&packet);
                }
                Err(pcap::Error::TimeoutExpired) => {
                    // Normal timeout, continue
                    tokio::task::yield_now().await;
                }
                Err(e) => {
                    warn!("Packet capture error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    fn process_packet(&self, packet: &pcap::Packet) {
        let ethernet = match EthernetPacket::new(packet.data) {
            Some(eth) => eth,
            None => return,
        };

        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                    self.process_ipv4(&ipv4);
                }
            }
            _ => {
                // Skip non-IPv4 for now (could add IPv6 support)
            }
        }
    }

    fn process_ipv4(&self, ipv4: &Ipv4Packet) {
        let src_ip = ipv4.get_source().to_string();
        let dst_ip = ipv4.get_destination().to_string();
        let protocol = ipv4.get_next_level_protocol();
        let timestamp = chrono::Utc::now().timestamp_millis();

        match protocol {
            IpNextHeaderProtocols::Tcp => {
                if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                    let src_port = tcp.get_source();
                    let dst_port = tcp.get_destination();
                    
                    let proto = self.identify_protocol_by_port(dst_port);
                    let mut metadata = HashMap::new();
                    metadata.insert("flags".to_string(), format!("{:?}", tcp.get_flags()));

                    let event = Event::Traffic(TrafficEvent {
                        timestamp,
                        protocol: proto,
                        src_ip,
                        dst_ip,
                        src_port,
                        dst_port,
                        bytes: ipv4.get_total_length() as u64,
                        packets: 1,
                        metadata,
                    });

                    if let Err(e) = self.buffer.push(event) {
                        warn!("Buffer full, dropping traffic event: {}", e);
                    }
                }
            }
            IpNextHeaderProtocols::Udp => {
                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                    let src_port = udp.get_source();
                    let dst_port = udp.get_destination();

                    let event = Event::Traffic(TrafficEvent {
                        timestamp,
                        protocol: Protocol::UDP,
                        src_ip,
                        dst_ip,
                        src_port,
                        dst_port,
                        bytes: ipv4.get_total_length() as u64,
                        packets: 1,
                        metadata: HashMap::new(),
                    });

                    if let Err(e) = self.buffer.push(event) {
                        warn!("Buffer full, dropping traffic event: {}", e);
                    }
                }
            }
            IpNextHeaderProtocols::Icmp => {
                let event = Event::Traffic(TrafficEvent {
                    timestamp,
                    protocol: Protocol::ICMP,
                    src_ip,
                    dst_ip,
                    src_port: 0,
                    dst_port: 0,
                    bytes: ipv4.get_total_length() as u64,
                    packets: 1,
                    metadata: HashMap::new(),
                });

                if let Err(e) = self.buffer.push(event) {
                    warn!("Buffer full, dropping traffic event: {}", e);
                }
            }
            _ => {}
        }
    }

    fn identify_protocol_by_port(&self, port: u16) -> Protocol {
        match port {
            80 => Protocol::HTTP,
            443 => Protocol::HTTPS,
            _ => Protocol::TCP,
        }
    }
}
