#[macro_use]
extern crate rust_i18n;

mod event;
mod semaphore_manager;
mod server;
mod ui;

use eframe::egui::{self, Context, style::ScrollStyle};
use event::ServerEvent;
use semaphore_manager::SemaphoreManager;
use server::Server;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};

struct McServerStatusApp {
    rt: Runtime,
    tx: Sender<ServerEvent>,
    rx: Receiver<ServerEvent>,
    servers: HashMap<usize, Server>,
    servers_id_counter: usize,
    current_page_index: usize,
    items_per_page: usize,
    server_list: ui::server_list::ServerList,
    semaphore_manager: SemaphoreManager,
}

impl McServerStatusApp {
    fn apply(&mut self, event: ServerEvent) {
        match event {
            ServerEvent::PingStatus { id, status } => {
                if let Some(_) = self.servers.get_mut(&id) {
                    self.server_list.update_status(id, status);
                }
            }
            ServerEvent::RefreshRequest { id } => {
                let Some(server) = self.servers.get_mut(&id) else {
                    return;
                };

                // If the server is already pinging, don't send a new request
                if server.is_pinging() {
                    return;
                }

                let tx_clone = self.tx.clone();
                let mut server_clone = server.clone();
                let semaphore = self.semaphore_manager.semaphore();

                self.rt.spawn(async move {
                    let _permit = semaphore.acquire().await;
                    server_clone.check_server_status(tx_clone).await;
                });
            }
            ServerEvent::RemoveServer { id } => {
                self.servers.remove(&id);
            }
        }
    }
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
            while let Ok(event) = self.rx.try_recv() {
                self.apply(event);
            }

            ui.heading("Minecraft Server Status");

            if ui.button("Add server (test)").clicked() {
                let ip = "localhost".to_string();
                let port = 25565;

                let duplicate = self
                    .servers
                    .values()
                    .any(|s| s.ip == ip && s.port == port);

                if duplicate {
                    println!("This server is already added!");
                } else {
                    let server_id = self.servers_id_counter;
                    let server = Server::new(server_id, "test".to_string(), ip, port);
                    self.servers.insert(server_id, server);
                    self.servers_id_counter += 1;

                    if let Err(e) = self
                        .tx
                        .try_send(ServerEvent::RefreshRequest { id: server_id })
                    {
                        println!("Failed to send refresh request: {:?}", e);
                    }
                }
            }

            if ui.button("Ping all servers").clicked() {
                for server in self.servers.values() {
                    if let Err(e) = self
                        .tx
                        .try_send(ServerEvent::RefreshRequest { id: server.id })
                    {
                        println!("Failed to send refresh request: {:?}", e);
                    }
                }
            }

            self.server_list.show(
                ui,
                &self.tx,
                &self.servers,
                &mut self.current_page_index,
                self.items_per_page,
            );
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Minecraft Server Status",
        options,
        Box::new(|cc: &eframe::CreationContext| {
            cc.egui_ctx.style_mut(|style| {
                style.spacing.scroll = ScrollStyle::solid();
                style.interaction.selectable_labels = false;
            });

            let rt = Runtime::new()?;
            let (tx, rx) = mpsc::channel(16);
            let semaphore_manager = SemaphoreManager::new(5);

            let app: Box<dyn eframe::App> = Box::new(McServerStatusApp {
                rt,
                tx,
                rx,
                servers: Default::default(),
                servers_id_counter: 0,
                current_page_index: 1,
                items_per_page: 10,
                server_list: ui::server_list::ServerList::new(),
                semaphore_manager,
            });

            Ok(app)
        }),
    )
    .expect("Failed to run eframe");

    Ok(())
}
