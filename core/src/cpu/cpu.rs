use super::registers::Registers;
use crate::bus::Bus;
use crate::cpu::instructions::table::OPCODE_TABLE;
use crate::cpu::instructions::cb_table::CB_TABLE;

#[derive(Default, Debug)]
pub struct Cpu {
    pub regs: Registers,
    pub halted: bool,
    pub ime: bool,
    pub last_opcode: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            halted: false,
            ime: false,
            last_opcode: 0,
        }
    }

    /// 1命令実行して、消費サイクル数を返す
    pub fn step(&mut self, bus: &mut dyn Bus) -> u8 {
        // opcode fetch
        let opcode = bus.read8(self.regs.pc);
        self.last_opcode = opcode;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        // CB prefix
        if opcode == 0xCB {
            let cb = bus.read8(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            self.last_opcode = cb;

            let handler = CB_TABLE[cb as usize];
            return handler(self, bus);
        }

        // normal opcode
        let handler = OPCODE_TABLE[opcode as usize];
        handler(self, bus)
    }

}
