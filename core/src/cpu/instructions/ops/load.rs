use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

pub fn op_01(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let lo = bus.read8(cpu.regs.pc);
    let hi = bus.read8(cpu.regs.pc + 1);
    cpu.regs.pc += 2;

    cpu.regs.set_bc(u16::from_le_bytes([lo, hi]));
    12
}
