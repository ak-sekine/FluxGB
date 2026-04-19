use crate::cpu::cpu::Cpu;
use crate::bus::Bus;

// 0x03: INC BC
pub fn op_03(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let bc = cpu.regs.bc().wrapping_add(1);
    cpu.regs.set_bc(bc);
    8
}

// 0x04: INC B
pub fn op_04(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let b = cpu.regs.b;
    let result = b.wrapping_add(1);

    cpu.regs.set_flag_z(result == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h((b & 0x0F) + 1 > 0x0F);

    cpu.regs.b = result;
    4
}

// 0x05: DEC B
pub fn op_05(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let b = cpu.regs.b;
    let result = b.wrapping_sub(1);

    cpu.regs.set_flag_z(result == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs.set_flag_h((b & 0x0F) == 0);

    cpu.regs.b = result;
    4
}

// 0x0C: INC C
pub fn op_0c(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let c = cpu.regs.c;
    let result = c.wrapping_add(1);

    cpu.regs.set_flag_z(result == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h((c & 0x0F) + 1 > 0x0F);

    cpu.regs.c = result;
    4
}

// 0x0B: DEC BC
pub fn op_0b(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let bc = cpu.regs.bc().wrapping_sub(1);
    cpu.regs.set_bc(bc);
    8
}

// 0x0D: DEC C
pub fn op_0d(cpu: &mut Cpu, _bus: &mut dyn Bus) -> u8 {
    let c = cpu.regs.c;
    let result = c.wrapping_sub(1);

    cpu.regs.set_flag_z(result == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs.set_flag_h((c & 0x0F) == 0);

    cpu.regs.c = result;
    4
}
