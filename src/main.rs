use directories::ProjectDirs;
use eframe::{NativeOptions, egui};
use libfunk::*;
use log::*;
use std::path::Path;

fn main() -> eframe::Result {
    let options: NativeOptions = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    colog::default_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let project_dir: Option<ProjectDirs> =
        directories::ProjectDirs::from("org", "codeberg.r6915ee", "dropfunk");
    let data_dir: &Path = project_dir
        .as_ref()
        .map(|dirs| dirs.data_dir())
        .expect("Home directory was not found for accessing data");

    match std::fs::create_dir_all(data_dir) {
        Ok(_) => {
            let root: EngineRoot = match EngineRoot::builder()
                .location(
                    data_dir
                        .to_str()
                        .expect("Data directory is made up of invalid UTF-8 characters")
                        .to_string()
                        .into(),
                )
                .build()
            {
                Ok(root) => root,
                Err(e) => panic!("{}", e),
            };

            let current_engine: usize = 0;

            eframe::run_native(
                "Dropfunk",
                options,
                Box::new(|_cc| {
                    Ok(Box::new(Application {
                        root,
                        current_engine,
                        current_version: None,
                        current_modpack: None,
                    }))
                }),
            )
        }
        Err(e) => panic!("{}", e),
    }
}

struct Application {
    root: EngineRoot,
    current_engine: usize,
    current_version: Option<String>,
    current_modpack: Option<usize>,
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        use egui::{containers::scroll_area::ScrollBarVisibility, *};
        if !self.root.engines.is_empty() {
            SidePanel::left("sidebar")
                .exact_width(40.)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.take_available_space();
                    ScrollArea::vertical()
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |_| {});
                });
            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let engine: &Engine = &self.root.engines[self.current_engine];
                    ui.heading(&self.root.display_names[self.current_engine]);
                    if let Some(authors) = &self.root.authors[self.current_engine] {
                        ui.label(authors);
                    }
                    let source: Option<Hyperlink> = self.root.source_codes[self.current_engine]
                        .as_ref()
                        .map(|source_code| {
                            Hyperlink::from_label_and_url("Source Code", source_code)
                        });
                    let website: Option<Hyperlink> = self.root.websites[self.current_engine]
                        .as_ref()
                        .map(|website| Hyperlink::from_label_and_url("Website", website));
                    if source.is_some() || website.is_some() {
                        ui.horizontal(|ui| {
                            if let Some(link) = source {
                                ui.add(link);
                            }
                            if let Some(link) = website {
                                ui.add(link);
                            }
                        });
                    }
                    ui.separator();
                    ui.heading("Modpacks");
                    Frame::NONE.fill(Color32::from_gray(0)).show(ui, |ui| {
                        ui.push_id(1, |ui| {
                            ScrollArea::vertical().max_height(320.).show(ui, |ui| {
                                ui.take_available_width();
                                macro_rules! make_modpack {
                                    ($x: expr) => {
                                        Frame::NONE.fill(Color32::from_rgb(0, 0, 0)).show(
                                            ui,
                                            |ui| {
                                                ui.heading(&*$x.display_name);
                                                ui.label(&*$x.version);
                                                ui.label(&*$x.brief);
                                            },
                                        );
                                    };
                                }
                                let mut iter = engine.modpacks.iter();
                                if let Some(modpack) = iter.next() {
                                    make_modpack!(modpack);
                                }
                                for modpack in iter {
                                    ui.separator();
                                    make_modpack!(modpack);
                                }
                            });
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.menu_button("Current Version", |ui| {
                            for version in &*engine.versions {
                                if ui.button(version).clicked() {
                                    self.current_version = Some(version.clone());
                                    warn!("Set current engine version to {}", version);
                                }
                            }
                        });
                        ui.menu_button("Current Modpack", |ui| {
                            for (index, modpack) in engine.modpacks.iter().enumerate() {
                                if ui.button(&*modpack.display_name).clicked() {
                                    self.current_modpack = Some(index);
                                    warn!("Set current modpack to {}", modpack.display_name);
                                }
                            }
                        });
                    });
                });
            });
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("No Engines");
                    ui.label("No engines have been installed. To use Dropfunk, please install an engine.");
                    ui.label("To install an engine:");
                    ui.label("1. Create a subdirectory under the engines directory. This subdirectory contains \
                        the versions of the engine, as well as metadata.");
                    ui.label("2. Create a subdirectory under the previous directory with the version number as \
                        the filename. Extract the downloaded engine into this subdirectory.");
                    ui.label("3. If any modpacks were distributed alongside the downloaded engine, move them into a \
                        specialized mods subdirectory under the main engine directory in order to keep the modpacks in-tact.");
                    ui.label("4. Dropfunk will create a template JSON file that describes the engine's metadata. You \
                        are free to configure this file as you please.");
                    ui.label("5. At this point, you should be able to start using Dropfunk!");
                    if ui.button("Open Engines Directory").clicked() {
                        open::that(&*self.root.location).unwrap_or_else(|_| {
                            panic!(
                                "No openers could open directory location {}",
                                self.root
                                    .location
                                    .to_str()
                                    .expect("Engine root location contains non-UTF8 characters"),
                            )
                        });
                    }
                });
            });
        }
    }
}
