use crate::cpu::cpu::Cpu;
use crate::bus::Bus;
use super::ops::*;

/// 命令関数の型
pub type OpFn = fn(&mut Cpu, &mut dyn Bus) -> u8;

/// 256エントリの命令テーブル
pub static OPCODE_TABLE: [OpFn; 256] = [
    // 0x00 - 0x0F
    op_00, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x10 - 0x1F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x20 - 0x2F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x30 - 0x3F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x40 - 0x4F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x50 - 0x5F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x60 - 0x6F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x70 - 0x7F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x80 - 0x8F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0x90 - 0x9F
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xA0 - 0xAF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xB0 - 0xBF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xC0 - 0xCF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xD0 - 0xDF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xE0 - 0xEF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,

    // 0xF0 - 0xFF
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
    op_unimplemented, op_unimplemented, op_unimplemented, op_unimplemented,
];

