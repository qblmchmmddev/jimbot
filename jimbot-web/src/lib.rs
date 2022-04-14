use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Stream};
use jimbot::jimbot::Jimbot;
use ringbuf::{Producer, RingBuffer};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct JimbotWeb {
    jimbot: Jimbot,
    _stream: Stream,
    audio_producer: Producer<f32>,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
impl JimbotWeb {
    #[wasm_bindgen(constructor)]
    pub fn new(cartridge_bytes: Box<[u8]>) -> JimbotWeb {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = RingBuffer::<f32>::new(44100);
        let (audio_producer, mut buff_con) = buffer.split();
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                let x = buff_con.pop().unwrap_or(0.) * 0.05 as f32;
                *sample = x;
            }
        };
        let stream = device
            .build_output_stream(&config, output_data_fn, |err| {
                panic!("Error build output stream: {:?}", err);
            })
            .unwrap();

        stream.play().expect("Cannot play audio");
        let jimbot = Jimbot::new_with_cartridge_bytes(cartridge_bytes.to_vec());
        Self {
            jimbot,
            _stream: stream,
            audio_producer,
        }
    }

    pub fn run(&mut self, lcd_data: &mut [u8]) {
        const M_CYCLE_PER_FRAME: f32 = (4194304.0 / 4.0) / 60.0;
        let mut current_cycle = 0.0f32;
        while current_cycle < M_CYCLE_PER_FRAME {
            self.jimbot.run();
            current_cycle += 1.;
        }
        self.audio_producer.push_slice(self.jimbot.get_sound_data().as_slice());
        let pixels = self.jimbot.ppu().lcd();
        for y in 0..144 {
            for x in 0..160 {
                let pixel = pixels[x][y];
                lcd_data[(y * 160) + x] = pixel;
            }
        }
    }

    pub fn joypad_release(&mut self, key: jimbot::mmu::joypad::Key) {
        self.jimbot.joypad_release(key); 
    }


    pub fn joypad_press(&mut self, key: jimbot::mmu::joypad::Key) {
        self.jimbot.joypad_press(key);
    }
}
