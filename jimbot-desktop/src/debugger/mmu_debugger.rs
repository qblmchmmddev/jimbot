use bevy::prelude::{Res, ResMut};
use bevy_egui::egui::{ScrollArea, Slider, Window};
use bevy_egui::EguiContext;
use pretty_hex::{config_hex, HexConfig};
use jimbot::jimbot::Jimbot;

pub fn run_mmu_debugger(
    mut jimbot: ResMut<Jimbot>,
    mut egui_context: ResMut<EguiContext>,
) {
    Window::new("MMU")
        .auto_sized()
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.collapsing("Boot rom (0x0000 - 0x0100)", |ui| {
                    ui.label(config_hex(jimbot.mmu().boot_rom(), HexConfig {
                        title: true,
                        ascii: false,
                        width: 16,
                        group: 4,
                        chunk: 1,
                    }))
                });
                if let Some(cart) = jimbot.mmu().cartridge() {
                    ui.collapsing("Cartridge (0x0000 - 0x7FFF)", |ui| {
                        ui.set_max_height(250.);
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.label(config_hex(cart.data(), HexConfig {
                                title: true,
                                ascii: false,
                                width: 16,
                                group: 4,
                                chunk: 1,
                            }))
                        });
                    });
                }
                ui.collapsing("VRAM (0x8000 - 0x9FFF)", |ui| {
                    ui.set_max_height(250.);
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.label(config_hex(jimbot.mmu().vram(), HexConfig {
                            title: true,
                            ascii: false,
                            width: 16,
                            group: 4,
                            chunk: 1,
                        }));
                    });
                });
                ui.label(format!("LY:{}", jimbot.mmu().ly()));
                let bgp: u8 = jimbot.mmu().bgp().into();
                ui.label(format!("BPG:{:08b}", bgp));
                ui.add(
                    Slider::new(jimbot.test(), -16..=16)
                );
                // ui.collapsing("APU (0x???? - 0x????)", |ui| {
                //     ui.set_max_height(250.);
                //     ScrollArea::vertical().show(ui, |ui| {
                //         ui.label(
                //             format!("\
                //             NR11:{:#010b} NR12:{:#010b}\n\
                //             NR50:{:#010b} NR51:{:#010b} NR52:{:#010b}",
                //                     jimbot.mmu().apu().nr11(),
                //                     jimbot.mmu().apu().nr12(),
                //                     jimbot.mmu().apu().nr50(),
                //                     jimbot.mmu().apu().nr51(),
                //                     jimbot.mmu().apu().nr52(),
                //             )
                //         );
                //     });
                // });

                ui.collapsing("WRAM (0xC000 - 0xDFFF)", |ui| {
                    ui.set_max_height(250.);
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.label(config_hex(jimbot.mmu().wram(), HexConfig {
                            title: true,
                            ascii: false,
                            width: 16,
                            group: 4,
                            chunk: 1,
                        }));
                    });
                });

                ui.collapsing("HRAM (0xFF80 - 0xFFFE)", |ui| {
                    ui.set_max_height(250.);
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.label(config_hex(jimbot.mmu().hram(), HexConfig {
                            title: true,
                            ascii: false,
                            width: 16,
                            group: 4,
                            chunk: 1,
                        }));
                    });
                });
            });
        });
}