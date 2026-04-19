pub mod load;
pub mod alu;
pub mod incdec;
pub mod jump;
pub mod rotate;
pub mod bit;
pub mod misc;

// すべての命令関数を再公開
pub use load::*;
pub use alu::*;
pub use incdec::*;
pub use jump::*;
pub use rotate::*;
pub use bit::*;
pub use misc::*;
