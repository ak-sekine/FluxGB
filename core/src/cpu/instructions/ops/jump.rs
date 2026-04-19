use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

// 0x18: JR r8
pub fn op_18(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let offset = bus.read8(cpu.regs.pc) as i8;
    cpu.regs.pc = cpu.regs.pc.wrapping_add(1);

    cpu.regs.pc = cpu.regs.pc.wrapping_add(offset as u16);
    12
}
