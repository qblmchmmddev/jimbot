use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Stream};
use jimbot::jimbot::Jimbot;
use ringbuf::{Producer, RingBuffer};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
use wasm_bindgen::closure::Closure;
use jimbot::saver::Saver;

#[wasm_bindgen]
pub struct JimbotWeb {
    jimbot: Jimbot,
    _stream: Stream,
    audio_producer: Producer<f32>,
}

struct WebSaver {
    // store: web_sys::Storage,
}

static mut SAVE_DATA: Vec<u8> = Vec::new();
static mut TITLE: String = String::new();

impl Saver for WebSaver {
    fn save(&self, title: String, data: Vec<u8>, at: u64) {
        unsafe {
            TITLE = title;
            SAVE_DATA = data;
        }
        // let store = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        // store.set_item(&title, &base64::encode(&data));
    }

    fn load(&self, title: String) -> Option<Vec<u8>> {
        let store = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        if let Some(base64data) = store.get_item(&title).unwrap() {
            if let Ok(data) = base64::decode(base64data) {
                unsafe { SAVE_DATA = data.clone(); }
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let cb = Closure::wrap(Box::new(||{
        let store = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        unsafe {
            store.set_item(&TITLE, &base64::encode(&SAVE_DATA)).unwrap();
        }
    }) as Box<dyn FnMut()>);
    window.set_onbeforeunload(Some(cb.as_ref().unchecked_ref()));
    window.set_onpagehide(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
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
        let jimbot = Jimbot::new_with_cartridge_bytes(Some(Box::new(WebSaver{})), cartridge_bytes.to_vec());
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
