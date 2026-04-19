use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

// 0x01: LD BC, d16
pub fn op_01(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let lo = bus.read8(cpu.regs.pc);
    let hi = bus.read8(cpu.regs.pc + 1);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(2);

    cpu.regs.set_bc(u16::from_le_bytes([lo, hi]));
    12
}

// 0x02: LD (BC), A
pub fn op_02(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let addr = cpu.regs.bc();
    bus.write8(addr, cpu.regs.a);
    8
}

// 0x06: LD B, d8
pub fn op_06(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let value = bus.read8(cpu.regs.pc);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(1);

    cpu.regs.b = value;
    8
}

// 0x08: LD (a16), SP
pub fn op_08(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let lo = bus.read8(cpu.regs.pc);
    let hi = bus.read8(cpu.regs.pc + 1);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(2);

    let addr = u16::from_le_bytes([lo, hi]);
    let sp = cpu.regs.sp;
    bus.write8(addr, (sp & 0x00FF) as u8);
    bus.write8(addr .wrapping_add(1), (sp >> 8) as u8);
    20
}

// 0x0A: LD A, (BC)
pub fn op_0a(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let addr = cpu.regs.bc();
    cpu.regs.a = bus.read8(addr);
    8
}

// 0x0E: LD C, d8
pub fn op_0e(cpu: &mut Cpu, bus: &mut dyn Bus) -> u8 {
    let value = bus.read8(cpu.regs.pc);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(1);

    cpu.regs.c = value;
    8
}
