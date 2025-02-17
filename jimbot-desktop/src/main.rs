mod debugger;

use std::borrow::BorrowMut;

use crate::debugger::cpu_debugger::{run_cpu_debugger, setup_cpu_debugger, CpuDebugger};
use crate::debugger::lcd_debugger::{run_lcd_debugger, setup_lcd_debugger};
use crate::debugger::mmu_debugger::run_mmu_debugger;
use crate::debugger::ppu_debugger::run_ppu_debugger;
use crate::debugger::setup_debugger;
use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use bevy::prelude::{Commands, Image, KeyCode, Res, ResMut, Resource};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::SpriteBundle;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use jimbot::cpu::instruction::Instruction;
use jimbot::cpu::op::Op;
use jimbot::cpu::registers::R16;
use jimbot::jimbot::Jimbot;
use jimbot::mmu::joypad;
use ringbuf::{Producer, RingBuffer};

#[derive(Resource)]
pub struct BuffProducer(Producer<f32>);

#[derive(Resource)]
pub struct JimbotResource(Jimbot);

fn main() {
    let host = cpal::default_host();
    let output_device = host.default_output_device().unwrap();

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };

    let buffer = RingBuffer::<f32>::new(44100);
    let (buff_prod, mut buff_con) = buffer.split();
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data {
            let x = buff_con.pop().unwrap_or(0.) * 0.05 as f32;
            *sample = x;
        }
    };
    #[cfg(not(target_os = "windows"))]
    let _output_stream = {
        let output_stream = output_device
            .build_output_stream(&config, output_data_fn, |err| {
                eprintln!("Error build output stream: {:?}", err);
            })
            .unwrap();
        output_stream.play().expect("Cannot play audio");
        output_stream
    };

    App::new()
        .insert_resource(BuffProducer(buff_prod))
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Jimbot".to_string(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        resolution: (160. * 5., 144. * 5.).into(),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        resize_constraints: Default::default(),
                        resizable: false,
                        decorations: true,
                        mode: WindowMode::Windowed,
                        transparent: false,
                        #[cfg(target_arch = "wasm32")]
                        canvas: None,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin)
        .insert_resource(JimbotResource(Jimbot::default()))
        .add_systems(
            Startup,
            (
                setup,
                // setup_cpu_debugger,
                // setup_lcd_debugger,
            ),
        )
        .add_systems(
            Update,
            (
                run_jimbot,
                // run_mmu_debugger,
                // run_cpu_debugger,
                // run_lcd_debugger,
                // run_ppu_debugger,
            ),
        )
        // .add_systems(Update, run_jimbot)
        // .add_system(run_mmu_debugger)
        // .add_system(run_cpu_debugger.before("run_jimbot"))
        // .add_system(run_lcd_debugger.after("run_jimbot"))
        // .add_system(run_ppu_debugger.after("run_jimbot"))
        .run();
}

#[derive(Resource)]
pub struct Display {
    pub image: Handle<Image>,
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = Image::new_fill(
        Extent3d {
            width: 160,
            height: 144,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0x0; 160 * 144 * 4],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );

    let image = images.add(image);

    commands.insert_resource(Display {
        image: image.clone(),
    });
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: image,
        transform: bevy::prelude::Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(5.0, 5.0, 1.0),
        },
        ..Default::default()
    });
}

// fn new_window(mut create_window_event: EventWriter<CreateWindow>) {
//     let window_id = WindowId::new();
//     create_window_event.send(CreateWindow {
//         id: window_id,
//         descriptor: WindowDescriptor {
//             width: 160.0 * 2.,
//             height: 144.0 * 2.,
//             position: None,
//             resize_constraints: Default::default(),
//             scale_factor_override: None,
//             title: "Second Window".to_string(),
//             vsync: true,
//             resizable: true,
//             decorations: true,
//             cursor_visible: true,
//             cursor_locked: false,
//             mode: WindowMode::Windowed,
//             transparent: false,
//         },
//     })
// }

