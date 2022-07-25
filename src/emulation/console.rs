struct Console {
    cpu: &CPU,
    apu: &APU,
    ppu: &PPU,
    cart: &Cartridge,
    controller1: &Controller,
    controller2: &Controller,
    ram: &[u8]
}