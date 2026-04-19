use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

pub fn op_00(_cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    4
}

// 0x07: RLCA
pub fn op_07(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let a = cpu.regs.a;
    let carry = (a & 0x80) != 0;
    let result = (a << 1) | (a >> 7);

    cpu.regs.a = result;
    cpu.regs.set_flag_z(false);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(false);
    cpu.regs.set_flag_c(carry);
    4
}

// 0x0F: RRCA
pub fn op_0f(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let a = cpu.regs.a;
    let carry = (a & 0x01) != 0;
    let result = (a >> 1) | (a << 7);

    cpu.regs.a = result;
    cpu.regs.set_flag_z(false);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(false);
    cpu.regs.set_flag_c(carry);
    4
}

pub fn op_unimplemented(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    panic!("未実装 opcode: {:02X}", cpu.last_opcode);
}
