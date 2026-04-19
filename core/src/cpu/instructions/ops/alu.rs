use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

// 0x09: ADD HL, BC
pub fn op_09(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let hl = cpu.regs.hl();
    let bc = cpu.regs.bc();
    let result = hl.wrapping_add(bc);

    cpu.regs.set_flag_n(false);
    cpu.regs
        .set_flag_h(((hl & 0x0FFF) + (bc & 0x0FFF)) > 0x0FFF);
    cpu.regs
        .set_flag_c((hl as u32 + bc as u32) > 0xFFFF);

    cpu.regs.set_hl(result);
    8
}
