use eframe::egui;
use rfd::FileDialog;
use tiles::{
    Tilemap,
    Tileset,
    Tile
};

#[derive(Default)]
struct Temp {
    tileset_name: String,
    tileset_directory: String,
    tileset_tile_count: u32,
    tileset_tile_count_str: String,
    tileset_tile_size: u32,
    tileset_tile_size_str: String,
    tilemap_width: usize,
    tilemap_height: usize,
    tilemap_width_str: String,
    tilemap_height_str: String
}

#[derive(PartialEq, Clone, Copy)]
enum TilesetSelection {
    First = 0,
    Second = 1,
    Third = 2,
    None
}

pub struct App {
    tilesets: Vec<Tileset>,
    tileset_window_open: bool,
    tileset_selection: TilesetSelection,
    tile_selection: Option<usize>,
    tilemap: Option<Tilemap>,
    tilemap_image: Option<Vec<u8>>,
    new_tilemap_window_open: bool,
    temp: Temp
}

impl Default for App {
    fn default() -> Self {
        App {
            tilesets: Vec::new(),
            tileset_window_open: false,
            tileset_selection: TilesetSelection::None,
            tile_selection: None,
            tilemap: None,
            tilemap_image: None,
            new_tilemap_window_open: false,
            temp: Temp::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                // Open Tileset Window
                egui::Window::new("Open Tileset")
                    .open(&mut self.tileset_window_open)
                    .show(ctx, |ui| {
                        egui::Grid::new("tileset-window-grid")
                            .max_col_width(200.0)
                            .show(ui, |ui| {
                                ui.label("Name");
                                ui.text_edit_singleline(&mut self.temp.tileset_name);
                                ui.end_row();

                                ui.label("Tile Count");
                                if ui.text_edit_singleline(&mut self.temp.tileset_tile_count_str).changed() {
                                    if let Ok(count) = self.temp.tileset_tile_count_str.parse() {
                                        self.temp.tileset_tile_count = count;
                                    }
                                }
                                ui.end_row();

                                ui.label("Tile Size");
                                if ui.text_edit_singleline(&mut self.temp.tileset_tile_size_str).changed() {
                                    if let Ok(size) = self.temp.tileset_tile_size_str.parse() {
                                        self.temp.tileset_tile_size = size;
                                    }
                                }
                                ui.end_row();

                                ui.label("Directory");
                                if self.temp.tileset_directory.as_str() == "" {
                                    if ui.button("Open Directory").clicked() {
                                        self.temp.tileset_directory = FileDialog::new()
                                            .set_directory("~")
                                            .pick_folder()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .into();
                                    }
                                } else {
                                    ui.label(&self.temp.tileset_directory);
                                }
                                ui.end_row();
                            });
                        ui.separator();

                        ui.vertical_centered(|ui| {
                            if ui.button("Add Tileset").clicked() {
                                self.tilesets.push(
                                    Tileset::new(
                                        self.temp.tileset_name.clone(),
                                        self.temp.tileset_directory.clone(),
                                        self.temp.tileset_tile_count,
                                        self.temp.tileset_tile_size
                                    )
                                );

                                self.temp.tileset_name = String::new();
                                self.temp.tileset_directory = String::new();
                                self.temp.tileset_tile_count = 0;
                                self.temp.tileset_tile_count_str = String::new();
                                self.temp.tileset_tile_size = 0;
                                self.temp.tileset_tile_size_str = String::new();
                            }
                        })
                    });

                // New Tilemap Window
                egui::Window::new("New Tilemap")
                    .open(&mut self.new_tilemap_window_open)
                    .show(ctx, |ui| {
                        egui::Grid::new("tilemap-window-grid")
                            .max_col_width(200.0)
                            .show(ui, |ui| {
                                ui.label("Width");
                                if ui.text_edit_singleline(&mut self.temp.tilemap_width_str).changed() {
                                    if let Ok(width) = self.temp.tilemap_width_str.parse() {
                                        self.temp.tilemap_width = width;
                                    }
                                }
                                ui.end_row();

                                ui.label("Height");
                                if ui.text_edit_singleline(&mut self.temp.tilemap_height_str).changed() {
                                    if let Ok(height) = self.temp.tilemap_height_str.parse() {
                                        self.temp.tilemap_height = height;
                                    }
                                }
                                ui.end_row();
                            });
                        ui.separator();

                        ui.vertical_centered(|ui| {
                            if ui.button("Create Tilemap").clicked() {
                                if self.tileset_selection != TilesetSelection::None {
                                    (self.tilemap, self.tilemap_image) = create_tilemap(
                                        self.temp.tilemap_width,
                                        self.temp.tilemap_height,
                                        self.tilesets[self.tileset_selection as usize].tile_size as usize
                                    );
                                }
                            }
                        })
                    });

                // Top Menu Bar
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Tilemap", |ui| {
                        if let Some(tilemap) = &self.tilemap {
                            if ui.button("New").clicked() {
                                self.new_tilemap_window_open ^= true;
                            }
                            ui.button("Open");
                            ui.button("Save");
                        } else {
                            if ui.button("New").clicked() {
                                self.new_tilemap_window_open ^= true;
                            }
                            ui.button("Open");
                        }
                    });

