use std::process::id;
use crate::mmu::MMU;
use crate::mmu::sprite::Sprite;
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::pixel_type::PixelType;
use crate::ppu::sprite_pixel_fetcher::SpritePixelFetcher;

#[derive(Debug)]
pub enum Step {
    WaitFifo { tile_pixel_row: [PixelType; 8] },
    FetchTileDataIndex,
    FetchTileDataLow { tile_data_index: u8 },
    FetchTileDataHi { tile_data_row_address_low: u16, tile_data_row_low: u8 },
    PushToFifo { tile_data_row_low: u8, tile_data_row_hi: u8 },
}

pub struct PixelFetcher {
    sprite_pixel_fetcher: SpritePixelFetcher,
    cycle_available: u8,
    current_step: Step,
    is_window_mode: bool,
    x_position_counter: u8,
    window_line_counter: u8,
}

impl Default for PixelFetcher {
    fn default() -> Self {
        Self {
            sprite_pixel_fetcher: Default::default(),
            cycle_available: 0,
            current_step: Step::FetchTileDataIndex,
            is_window_mode: false,
            x_position_counter: 0,
            window_line_counter: 0,
        }
    }
}

impl PixelFetcher {
    pub fn fetching_sprite(&self) -> bool {
        self.sprite_pixel_fetcher.need_step()
    }

    pub fn fetch_sprite(&mut self, sprite: Sprite, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        // println!("FSPRITE: {}", self.x_position_counter);
        self.sprite_pixel_fetcher.fetch(sprite, mmu, pixel_fifo);
    }

    pub fn fetch_window(&mut self, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        assert!(!self.sprite_pixel_fetcher.need_step(), "Should not start fetch window when sprite fetcher still in progress");
        self.x_position_counter = 0;
        self.is_window_mode = true;
        self.current_step = Step::FetchTileDataIndex;
        self.step(mmu, pixel_fifo);
        // println!("FSPRITE: {}", self.x_position_counter);
        // self.sprite_pixel_fetcher.fetch(sprite, mmu, pixel_fifo);
    }

    pub fn step(&mut self, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        // println!("STEP: {}", self.x_position_counter);
        if self.sprite_pixel_fetcher.need_step() {
            self.sprite_pixel_fetcher.step(mmu, pixel_fifo);
        } else {
            self.cycle_available += 1;
            match self.current_step {
                Step::FetchTileDataIndex => self.fetch_tile_data_index(mmu),
                Step::FetchTileDataLow { tile_data_index } => self.fetch_tile_data_low(tile_data_index, mmu),
                Step::FetchTileDataHi { tile_data_row_address_low, tile_data_row_low } => self.fetch_tile_data_hi(tile_data_row_address_low, tile_data_row_low, mmu),
                Step::PushToFifo { tile_data_row_low, tile_data_row_hi } => self.push_to_fifo(tile_data_row_low, tile_data_row_hi, mmu, pixel_fifo),
                Step::WaitFifo { tile_pixel_row } => self.wait_fifo(tile_pixel_row, pixel_fifo),
            }
        }
    }

    fn fetch_tile_data_index(&mut self, mmu: &MMU) {
        // println!("FTI: {}", self.x_position_counter);
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let lcdc = mmu.lcdc();
        let tile_map_area = if self.is_window_mode { lcdc.window_tilemap_area() } else { lcdc.bg_tilemap_area() };
        let ly = mmu.ly();
        let scx = mmu.scx();
        let scy = mmu.scy();
        let x_offset = if self.is_window_mode { self.x_position_counter as u16 } else { (self.x_position_counter as u16 + (scx as u16 / 8)) & 0x1F };
        let y_offset = if self.is_window_mode { 32 * (self.window_line_counter as u16 / 8) } else { 32 * (((ly as u16 + scy as u16) & 0xFF) / 8) };
        let offset = (x_offset + y_offset) & 0x3FF;
        let tile_data_address = tile_map_area.address(offset);
        let tile_data_index = mmu.get(tile_data_address);
        // if mmu.lcdc().is_window_enable() {
        //     println!("[w:{}]({},{}) s({},{}) off({},{}) add:{:#06X} idx:{:#04X}", self.is_window_mode, self.x_position_counter, ly, scx, scy, x_offset, y_offset, tile_data_address, tile_data_index);
        // }
        self.current_step = Step::FetchTileDataLow { tile_data_index };
    }

