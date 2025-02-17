use bevy::asset::io::Reader;
use bevy::prelude::{Assets, Commands, Handle, Image, ResMut, Resource};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::egui::{ScrollArea, TextureId};
use bevy_egui::{egui, EguiContext, EguiContexts};
use jimbot::jimbot::Jimbot;
use pretty_hex::{config_hex, pretty_hex, HexConfig};

use crate::JimbotResource;

#[derive(Resource)]
pub struct LcdDebugger {
    color0: [u8; 3],
    color1: [u8; 3],
    color2: [u8; 3],
    color3: [u8; 3],
    image: Handle<Image>,
    texture: TextureId,
}

pub fn setup_lcd_debugger(
    mut command: Commands,
    mut ctx: EguiContexts,
    mut images: ResMut<Assets<Image>>,
) {
    let image = Image::new(
        Extent3d {
            width: 160,
            height: 144,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0xFF; 160 * 144 * 4],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );

    let image = images.add(image);

    let texture = ctx.add_image(image.clone());

    command.insert_resource(LcdDebugger {
        image: image.clone(),
        texture,
        color0: [0x84, 0xd0, 0x7d],
        color1: [0x5e, 0x78, 0x5d],
        color2: [0x3e, 0x49, 0x43],
        color3: [0x25, 0x2b, 0x25],
    });
}

pub fn run_lcd_debugger(
    mut ctx: EguiContexts,
    jimbot: ResMut<JimbotResource>,
    mut lcd_viewer_debug: ResMut<LcdDebugger>,
    mut images: ResMut<Assets<Image>>,
) {
    let jimbot = &jimbot.0;
    let image = images.get_mut(&lcd_viewer_debug.image).unwrap();
    let lcd = jimbot.ppu().lcd();
    // let sprites = gembot.memory.get_sprites();
    let byte_per_row = 160 * 4;
    // 80 10 58

    for lcd_y in 0..144 {
        for lcd_x in 0..160 {
            let px: u8 = lcd[lcd_x][lcd_y].into();
            if px == 0x10 {
                panic!()
            }
            let color = match px {
                0b00 => lcd_viewer_debug.color0,
                0b01 => lcd_viewer_debug.color1,
                0b10 => lcd_viewer_debug.color2,
                _ => lcd_viewer_debug.color3,
            };
            // let color: u32 = match px {
            // 0b00 => 0xE0F8D0,
            // 0b01 => 0x88C070,
            // 0b10 => 0x346856,
            // _ => 0x081820,
            // 0b00 => 0x0,
            // 0b01 => 0xff0000,
            // 0b10 => 0x00ff00,
            // _ => 0x0000ff,
            // 0b00 => 0x9BBC0F,
            // 0b01 => 0x8BAC0F,
            // 0b10 => 0x306230,
            // _ => 0x0F380F,
            // 0b00 => 0x84d07d,
            // 0b01 => 0x5e785d,
            // 0b10 => 0x3e4943,
            // _ => 0x252b25,
            // 0b00 => 0x84d07d,
            // 0b01 => 0x5e785d,
            // 0b10 => 0x3e4943,
            // _ => 0x252b25,
            // };
            let offset_y = (lcd_y * byte_per_row) as usize;
            let offset_x = (lcd_x as usize * 4); //(x_tile * 8 * 4);
            let index = (offset_x + offset_y) as usize;
            // image.data[index + 0] = ((color >> 16) & 0xFF) as u8;
            // image.data[index + 1] = ((color >> 8) & 0xFF) as u8;
            // image.data[index + 2] = (color & 0xFF) as u8;
            // image.data[index + 3] = 0xFF;
            image.data[index + 0] = color[0];
            image.data[index + 1] = color[1];
            image.data[index + 2] = color[2];
            image.data[index + 3] = 0xFF;
        }
    }

    egui::Window::new("LCD").show(ctx.ctx_mut(), |ui| {
        // ScrollArea::vertical().show(ui, |ui|{
        //     ui.label(pretty_hex(gembot.memory.get_oam()))
        // });
        ui.image(egui::load::SizedTexture::new(
            lcd_viewer_debug.texture,
            egui::Vec2::new(160.0 * 4.0, 144.0 * 4.0),
        ));
        ui.horizontal(|ui| {
            ui.label("Color 0");
            ui.color_edit_button_srgb(&mut lcd_viewer_debug.color0);
            ui.label("Color 1");
            ui.color_edit_button_srgb(&mut lcd_viewer_debug.color1);
            ui.label("Color 2");
            ui.color_edit_button_srgb(&mut lcd_viewer_debug.color2);
            ui.label("Color 3");
            ui.color_edit_button_srgb(&mut lcd_viewer_debug.color3);
        });
    });
    //
    // egui::Window::new("Interrupt").show(ctx.ctx_mut(), |ui| {
    //     ui.vertical(|ui| {
    //         ui.label(format!("IME: {}", gembot.cpu.ime));
    //         ui.label(format!("IE: {}", gembot.memory.get_interrupt_enable()));
    //         ui.label(format!("IF: {}", gembot.memory.get_interrupt_flag()));
    //     });
    // });
    //
    // egui::Window::new("TIMER").show(ctx.ctx_mut(), |ui| {
    //     ui.vertical(|ui| {
    //         ui.label(format!("INTERNAL_DIV: {:#160b}", gembot.timer.internal_div_counter));
    //         ui.label(format!("IE: {}", gembot.memory.get_interrupt_enable()));
    //         ui.label(format!("IF: {}", gembot.memory.get_interrupt_flag()));
    //     });
    // });
}
