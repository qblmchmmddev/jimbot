use bevy::prelude::ResMut;
use bevy_egui::egui::{FontData, FontDefinitions, FontFamily, TextStyle};
use bevy_egui::EguiContext;

pub mod mmu_debugger;
pub mod cpu_debugger;
pub mod lcd_debugger;
pub mod ppu_debugger;

pub fn setup_debugger(
    mut ctx: ResMut<EguiContext>,
) {
    let mut font = FontDefinitions::default();

    font.font_data.insert(
        "mn".to_owned(),
        FontData::from_static(include_bytes!("../fonts/mn.ttf")),
    );

    font.families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "mn".to_owned());
    font.families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "mn".to_owned());

    ctx.ctx_mut()
        .set_fonts(font);

    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles.get_mut(&TextStyle::Small).unwrap().size = 1.0;

    ctx.ctx_mut()
        .set_style(style);
}