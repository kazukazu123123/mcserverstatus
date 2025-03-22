use super::server_item::ServerItem;
use crate::server::Server;
use eframe::egui::{self, FontId, Layout, RichText, ScrollArea};

pub fn show(
    ui: &mut egui::Ui,
    servers: &Vec<Server>,
    current_page: &mut usize,
    items_per_page: usize,
) {
    // Calculate total number of items and pages
    let total_items = servers.len();
    let total_pages = if total_items == 0 {
        0
    } else {
        (total_items as f32 / items_per_page as f32).ceil() as usize
    };

    // Add constraints to ensure current_page is within range
    if total_pages == 0 {
        *current_page = 0;
    } else {
        *current_page = (*current_page).min(total_pages - 1).max(0);
    }

    let start_idx = *current_page * items_per_page;
    let end_idx = ((*current_page + 1) * items_per_page).min(servers.len());

    // Ensure start_idx does not exceed the server length
    let start_idx = start_idx.min(servers.len());

    ScrollArea::vertical().show(ui, |ui| {
        if servers.is_empty() {
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.label(RichText::new("No servers found").font(FontId::proportional(40.0)));
                },
            );
        } else {
            // Loop through servers and display them
            for server in servers[start_idx..end_idx].iter() {
                ServerItem::show(ui, server);
            }
        }
    });
}
