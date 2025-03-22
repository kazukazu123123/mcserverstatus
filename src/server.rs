use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::time::Duration;

static mut NEXT_ID: u32 = 1;

#[derive(Clone, PartialEq)]
pub struct Server {
    pub id: u32,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub status: ServerStatus,
}

pub struct ServerStatusEvent {
    pub server_id: u32,
    pub status: ServerStatus,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ServerStatus {
    Unknown,
    Online,
    Offline,
    Pinging,
    Error,
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerStatus::Unknown => write!(f, "Unknown"),
            ServerStatus::Online => write!(f, "Online"),
            ServerStatus::Offline => write!(f, "Offline"),
            ServerStatus::Pinging => write!(f, "Pinging"),
            ServerStatus::Error => write!(f, "Error"),
        }
    }
}

#[derive(PartialEq)]
pub struct ServerInfo {
    pub motd: Option<String>,
    pub players: Option<Vec<Player>>,
    pub max_players: Option<u16>,
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
    pub fn new(name: String, ip: String, port: u16) -> Self {
        unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            Self {
                id,
                name,
                ip,
                port,
                status: ServerStatus::Unknown,
            }
        }
    }

    // Address resolution
    fn resolve_address(&self) -> Result<SocketAddr, io::Error> {
        if let Ok(ip) = IpAddr::from_str(&self.ip) {
            // IP address is specified
            Ok(SocketAddr::new(ip, self.port))
        } else {
            // Domain name is specified
            let address = format!("{}:{}", self.ip, self.port);
            match address.to_socket_addrs() {
                Ok(mut addresses) => addresses
                    .next()
                    .ok_or(io::Error::new(io::ErrorKind::NotFound, "No address found")),
                Err(e) => Err(e),
            }
        }
    }

    // Ping the server
    pub fn ping(&mut self) -> ServerPingInfo {
        if self.status == ServerStatus::Pinging {
            return ServerPingInfo {
                id: self.id,
                status: ServerStatus::Pinging,
                info: None,
            };
        }
        self.status = ServerStatus::Pinging;

        let socket_addr = match self.resolve_address() {
            Ok(address) => address,
            Err(_) => {
                self.status = ServerStatus::Error;
                return ServerPingInfo {
                    id: self.id,
                    status: ServerStatus::Error,
                    info: None,
                };
            }
        };

        println!("Attempting to connect to: {:?}", socket_addr);

        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(15)) {
            Ok(_stream) => ServerPingInfo {
                id: self.id,
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
            },
            Err(e) => {
                self.status = ServerStatus::Offline;
                println!("Failed to connect: {}", e);
                ServerPingInfo {
                    id: self.id,
                    status: ServerStatus::Offline,
                    info: None,
                }
            }
        }
    }
}
