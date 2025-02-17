use egui::{Color32, RichText};

use crate::file_handler;

pub fn picker_ui(ui: &mut egui::Ui, picked_path: &mut Option<String>) {
    if ui.button("Open zip archive...").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            *picked_path = Some(path.display().to_string());
        }
    }

    if let Some(picked_path) = &picked_path {
        ui.horizontal(|ui| {
            ui.label("Picked file:");
            ui.monospace(picked_path);
        });

        if picked_path.ends_with(".zip") {
            ui.label(
                RichText::new("✔ File is a valid .zip archive")
                    .color(Color32::GREEN),
            );
            if ui.button("Unzip file").clicked() {
                if let Err(e) = file_handler::unzip_file(picked_path, None) {
                    ui.label(
                        RichText::new("⚠ Failed to unzip file ⚠")
                        .color(ui.visuals().warn_fg_color),
                    );
                }
            }
        } else {
            ui.label(
                RichText::new("⚠ Warning - selected file not a .zip! ⚠")
                    .color(ui.visuals().warn_fg_color),
            );
        }
    }
}