use dialoguer::{Input, theme::ColorfulTheme};
use shared::{ClientMessage, ServerMessage};
use std::net::UdpSocket;
use std::time::Duration;

pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub username: String,
}

impl ConnectionInfo {
    pub fn prompt_user() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🎮 Welcome to Maze Wars!");
        println!("=============================");

        let theme = ColorfulTheme::default();

        // Prompt for server address
        let host: String = Input::with_theme(&theme)
            .with_prompt("Server address")
            .default("127.0.0.1".to_string())
            .interact_text()?;

        // Prompt for port
        let port: u16 = Input::with_theme(&theme)
            .with_prompt("Server port")
            .default(8080)
            .interact_text()?;

        // Test server connection first
        println!("🔍 Testing connection to {}:{}...", host, port);
        if !test_server_health(&host, port) {
            return Err("❌ Cannot connect to server. Please check the address and port.".into());
        }
        println!("✅ Server connection successful!");

        // Prompt for username
        let username: String = Input::with_theme(&theme)
            .with_prompt("Enter your username")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("Username cannot be empty")
                } else if input.len() > 20 {
                    Err("Username must be 20 characters or less")
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        // Test username availability
        println!("🔍 Checking username availability...");
        match test_username_availability(&host, port, &username) {
            UsernameStatus::Available => {
                println!("✅ Username '{}' is available!", username);
            }
            UsernameStatus::Taken => {
                return Err(
                    "❌ Username is already taken. Please restart and try a different name.".into(),
                );
            }
            UsernameStatus::Error(msg) => {
                return Err(format!("❌ Error checking username: {}", msg).into());
            }
        }

        Ok(ConnectionInfo {
            host,
            port,
            username,
        })
    }
}

#[derive(Debug)]
enum UsernameStatus {
    Available,
    Taken,
    Error(String),
}

fn test_server_health(host: &str, port: u16) -> bool {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return false,
    };

    if socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .is_err()
    {
        return false;
    }

    let server_addr = format!("{}:{}", host, port);
    let test_msg = ClientMessage::TestHealth;

    let serialized = match serde_json::to_string(&test_msg) {
        Ok(s) => s,
        Err(_) => return false,
    };

    if socket.send_to(serialized.as_bytes(), &server_addr).is_err() {
        return false;
    }

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, _)) => {
            let response = String::from_utf8_lossy(&buf[..len]);
            serde_json::from_str::<ServerMessage>(&response).is_ok()
        }
        Err(_) => false,
    }
}

fn test_username_availability(host: &str, port: u16, username: &str) -> UsernameStatus {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => return UsernameStatus::Error(format!("Failed to create socket: {}", e)),
    };

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(5))) {
        return UsernameStatus::Error(format!("Failed to set timeout: {}", e));
    }

    let server_addr = format!("{}:{}", host, port);
    let join_msg = ClientMessage::JoinGame {
        player_name: username.to_string(),
    };

    let serialized = match serde_json::to_string(&join_msg) {
        Ok(s) => s,
        Err(e) => return UsernameStatus::Error(format!("Failed to serialize message: {}", e)),
    };

    if let Err(e) = socket.send_to(serialized.as_bytes(), &server_addr) {
        return UsernameStatus::Error(format!("Failed to send message: {}", e));
    }

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, _)) => {
            let response = String::from_utf8_lossy(&buf[..len]);
            match serde_json::from_str::<ServerMessage>(&response) {
                Ok(ServerMessage::NameAlreadyTaken) => UsernameStatus::Taken,
                Ok(ServerMessage::GameJoined { .. }) => {
                    // Send leave message to clean up the test connection
                    let leave_msg = ClientMessage::LeaveGame;
                    if let Ok(leave_serialized) = serde_json::to_string(&leave_msg) {
                        let _ = socket.send_to(leave_serialized.as_bytes(), &server_addr);
                    }
                    UsernameStatus::Available
                }
                Ok(_) => UsernameStatus::Available, // Other responses mean we can connect
                Err(e) => UsernameStatus::Error(format!("Invalid server response: {}", e)),
            }
        }
        Err(e) => UsernameStatus::Error(format!("No response from server: {}", e)),
    }
}
