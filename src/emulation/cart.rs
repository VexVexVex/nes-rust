struct Cartridge {
    prg: &[u8], // PRG-ROM,
    chr: &[u8], // CHR-ROM,
    sram: &[u8], // Save RAM not Static RAM
    mapper: u8, // Type of Mapper
    mirror: u8, // Mirroring mode
    battery: u8 // Battery present
}