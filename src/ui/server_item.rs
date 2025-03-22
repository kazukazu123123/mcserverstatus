use crate::event::ServerEvent;
use crate::server::{Server, ServerPingError, ServerStatus};
use eframe::egui::{Align, Button, Color32, Layout, Ui};
use tokio::sync::mpsc::Sender;

const BUTTON_WIDTH: f32 = 100.0;

pub fn show(ui: &mut Ui, tx: &Sender<ServerEvent>, server: &Server, status: &ServerStatus) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            // Server information
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.vertical(|ui| {
                    let status_text = match status {
                        ServerStatus::Online => "Online",
                        ServerStatus::Offline => "Offline",
                        ServerStatus::Error(e) => match e {
                            ServerPingError::DnsResolveError => "DNS Resolve Error",
                            ServerPingError::ConnectionError => "Connection Error",
                        },
                        ServerStatus::Unknown => "Unknown",
                        ServerStatus::Pinging => "Pinging",
                    };

                    ui.label(format!("{}: {}", server.name, server.ip));
                    ui.label(format!("Status: {}", status_text));
                });
            });

            // Action buttons
            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                let available_height = ui.available_height();
                let button_height = available_height / 2.0;

                ui.scope(|ui| {
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill =
                        Color32::from_rgb(160, 0, 0);
                    if ui
                        .add_sized([BUTTON_WIDTH, button_height], Button::new("🗙 Remove"))
                        .clicked()
                    {
                        let server_id = server.id;
                        if let Err(e) = tx.try_send(ServerEvent::RemoveServer { id: server_id }) {
                            println!("Failed to send remove server event: {:?}", e);
                        }
                    }
                });

                ui.scope(|ui| {
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                        Color32::from_rgb(0, 180, 0);
                    ui.style_mut().visuals.widgets.active.weak_bg_fill =
                        Color32::from_rgb(0, 140, 0);

                    let refresh_button_enabled = *status != ServerStatus::Pinging;

                    ui.add_enabled_ui(refresh_button_enabled, |ui| {
                        if ui
                            .add_sized([BUTTON_WIDTH, button_height], Button::new("⟳ Refresh"))
                            .clicked()
                        {
                            let server_id = server.id;
                            if let Err(e) =
                                tx.try_send(ServerEvent::RefreshRequest { id: server_id })
                            {
                                println!("Failed to send refresh request: {:?}", e);
                            }
                        }
                    });
                });
            });
        });
    });
}
