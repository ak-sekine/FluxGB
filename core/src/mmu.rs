use crate::bus::Bus;

pub struct Mmu {
    pub rom: Vec<u8>,           // カートリッジROM
    pub wram: [u8; 0x2000],     // Work RAM (8KB)
    pub hram: [u8; 0x7F],       // High RAM (127B)
}

impl Mmu {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            wram: [0; 0x2000],
            hram: [0; 0x7F],
        }
    }
}

impl Bus for Mmu {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // 0000–7FFF: ROM
            0x0000..=0x7FFF => {
                self.rom.get(addr as usize).copied().unwrap_or(0xFF)
            }

            // C000–DFFF: Work RAM
            0xC000..=0xDFFF => {
                let offset = (addr - 0xC000) as usize;
                self.wram[offset]
            }

            // FF80–FFFE: High RAM
            0xFF80..=0xFFFE => {
                let offset = (addr - 0xFF80) as usize;
                self.hram[offset]
            }

            // それ以外はまだ未実装なので 0xFF を返す
            _ => 0xFF,
        }
    }

    fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            // C000–DFFF: Work RAM
            0xC000..=0xDFFF => {
                let offset = (addr - 0xC000) as usize;
                self.wram[offset] = value;
            }

            // FF80–FFFE: High RAM
            0xFF80..=0xFFFE => {
                let offset = (addr - 0xFF80) as usize;
                self.hram[offset] = value;
            }

            // ROM や未実装領域は今は無視
            _ => {}
        }
    }
}
