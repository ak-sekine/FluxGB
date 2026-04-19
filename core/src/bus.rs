pub trait Bus {
    /// 8bit 読み込み
    fn read8(&self, addr: u16) -> u8;

    /// 8bit 書き込み
    fn write8(&mut self, addr: u16, value: u8);
}