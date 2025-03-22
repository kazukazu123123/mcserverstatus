use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

use crate::event::ServerEvent;

#[derive(Clone, PartialEq)]
pub struct Server {
    is_pinging: bool,
    pub id: usize,
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Clone, PartialEq)]
pub enum ServerStatus {
    Unknown,
    Online,
    Offline,
    Pinging,
    Error(ServerPingError),
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerStatus::Unknown => write!(f, "Unknown"),
            ServerStatus::Online => write!(f, "Online"),
            ServerStatus::Offline => write!(f, "Offline"),
            ServerStatus::Pinging => write!(f, "Pinging"),
            ServerStatus::Error(e) => match e {
                ServerPingError::DnsResolveError => write!(f, "DNS Resolve Error"),
                ServerPingError::ConnectionError => write!(f, "Connection Error"),
            },
        }
    }
}

#[derive(PartialEq)]
pub struct ServerInfo {
    pub motd: Option<String>,
    pub players: Option<Vec<Player>>,
    pub max_players: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerPingError {
    DnsResolveError,
    ConnectionError,
}

pub struct ServerPingInfo {
    pub id: u32,
    pub status: ServerStatus,
    pub info: Option<ServerInfo>,
}

#[derive(PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub uuid: String,
}

impl Server {
    pub fn new(id: usize, name: String, ip: String, port: u16) -> Self {
        Self {
            is_pinging: false,
            id,
            name,
            ip,
            port,
        }
    }

    pub fn is_pinging(&self) -> bool {
        self.is_pinging
    }

    // Address resolution
    fn resolve_address(&self) -> Result<SocketAddr, io::Error> {
        // If an IP address is specified
        if let Ok(ip) = IpAddr::from_str(&self.ip) {
            return Ok(SocketAddr::new(ip, self.port));
        }

        // Try to resolve the domain name
        let addr = format!("{}:{}", self.ip, self.port);
        let mut addrs_iter = addr.to_socket_addrs()?;

        // Return the first resolved address
        addrs_iter.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("Could not resolve address: {}", self.ip),
            )
        })
    }

    // Ping the server
    async fn ping(&mut self) -> Result<ServerPingInfo, ServerPingError> {
        let socket_addr = self
            .resolve_address()
            .map_err(|_| ServerPingError::DnsResolveError)?;

        println!("Attempting to connect to: {:?}", socket_addr);

        match TcpStream::connect(&socket_addr).await {
            Ok(_stream) => Ok(ServerPingInfo {
                id: self.id as u32,
                status: ServerStatus::Online,
                info: Some(ServerInfo {
                    motd: Some("Minecraft Server".to_string()),
                    players: Some(vec![
                        Player {
                            name: "Player1".to_string(),
                            uuid: "uuid1".to_string(),
                        },
                        Player {
                            name: "Player2".to_string(),
                            uuid: "uuid2".to_string(),
                        },
                    ]),
                    max_players: Some(20),
                }),
            }),
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err(ServerPingError::ConnectionError)
            }
        }
    }

    // Check the server status
    pub async fn check_server_status(&mut self, tx: Sender<ServerEvent>) {
        if self.is_pinging {
            return;
        }

        self.is_pinging = true;

        let _ = tx
            .send(ServerEvent::PingStatus {
                id: self.id,
                status: ServerStatus::Pinging,
            })
            .await;

        match self.ping().await {
            Ok(ping_info) => {
                let _ = tx
                    .send(ServerEvent::PingStatus {
                        id: self.id,
                        status: ping_info.status,
                    })
                    .await;
            }
            Err(e) => {
                println!("Error while pinging server: {:?}", e);

                let _ = tx
                    .send(ServerEvent::PingStatus {
                        id: self.id,
                        status: ServerStatus::Error(e),
                    })
                    .await;
            }
        }

        self.is_pinging = false;
    }
}
