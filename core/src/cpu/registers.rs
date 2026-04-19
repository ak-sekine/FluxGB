#[derive(Default, Debug, Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub f: u8, // フラグレジスタ（Z N H C）

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    // --- 16bit getter/setter ---

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16 & 0xF0) // F の下位4bitは常に0
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8; // 下位4bitは使わない
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

impl Registers {
    // --- Flag operations ---
    // ------------------------------------------------------------
    // F（フラグレジスタ）のビット構成
    //
    //  Bit | Name | 意味
    // -----+------+------------------------------
    //   7  |  Z   | Zero flag（結果が0なら1）
    //   6  |  N   | Subtract flag（減算命令なら1）
    //   5  |  H   | Half Carry flag（4bit繰り上がり）
    //   4  |  C   | Carry flag（8/16bit繰り上がり）
    //  3-0 |  -   | 常に 0（未使用ビット）
    //
    //  ※ Game Boy CPU（LR35902）では F の下位4bit は常に 0。
    // ------------------------------------------------------------
    pub fn get_flag_z(&self) -> bool { self.f & 0x80 != 0 }
    pub fn get_flag_n(&self) -> bool { self.f & 0x40 != 0 }
    pub fn get_flag_h(&self) -> bool { self.f & 0x20 != 0 }
    pub fn get_flag_c(&self) -> bool { self.f & 0x10 != 0 }

    pub fn set_flag_z(&mut self, v: bool) {
        if v { self.f |= 0x80 } else { self.f &= !0x80 }
    }
    pub fn set_flag_n(&mut self, v: bool) {
        if v { self.f |= 0x40 } else { self.f &= !0x40 }
    }
    pub fn set_flag_h(&mut self, v: bool) {
        if v { self.f |= 0x20 } else { self.f &= !0x20 }
    }
    pub fn set_flag_c(&mut self, v: bool) {
        if v { self.f |= 0x10 } else { self.f &= !0x10 }
    }
}
