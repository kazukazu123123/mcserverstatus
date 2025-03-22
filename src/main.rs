#[macro_use]
extern crate rust_i18n;

mod server;
mod ui;
use eframe::egui::{self, style::ScrollStyle, Context, Spinner};
use server::Server;
use server::ServerPingInfo;
use tokio::runtime::Runtime;
use std::{error::Error, sync::mpsc::{Receiver, Sender}, time::Duration};

struct McServerStatusApp {
    tx: Sender<Option<ServerPingInfo>>,
    rx: Receiver<Option<ServerPingInfo>>,
    servers: Vec<Server>,
    current_page_index: usize,
    items_per_page: usize,
}

impl eframe::App for McServerStatusApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let (total_items, items_per_page) =
            ui::pagination::get_pagination_info(&self.servers, self.items_per_page);
        ui::pagination::show(
            ctx,
            &mut self.current_page_index,
            total_items,
            items_per_page,
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(incr) = self.rx.try_recv() {
                if let Some(result) = incr {
                    self.servers.last_mut().unwrap().status = result.status;
                }
            }

            ui.heading("Minecraft Server Status");

            if ui.button("Add server (test)").clicked() {
                let server = Server::new("test".to_string(), "localhost".to_string(), 25565);

                self.servers.push(server);

                todo!("Ping server");
            }

            ui::server_list::show(
                ui,
                &mut self.servers,
                &mut self.current_page_index,
                self.items_per_page,
            );
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minecraft Server Status",
        options,
        Box::new(|cc: &eframe::CreationContext| {
            cc.egui_ctx.style_mut(|style| {
                style.spacing.scroll = ScrollStyle::solid();
                style.interaction.selectable_labels = false;
            });

            let app: Box<dyn eframe::App> = Box::new(McServerStatusApp {
                tx,
                rx,
                servers: Vec::new(),
                current_page_index: 1,
                items_per_page: 10,
            });

            Ok(app)
        }),
    )
    .expect("Failed to run eframe");

    Ok(())
}
