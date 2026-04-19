use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

pub fn op_00(_cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    4
}

pub fn op_unimplemented(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    panic!("未実装 opcode: {:02X}", cpu.last_opcode);
}
