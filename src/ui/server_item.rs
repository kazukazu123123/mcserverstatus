use crate::server::{Server, ServerPingInfo, ServerStatus, ServerStatusEvent};
use eframe::egui::{Align, Button, Color32, Layout, Ui};

pub struct ServerItem;

const BUTTON_WIDTH: f32 = 100.0;

impl ServerItem {
    pub fn show(
        ui: &mut Ui,
        server: &Server,
    ) -> bool {
        let mut keep = true;

        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Server information
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("{}: {}", server.name, server.status));
                        // TODO: Other server information can be added here
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
                            keep = false;
                        }
                    });

                    ui.scope(|ui| {
                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                            Color32::from_rgb(0, 180, 0);
                        ui.style_mut().visuals.widgets.active.weak_bg_fill =
                            Color32::from_rgb(0, 140, 0);

                        let refresh_button_enabled = {
                            match server.status {
                                ServerStatus::Online => true,
                                ServerStatus::Offline => true,
                                _ => false,
                            }
                        };

                        ui.add_enabled_ui(refresh_button_enabled, |ui| {
                            if ui
                                .add_sized([BUTTON_WIDTH, button_height], Button::new("⟳ Refresh"))
                                .clicked()
                            {
                                todo!("Ping server");
                            }
                        });
                    });
                });
            });
        });

        keep
    }
}
