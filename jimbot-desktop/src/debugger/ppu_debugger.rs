use bevy::prelude::{Commands, Res, ResMut};
use bevy_egui::egui::{RichText, ScrollArea, TextStyle, Window};
use bevy_egui::{EguiContext, EguiContexts};
use jimbot::jimbot::Jimbot;
use pretty_hex::{config_hex, HexConfig};

use crate::JimbotResource;

// pub struct CpuDebugger {
//     pub instructions: Vec<String>,
// }

// impl Default for CpuDebugger {
//     fn default() -> Self {
//         Self {
//             instructions: Vec::new(),
//         }
//     }
// }

// pub fn setup_ppu_debugger(mut commands: Commands) {
//     commands.insert_resource(CpuDebugger::default())
// }

pub fn run_ppu_debugger(
    jimbot: Res<JimbotResource>,
    // mut cpu_debugger: ResMut<CpuDebugger>,
    mut egui_context: EguiContexts,
) {
    let jimbot = &jimbot.0;
    Window::new("PPU")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.collapsing("LCD", |ui| {
                    ui.set_max_height(250.);
                    ScrollArea::vertical().show(ui, |ui| {
                        let lcd = jimbot.ppu().lcd();
                        let mut lcd_flat = [0u8; 144 * 160];
                        for y in 0..144usize {
                            for x in 0..160usize {
                                let index = y * 160 + x;
                                lcd_flat[index] = lcd[x][y];
                            }
                        }
                        ui.label(
                            RichText::new(config_hex(
                                &lcd_flat,
                                HexConfig {
                                    title: false,
                                    ascii: false,
                                    width: 160,
                                    group: 160,
                                    chunk: 1,
                                },
                            ))
                            .text_style(TextStyle::Small),
                        );
                    });
                });
            });
        });
}