fn run_jimbot(
    mut jimbot: ResMut<JimbotResource>,
    mut time: Res<Time>,
    display: Res<Display>,
    keys: Res<ButtonInput<KeyCode>>,
    mut images: ResMut<Assets<Image>>,
    mut audio_producer: ResMut<BuffProducer>,
) {
    let jimbot = jimbot.0.borrow_mut();
    let audio_producer = audio_producer.0.borrow_mut();
    const M_CYCLE_PER_FRAME: f32 = (4194304.0 / 4.0) / 60.0;
    let mut current_cycle = 0.0f32;

    if keys.pressed(KeyCode::KeyW) {
        jimbot.joypad_press(joypad::Key::Up)
    } else {
        jimbot.joypad_release(joypad::Key::Up)
    }
    if keys.pressed(KeyCode::KeyA) {
        jimbot.joypad_press(joypad::Key::Left)
    } else {
        jimbot.joypad_release(joypad::Key::Left)
    }
    if keys.pressed(KeyCode::KeyS) {
        jimbot.joypad_press(joypad::Key::Down)
    } else {
        jimbot.joypad_release(joypad::Key::Down)
    }
    if keys.pressed(KeyCode::KeyD) {
        jimbot.joypad_press(joypad::Key::Right)
    } else {
        jimbot.joypad_release(joypad::Key::Right)
    }
    if keys.pressed(KeyCode::KeyK) {
        jimbot.joypad_press(joypad::Key::A)
    } else {
        jimbot.joypad_release(joypad::Key::A)
    }
    if keys.pressed(KeyCode::KeyJ) {
        jimbot.joypad_press(joypad::Key::B)
    } else {
        jimbot.joypad_release(joypad::Key::B)
    }
    if keys.pressed(KeyCode::KeyB) {
        jimbot.joypad_press(joypad::Key::Start)
    } else {
        jimbot.joypad_release(joypad::Key::Start)
    }
    if keys.pressed(KeyCode::KeyV) {
        jimbot.joypad_press(joypad::Key::Select)
    } else {
        jimbot.joypad_release(joypad::Key::Select)
    }

    while current_cycle < M_CYCLE_PER_FRAME {
        // println!("Tima: {}", jimbot.mmu().get(0xFF04));
        jimbot.run();
        current_cycle += 1.;
        // let pc = jimbot.cpu().registers().get16(R16::PC);
        // if jimbot.error_message().is_none() {// && (pc >= 0x348) && (pc <= 0x38A) {// && pc < 0xCB80) {
        //     let ly = jimbot.mmu().ly();
        //     let (op, p1, p2) = jimbot.cpu().instruction().ins();
        //     let af = jimbot.cpu().registers().get16(R16::AF);
        //     let bc = jimbot.cpu().registers().get16(R16::BC);
        //     let de = jimbot.cpu().registers().get16(R16::DE);
        //     let hl = jimbot.cpu().registers().get16(R16::HL);
        //     let sp = jimbot.cpu().registers().get16(R16::SP);
        //     match (op, p1, p2) {
        //         // (Op::Dcd, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}, IME:{}, IF:{:#08b}, IE:{:#08b}] ly:{}", pc, op, p1, p2, af, bc, de, hl, sp, jimbot.cpu().ime(), jimbot.mmu().get(0xFF0F), jimbot.mmu().get(0xFFFF), ly),
        //         (Op::Dcd, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}] ly:{} lyc:{} stat:{:#010b} if:{:#010b} ie:{:#010b} ime:{}", pc, op, p1, p2, af, bc, de, hl, sp, jimbot.mmu().ly(), jimbot.mmu().lyc(), jimbot.mmu().get(0xFF41), jimbot.mmu().get(0xFF0F), jimbot.mmu().get(0xFFFF), jimbot.cpu().ime()),
        //         // (Op::DcdCB, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}]", pc, op, p1, p2, af, bc, de, hl, sp),
        //         (_, _, _) => {}//println!("\t-----> {:?} {:?} {:?}", op, p1, p2),
        //     }
        //     //     cpu_debugger.instructions.push(ins);
        // }
    }
    let image = images.get_mut(&display.image).unwrap();
    let lcd = jimbot.ppu().lcd();
    let byte_per_row = 160 * 4;
    for lcd_y in 0..144 {
        for lcd_x in 0..160 {
            let px: u8 = lcd[lcd_x][lcd_y].into();
            if px == 0x10 {
                panic!()
            }
            let color: u32 = match px {
                // 0b00 => 0x84d07d,
                // 0b01 => 0x5e785d,
                // 0b10 => 0x3e4943,
                // _ => 0x252b25,
                0b00 => 0xE0F8D0,
                0b01 => 0x88C070,
                0b10 => 0x346856,
                _ => 0x081820,
            };
            let offset_y = (lcd_y * byte_per_row) as usize;
            let offset_x = (lcd_x as usize * 4); //(x_tile * 8 * 4);
            let index = (offset_x + offset_y) as usize;
            image.data[index + 0] = ((color >> 16) & 0xFF) as u8;
            image.data[index + 1] = ((color >> 8) & 0xFF) as u8;
            image.data[index + 2] = (color & 0xFF) as u8;
            image.data[index + 3] = 0xFF;
        }
    }
    let sound_data = jimbot.get_sound_data();
    audio_producer.push_slice(sound_data.as_slice());
}
