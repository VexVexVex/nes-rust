struct PPU {
    cycle: u16,
    scan_line: u16,
    frame: usize,

    // Storage
    palette: [u8; 32],
    name_table: [u8; 2048],
    oam: [u8; 256],
    front: &image::Rgba,
    back: &image::Rgba,

    // Registers
    current_vram: u16,
    temp_vram: u16,
    x_scroll: u8,
    write: bool,
    even_odd: bool,
    register: u8,

    // NMI flags
    nmi_ocurred: bool,
    nmi_output: bool,
    nmi_previous: bool,
    nmi_delay: u8,

    // Background temp data
    name_table_byte: u8,
    attr_table_byte: u8,
    low_tile_byte: u8,
    high_tile_byte: u8,
    tile_data: usize,

    // Sprite temp data
    sprite_count: isize,
    sprite_patterns: [u32; 8],
    sprite_positions: [u8; 8],
    sprite_priorities: [u8; 8],
    sprite_indexes: [u8],

    // $2000 PPUCTRL
    flag_name_table: u8,
    flag_increment: u8,
    flag_sprite_table: u8,
    flag_background_table: u8,
    flag_sprite_size: u8,
    flag_main_secondary: u8,

    // $2001 PPUMASK
    gray_scale: bool,
    show_left_bgr: bool,
    show_left_spr: bool,
    show_bgr: bool,
    show_spr: bool,
    red_tint: bool,
    green_tine: bool,
    blue_tint: bool,

    // $2002 PPUSTATUS
    spr_zero_hit: u8,
    spr_overflow: u8,

    // $2003 OAM Address
    oam_address: u8,

    // $2007 PPU Data
    buff_data: u8

}