                    if ui.button("Open Tileset").clicked() {
                        self.tileset_window_open ^= true;
                    }
                });
                ui.separator();

                let canvas_area_size = egui::Vec2::new(ui.available_width(), ui.available_height() / 1.5);
                ui.add_sized(canvas_area_size, |ui: &mut egui::Ui| {
                    if let Some(tilemap) = &self.tilemap {
                        draw_canvas(
                            tilemap.tiles[0].len(),
                            tilemap.tiles.len(),
                            *&self.tilesets[self.tileset_selection as usize].tile_size as usize,
                            &self.tilemap_image,
                            ctx,
                            ui
                        )
                    } else {
                        draw_canvas(0, 0, 0, &self.tilemap_image, ctx, ui)
                    }
                });

                ui.separator();
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true), |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Tilesets");
                        let mut tileset_iter = self.tilesets.iter();

                        if let Some(tileset) = tileset_iter.next() {
                            if ui.selectable_value(&mut self.tileset_selection, TilesetSelection::First, &tileset.name).clicked() {
                                self.tile_selection = None;
                            }
                        }
                        if let Some(tileset) = tileset_iter.next() {
                            if ui.selectable_value(&mut self.tileset_selection, TilesetSelection::Second, &tileset.name).clicked() {
                                self.tile_selection = None;
                            }
                        }
                        if let Some(tileset) = tileset_iter.next() {
                            if ui.selectable_value(&mut self.tileset_selection, TilesetSelection::Third, &tileset.name).clicked() {
                                self.tile_selection = None;
                            }
                        }
                    });
                    ui.separator();

                    if self.tileset_selection != TilesetSelection::None {
                        ui.vertical(|ui| {
                            ui.heading(&self.tilesets[self.tileset_selection as usize].name);
                            ui.label(format!("Tile Count: {}", &self.tilesets[self.tileset_selection as usize].tiles.len()));
                            ui.label(format!("Tile Size: {}", &self.tilesets[self.tileset_selection as usize].tile_size));
                        });
                        
                        egui::ScrollArea::vertical()
                            .show(ui, |ui| {
                                for i in 0..self.tilesets[self.tileset_selection as usize].tiles.len() {
                                    let tile_size = self.tilesets[self.tileset_selection as usize].tile_size as usize;
                                    let image = egui::ColorImage::from_rgba_unmultiplied(
                                        [tile_size, tile_size],
                                        &self.tilesets[self.tileset_selection as usize].tiles[i]
                                    );
                                    let texture = ctx.load_texture(
                                        format!("{i}"),
                                        image,
                                        egui::TextureOptions::default()
                                    );
                                    let image_button = {
                                        let image = egui::Image::new((texture.id(), texture.size_vec2()))
                                            .fit_to_exact_size(egui::Vec2::new(50.0, 50.0));
                                        let mut selected = false;
                                        for j in 0..self.tilesets[self.tileset_selection as usize].tiles.len() {
                                            if let Some(selection) = self.tile_selection {
                                                if selection == j && j == i {
                                                    selected = true;
                                                }
                                            }
                                        }

                                        egui::ImageButton::new(image)
                                            .selected(selected)
                                    };

                                    if ui.add(image_button).clicked() {
                                        self.tile_selection = Some(i);
                                    }
                                }
                            });
                    }
                });
            });
    }
}

fn draw_canvas(width: usize, height: usize, tile_size: usize, tilemap_image: &Option<Vec<u8>>, ctx: &egui::Context, ui: &mut egui::Ui) -> egui::Response {
    let size = egui::Vec2::new((width * tile_size) as f32, (height * tile_size) as f32);
    egui::Frame::canvas(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(50, 50, 50, 255))
        .show(ui, |ui| {
            if let Some(tilemap_image) = tilemap_image {
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [width * tile_size, height * tile_size],
                    tilemap_image
                );
                let texture = ctx.load_texture(
                    "canvas",
                    color_image,
                    egui::TextureOptions::default()
                );

                ui.add(egui::Image::new((texture.id(), texture.size_vec2()))
                    .fit_to_exact_size(ui.available_size()))
            } else {
                ui.label("No Tilemap")
            }
        }).response
}

fn create_tilemap(width: usize, height: usize, tile_size: usize) -> (Option<Tilemap>, Option<Vec<u8>>) {
    let mut tilemap = Tilemap::default();
    tilemap.tiles.resize_with(height, || Vec::new());
    for row in tilemap.tiles.iter_mut() {
        row.resize_with(width, || Tile::default());
    }

    let mut tilemap_image: Vec<u8> = Vec::new();
    tilemap_image.resize_with(width * tile_size * height * tile_size * 4, || 0);
    for pixel in tilemap_image.chunks_exact_mut(4) {
        pixel[3] = 255;
    }

    (Some(tilemap), Some(tilemap_image))
}
