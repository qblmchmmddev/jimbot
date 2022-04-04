use bevy::prelude::{Commands, Res, ResMut};
use bevy_egui::egui::{ScrollArea, TextStyle, Window};
use bevy_egui::EguiContext;
use pretty_hex::{config_hex, HexConfig};
use jimbot::cpu::registers::R16;
use jimbot::jimbot::Jimbot;

pub struct CpuDebugger {
    pub instructions: Vec<String>,
}

impl Default for CpuDebugger {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

pub fn setup_cpu_debugger(mut commands: Commands) {
    commands.insert_resource(CpuDebugger::default())
}


pub fn run_cpu_debugger(
    mut jimbot: ResMut<Jimbot>,
    mut cpu_debugger: ResMut<CpuDebugger>,
    mut egui_context: ResMut<EguiContext>,
) {
    // let op = jimbot.cpu().cycle_op();
    // if jimbot.error_message().is_none() {
    //     match (op, p1, p2) {
    //         (Op::UnfetchOp, P::UnfetchP, P::UnfetchP) => {},
    //         (_, _, P::UnfetchU16(_, _)) => {},
    //         (_, _, P::U16(u16)) => cpu_debugger.instructions.push(format!("{:#06X} {:<3} {:<9} U16({:#06X})", pc, format!("{:?}",op), format!("{:?}",p1), u16)),
    //         _ => cpu_debugger.instructions.push(format!("{:#06X} {:<3} {:<9} {:?}", pc, format!("{:?}",op), format!("{:?}",p1), p2)),
    //     };
    // }
    Window::new("CPU")
        .auto_sized()
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                if let Some(error) = jimbot.error_message() {
                    ui.label(format!("Error: {}", error));
                    if (ui.button("Next")).clicked() {
                        jimbot.clear_error()
                    }
                }
                ui.group(|ui| {
                    ui.label("Instructions:");
                    let text_style = TextStyle::Body;
                    let row_height = ui.text_style_height(&text_style);
                    ui.indent("", |ui| {
                        ui.set_max_height(250.);
                        ScrollArea::vertical().stick_to_bottom().show_rows(
                            ui,
                            row_height,
                            cpu_debugger.instructions.len(),
                            |ui, range| {
                                for row in range {
                                    ui.label(&cpu_debugger.instructions[row]);
                                }
                            },
                        );
                    });
                });
                ui.group(|ui| {
                    ui.label("Registers:");
                    ui.indent("", |ui| {
                        ui.label(format!(
                            "AF:{:#06X} BC:{:#06X} DE:{:#06X}\nHL:{:#06X} SP:{:#06X} PC:{:#06X}",
                            jimbot.cpu().registers().get16(R16::AF),
                            jimbot.cpu().registers().get16(R16::BC),
                            jimbot.cpu().registers().get16(R16::DE),
                            jimbot.cpu().registers().get16(R16::HL),
                            jimbot.cpu().registers().get16(R16::SP),
                            jimbot.cpu().registers().get16(R16::PC),
                        ));
                    });
                });
            });
        });
}