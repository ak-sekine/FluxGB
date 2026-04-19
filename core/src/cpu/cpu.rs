use super::registers::Registers;
use crate::bus::Bus;

#[derive(Default, Debug)]
pub struct Cpu {
    pub regs: Registers,
    pub halted: bool,
    pub ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            halted: false,
            ime: false,
        }
    }

    pub fn step(&mut self, bus: &mut impl crate::bus::Bus) -> u32 {
        // 1. opcode フェッチ
        let pc = self.regs.pc;
        let opcode = bus.read8(pc);

        // 2. PC を進める
        self.regs.pc = self.regs.pc.wrapping_add(1);

        // 3. opcode に応じて命令を実行（今はNOPだけ）
        match opcode {
            0x00 => { 4 } // NOP

            0x01 => unimplemented!("LD BC, d16"),
            0x02 => unimplemented!("LD (BC), A"),
            0x03 => unimplemented!("INC BC"),
            0x04 => unimplemented!("INC B"),
            0x05 => unimplemented!("DEC B"),
            0x06 => unimplemented!("LD B, d8"),
            0x07 => unimplemented!("RLCA"),
            0x08 => unimplemented!("LD (a16), SP"),
            0x09 => unimplemented!("ADD HL, BC"),
            0x0A => unimplemented!("LD A, (BC)"),
            0x0B => unimplemented!("DEC BC"),
            0x0C => unimplemented!("INC C"),
            0x0D => unimplemented!("DEC C"),
            0x0E => unimplemented!("LD C, d8"),
            0x0F => unimplemented!("RRCA"),

            0x10 => unimplemented!("STOP"),
            0x11 => unimplemented!("LD DE, d16"),
            0x12 => unimplemented!("LD (DE), A"),
            0x13 => unimplemented!("INC DE"),
            0x14 => unimplemented!("INC D"),
            0x15 => unimplemented!("DEC D"),
            0x16 => unimplemented!("LD D, d8"),
            0x17 => unimplemented!("RLA"),
            0x18 => unimplemented!("JR r8"),
            0x19 => unimplemented!("ADD HL, DE"),
            0x1A => unimplemented!("LD A, (DE)"),
            0x1B => unimplemented!("DEC DE"),
            0x1C => unimplemented!("INC E"),
            0x1D => unimplemented!("DEC E"),
            0x1E => unimplemented!("LD E, d8"),
            0x1F => unimplemented!("RRA"),

            0x20 => unimplemented!("JR NZ, r8"),
            0x21 => unimplemented!("LD HL, d16"),
            0x22 => unimplemented!("LD (HL+), A"),
            0x23 => unimplemented!("INC HL"),
            0x24 => unimplemented!("INC H"),
            0x25 => unimplemented!("DEC H"),
            0x26 => unimplemented!("LD H, d8"),
            0x27 => unimplemented!("DAA"),
            0x28 => unimplemented!("JR Z, r8"),
            0x29 => unimplemented!("ADD HL, HL"),
            0x2A => unimplemented!("LD A, (HL+)"),
            0x2B => unimplemented!("DEC HL"),
            0x2C => unimplemented!("INC L"),
            0x2D => unimplemented!("DEC L"),
            0x2E => unimplemented!("LD L, d8"),
            0x2F => unimplemented!("CPL"),

            0x30 => unimplemented!("JR NC, r8"),
            0x31 => unimplemented!("LD SP, d16"),
            0x32 => unimplemented!("LD (HL-), A"),
            0x33 => unimplemented!("INC SP"),
            0x34 => unimplemented!("INC (HL)"),
            0x35 => unimplemented!("DEC (HL)"),
            0x36 => unimplemented!("LD (HL), d8"),
            0x37 => unimplemented!("SCF"),
            0x38 => unimplemented!("JR C, r8"),
            0x39 => unimplemented!("ADD HL, SP"),
            0x3A => unimplemented!("LD A, (HL-)"),
            0x3B => unimplemented!("DEC SP"),
            0x3C => unimplemented!("INC A"),
            0x3D => unimplemented!("DEC A"),
            0x3E => unimplemented!("LD A, d8"),
            0x3F => unimplemented!("CCF"),

            // 0x40〜0x7F: LD r, r
            0x40..=0x7F => unimplemented!("LD r, r"),

            // 0x80〜0x8F: ADD A, r
            0x80..=0x8F => unimplemented!("ADD A, r"),

            // 0x90〜0x9F: SUB / SBC
            0x90..=0x9F => unimplemented!("SUB/SBC A, r"),

            // 0xA0〜0xAF: AND / XOR
            0xA0..=0xAF => unimplemented!("AND/XOR A, r"),

            // 0xB0〜0xBF: OR / CP
            0xB0..=0xBF => unimplemented!("OR/CP A, r"),

            // 0xC0〜0xFF: ジャンプ・コール・リターン・割り込み
            0xC0 => unimplemented!("RET NZ"),
            0xC1 => unimplemented!("POP BC"),
            0xC2 => unimplemented!("JP NZ, a16"),
            0xC3 => unimplemented!("JP a16"),
            0xC4 => unimplemented!("CALL NZ, a16"),
            0xC5 => unimplemented!("PUSH BC"),
            0xC6 => unimplemented!("ADD A, d8"),
            0xC7 => unimplemented!("RST 00H"),
            0xC8 => unimplemented!("RET Z"),
            0xC9 => unimplemented!("RET"),
            0xCA => unimplemented!("JP Z, a16"),
            0xCB => return self.exec_cb(bus), // CB テーブルへ
            0xCC => unimplemented!("CALL Z, a16"),
            0xCD => unimplemented!("CALL a16"),
            0xCE => unimplemented!("ADC A, d8"),
            0xCF => unimplemented!("RST 08H"),

            0xD0 => unimplemented!("RET NC"),
            0xD1 => unimplemented!("POP DE"),
            0xD2 => unimplemented!("JP NC, a16"),
            0xD3 => unimplemented!("(未使用)"),
            0xD4 => unimplemented!("CALL NC, a16"),
            0xD5 => unimplemented!("PUSH DE"),
            0xD6 => unimplemented!("SUB d8"),
            0xD7 => unimplemented!("RST 10H"),
            0xD8 => unimplemented!("RET C"),
            0xD9 => unimplemented!("RETI"),
            0xDA => unimplemented!("JP C, a16"),
            0xDB => unimplemented!("(未使用)"),
            0xDC => unimplemented!("CALL C, a16"),
            0xDD => unimplemented!("(未使用)"),
            0xDE => unimplemented!("SBC A, d8"),
            0xDF => unimplemented!("RST 18H"),

            0xE0 => unimplemented!("LDH (a8), A"),
            0xE1 => unimplemented!("POP HL"),
            0xE2 => unimplemented!("LD (C), A"),
            0xE3 => unimplemented!("(未使用)"),
            0xE4 => unimplemented!("(未使用)"),
            0xE5 => unimplemented!("PUSH HL"),
            0xE6 => unimplemented!("AND d8"),
            0xE7 => unimplemented!("RST 20H"),
            0xE8 => unimplemented!("ADD SP, r8"),
            0xE9 => unimplemented!("JP (HL)"),
            0xEA => unimplemented!("LD (a16), A"),
            0xEB => unimplemented!("(未使用)"),
            0xEC => unimplemented!("(未使用)"),
            0xED => unimplemented!("(未使用)"),
            0xEE => unimplemented!("XOR d8"),
            0xEF => unimplemented!("RST 28H"),

            0xF0 => unimplemented!("LDH A, (a8)"),
            0xF1 => unimplemented!("POP AF"),
            0xF2 => unimplemented!("LD A, (C)"),
            0xF3 => unimplemented!("DI"),
            0xF4 => unimplemented!("(未使用)"),
            0xF5 => unimplemented!("PUSH AF"),
            0xF6 => unimplemented!("OR d8"),
            0xF7 => unimplemented!("RST 30H"),
            0xF8 => unimplemented!("LD HL, SP+r8"),
            0xF9 => unimplemented!("LD SP, HL"),
            0xFA => unimplemented!("LD A, (a16)"),
            0xFB => unimplemented!("EI"),
            0xFC => unimplemented!("(未使用)"),
            0xFD => unimplemented!("(未使用)"),
            0xFE => unimplemented!("CP d8"),
            0xFF => unimplemented!("RST 38H"),

            _ => panic!("未実装の opcode: {:02X}", opcode),
        }
   }

    fn exec_cb(&mut self, bus: &mut impl Bus) -> u32 {
        let cb = bus.read8(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);

        match cb {
            0x00 => unimplemented!("RLC B"),
            0x01 => unimplemented!("RLC C"),
            0x02 => unimplemented!("RLC D"),
            0x03 => unimplemented!("RLC E"),
            0x04 => unimplemented!("RLC H"),
            0x05 => unimplemented!("RLC L"),
            0x06 => unimplemented!("RLC (HL)"),
            0x07 => unimplemented!("RLC A"),

            0x08 => unimplemented!("RRC B"),
            0x09 => unimplemented!("RRC C"),
            0x0A => unimplemented!("RRC D"),
            0x0B => unimplemented!("RRC E"),
            0x0C => unimplemented!("RRC H"),
            0x0D => unimplemented!("RRC L"),
            0x0E => unimplemented!("RRC (HL)"),
            0x0F => unimplemented!("RRC A"),

            0x10 => unimplemented!("RL B"),
            0x11 => unimplemented!("RL C"),
            0x12 => unimplemented!("RL D"),
            0x13 => unimplemented!("RL E"),
            0x14 => unimplemented!("RL H"),
            0x15 => unimplemented!("RL L"),
            0x16 => unimplemented!("RL (HL)"),
            0x17 => unimplemented!("RL A"),

            0x18 => unimplemented!("RR B"),
            0x19 => unimplemented!("RR C"),
            0x1A => unimplemented!("RR D"),
            0x1B => unimplemented!("RR E"),
            0x1C => unimplemented!("RR H"),
            0x1D => unimplemented!("RR L"),
            0x1E => unimplemented!("RR (HL)"),
            0x1F => unimplemented!("RR A"),

            0x20 => unimplemented!("SLA B"),
            0x21 => unimplemented!("SLA C"),
            0x22 => unimplemented!("SLA D"),
            0x23 => unimplemented!("SLA E"),
            0x24 => unimplemented!("SLA H"),
            0x25 => unimplemented!("SLA L"),
            0x26 => unimplemented!("SLA (HL)"),
            0x27 => unimplemented!("SLA A"),

            0x28 => unimplemented!("SRA B"),
            0x29 => unimplemented!("SRA C"),
            0x2A => unimplemented!("SRA D"),
            0x2B => unimplemented!("SRA E"),
            0x2C => unimplemented!("SRA H"),
            0x2D => unimplemented!("SRA L"),
            0x2E => unimplemented!("SRA (HL)"),
            0x2F => unimplemented!("SRA A"),

            0x30 => unimplemented!("SWAP B"),
            0x31 => unimplemented!("SWAP C"),
            0x32 => unimplemented!("SWAP D"),
            0x33 => unimplemented!("SWAP E"),
            0x34 => unimplemented!("SWAP H"),
            0x35 => unimplemented!("SWAP L"),
            0x36 => unimplemented!("SWAP (HL)"),
            0x37 => unimplemented!("SWAP A"),

            0x38 => unimplemented!("SRL B"),
            0x39 => unimplemented!("SRL C"),
            0x3A => unimplemented!("SRL D"),
            0x3B => unimplemented!("SRL E"),
            0x3C => unimplemented!("SRL H"),
            0x3D => unimplemented!("SRL L"),
            0x3E => unimplemented!("SRL (HL)"),
            0x3F => unimplemented!("SRL A"),

            // 0x40〜0x7F: BIT b, r
            0x40..=0x7F => unimplemented!("BIT b, r"),

            // 0x80〜0xBF: RES b, r
            0x80..=0xBF => unimplemented!("RES b, r"),

            // 0xC0〜0xFF: SET b, r
            0xC0..=0xFF => unimplemented!("SET b, r"),

            _ => panic!("未実装の CB opcode: {:02X}", cb),
        }
    }

}