    fn fetch_tile_data_low(&mut self, tile_data_index: u8, mmu: &MMU) {
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let lcdc = mmu.lcdc();
        let ly = mmu.ly();
        let scy = mmu.scy();
        let tile_data_area = lcdc.bg_window_tiledata_area();
        let tile_data_address = tile_data_area.address(tile_data_index as u16); // 16 bytes per tile
        let tile_row_offset = if self.is_window_mode { 2 * (self.window_line_counter as u16 % 8) } else { 2 * ((ly as u16 + scy as u16) % 8) };
        let tile_data_row_address_low = tile_data_address + tile_row_offset;
        let tile_data_row_low = mmu.get(tile_data_row_address_low);
        // if mmu.lcdc().is_window_enable() {
        //     println!("FTL [w:{}]({},{}) scy:{} off:{} add:{:#06X} data:{:08b}", self.is_window_mode, self.x_position_counter, ly, scy, tile_row_offset, tile_data_row_address_low, tile_data_row_low);
        // }
        self.current_step = Step::FetchTileDataHi { tile_data_row_address_low, tile_data_row_low }
    }

    fn fetch_tile_data_hi(&mut self, tile_data_row_address_low: u16, tile_data_row_low: u8, mmu: &MMU) {
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let tile_data_row_hi = mmu.get(tile_data_row_address_low + 1);
        // if mmu.lcdc().is_window_enable() {
        //     println!("FTH addr:{:#06X} data:{:08b}", tile_data_row_address_low + 1, tile_data_row_hi);
        // }
        self.current_step = Step::PushToFifo {
            tile_data_row_low,
            tile_data_row_hi,
        }
    }

    fn push_to_fifo(&mut self, tile_data_row_low: u8, tile_data_row_hi: u8, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        // println!("PF: {}", self.x_position_counter);
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let tile_pixel_row = if mmu.lcdc().is_bg_window_enable() {
            // if mmu.ly()>=8 && mmu.ly()<=15 { println!("Draw bg enabled: ly: {}", mmu.ly()) }
            if self.is_window_mode {
                PixelType::from_window_tile_data(tile_data_row_low, tile_data_row_hi)
            } else {
                PixelType::from_bg_tile_data(tile_data_row_low, tile_data_row_hi)
            }
        } else {
            // println!("remove bg: ly:{}", mmu.ly());
            if self.is_window_mode {
                [PixelType::Window(0); 8]
            } else {
                [PixelType::Background(0); 8]
            }
        };

        // if tile_data_index == 0x16 {
        //     println!("[w:{}]({},{}) scy:{} off:{} add:{:#06X} data:{:08b}", self.is_window_mode, self.x_position_counter, ly, scy, tile_row_offset, tile_data_row_address_low, tile_data_row_low);
        // }
        // println!("PUSH FIFO tdrl:{:08b} tdrh:{:08b} -> {:?}", tile_data_row_low, tile_data_row_hi, tile_pixel_row);

        if pixel_fifo.can_push() {
            pixel_fifo.push_tile_pixel_row(tile_pixel_row);
            self.x_position_counter += 1;
            self.current_step = Step::FetchTileDataIndex;
        } else {
            self.current_step = Step::WaitFifo { tile_pixel_row }
        }
        assert_eq!(self.cycle_available, 0, "Cycle available should 0 but {}", self.cycle_available);
    }

    fn wait_fifo(&mut self, tile_pixel_row: [PixelType; 8], pixel_fifo: &mut PixelFifo) {
        // println!("WF: {}", self.x_position_counter);
        self.cycle_available -= 1;
        if pixel_fifo.can_push() {
            pixel_fifo.push_tile_pixel_row_front(tile_pixel_row);
            self.x_position_counter += 1;
            self.current_step = Step::FetchTileDataIndex;
        }
        assert_eq!(self.cycle_available, 0, "Cycle available should 0 but {}", self.cycle_available);
    }

    pub fn reset(&mut self, is_vblank: bool) {
        // assert_eq!(self.current_step, Step::Idle, "Step should be idle but {:?}", self.current_step);
        self.current_step = Step::FetchTileDataIndex;
        self.cycle_available = 0;
        if self.is_window_mode { self.window_line_counter += 1; }
        self.is_window_mode = false;
        self.x_position_counter = 0;
        if is_vblank { self.window_line_counter = 0; }
    }
    pub fn is_window_mode(&self) -> bool {
        self.is_window_mode
    }
}