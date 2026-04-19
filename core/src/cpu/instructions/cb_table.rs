use crate::cpu::cpu::Cpu;
use crate::bus::Bus;
use super::ops::op_unimplemented;

pub type OpFn = fn(&mut Cpu, &mut dyn Bus) -> u8;

pub static CB_TABLE: [OpFn; 256] = [op_unimplemented; 256];
