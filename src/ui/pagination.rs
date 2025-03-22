use std::collections::HashMap;

use crate::server::Server;
use eframe::egui::{self, Layout, TopBottomPanel};

// Helper function to provide pagination information
pub fn get_pagination_info(
    servers: &HashMap<usize, Server>,
    items_per_page: usize,
) -> (usize, usize) {
    (servers.len(), items_per_page)
}

pub fn show(
    ctx: &egui::Context,
    current_page: &mut usize,
    total_items: usize,
    items_per_page: usize,
) {
    let total_pages = (total_items as f32 / items_per_page as f32).ceil() as usize;

    // If the number of pages is 0, return early
    if total_pages == 0 {
        return;
    }

    // Add constraints to ensure current_page is within range
    *current_page = (*current_page).min(total_pages - 1).max(0);

    // Use BottomPanel to display pagination
    TopBottomPanel::bottom("pagination_panel").show(ctx, |ui| {
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.horizontal(|ui| {
                    // "First page" button
                    ui.add_enabled_ui(*current_page > 0, |ui| {
                        if ui.button("<<").clicked() {
                            *current_page = 0;
                        }
                    });

                    // "Previous page" button
                    ui.add_enabled_ui(*current_page > 0, |ui| {
                        if ui.button("<").clicked() {
                            *current_page -= 1;
                        }
                    });

                    // Page display
                    ui.add(
                        egui::DragValue::new(&mut *current_page)
                            .update_while_editing(false)
                            .range(0..=total_pages)
                            .custom_formatter(|n, _| {
                                let n = n as u32;
                                format!("{}", n + 1)
                            }),
                    );
                    ui.label(format!("/ {}", total_pages));

                    // "Next page" button
                    ui.add_enabled_ui(*current_page < total_pages - 1, |ui| {
                        if ui.button(">").clicked() {
                            *current_page += 1;
                        }
                    });

                    // "Last page" button
                    ui.add_enabled_ui(*current_page < total_pages - 1, |ui| {
                        if ui.button(">>").clicked() {
                            *current_page = total_pages - 1;
                        }
                    });
                });
            },
        );
    });
}
