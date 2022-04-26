use std::sync::{Arc, Mutex};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Stream};
use jimbot::jimbot::Jimbot;
use ringbuf::{Producer, RingBuffer};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
use wasm_bindgen::closure::Closure;

#[wasm_bindgen]
pub struct JimbotWeb {
    jimbot: Arc<Mutex<Jimbot>>,
    _stream: Stream,
    audio_producer: Producer<f32>,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
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
        web_sys::console::log_1(&format!("Cart size: {}", cartridge_bytes.len()).into());
        let mut jimbot = Jimbot::new_with_cartridge_bytes(cartridge_bytes.to_vec());
        let cart = jimbot.cartridge().as_ref();
        web_sys::console::log_1(&format!("Cart loaded: {}", cart.is_some()).into());
        let cart = cart.expect("No Cartridge");
        web_sys::console::log_1(&format!("{:#?}", cart.metadata()).into());
        let title = cart.metadata().title().to_string();
        if let Some(save_data) = jimbot.save_data_mut() {
            let store = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            if let Some(base64data) = store.get_item(&title).unwrap() {
                if let Ok(data) = base64::decode(base64data) {
                    save_data.copy_from_slice(data.as_slice());
                    web_sys::console::log_1(&format!("Saved data loaded: {}", &title).into());
                }
            }
        }
        let window = web_sys::window().unwrap();
        let jimbot = Arc::new(Mutex::new(jimbot));
        let jimbot_cb = jimbot.clone();
        let cb = Closure::wrap(Box::new(move ||{
            let store = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            let jimbot = jimbot_cb.lock().unwrap();
            if let Some(save_data) = jimbot.save_data() {
                    store.set_item(jimbot.mmu().cartridge().as_ref().unwrap().metadata().title(), &base64::encode(save_data)).unwrap();
            }
        }) as Box<dyn FnMut()>);
        window.set_onbeforeunload(Some(cb.as_ref().unchecked_ref()));
        window.set_onpagehide(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
        Self {
            jimbot: jimbot.clone(),
            _stream: stream,
            audio_producer,
        }
    }

    pub fn run(&mut self, lcd_data: &mut [u8]) {
        const M_CYCLE_PER_FRAME: f32 = (4194304.0 / 4.0) / 60.0;
        let mut current_cycle = 0.0f32;
        let mut jimbot = self.jimbot.lock().unwrap();
        while current_cycle < M_CYCLE_PER_FRAME {
            jimbot.run();
            current_cycle += 1.;
        }
        self.audio_producer.push_slice(jimbot.get_sound_data().as_slice());
        let pixels = jimbot.ppu().lcd();
        for y in 0..144 {
            for x in 0..160 {
                let pixel = pixels[x][y];
                lcd_data[(y * 160) + x] = pixel;
            }
        }
    }

    pub fn joypad_release(&mut self, key: jimbot::mmu::joypad::Key) {
        self.jimbot.lock().unwrap().joypad_release(key);
    }


    pub fn joypad_press(&mut self, key: jimbot::mmu::joypad::Key) {
        self.jimbot.lock().unwrap().joypad_press(key);
    }
}
