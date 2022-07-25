struct Pulse {
    enabled: bool,
    channel: u8,
    length_enabled: bool,
    length_value: u8,
    timer_period: u16,
    time_value: u16,
    duty_mode: u8,
    duty_value: u8,
    sweep_reload: bool,
    sweep_enabled: bool,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_period: u8,
    sweep_value: u8,
    envelope_enabled: bool,
    envelope_loop: bool,
    envelope_start: bool,
    envelope_period: u8,
    envelope_value: u8,
    envelope_volume: u8,
    constant_volume: u8
}

struct Triangle {
    enabled: bool,
    length_enabled: bool,
    length_value: u8,
    timer_period: u16,
    timer_value: u16,
    duty_value: u8,
    counter_period: u8,
    counter_value: u8,
    counter_reload: bool
}

struct Noise {
    enabled: bool,
    mode: bool,
    shift_register: u16,
    length_enabled: bool,
    length_value: u8,
    timer_period: u16,
    timer_value: u16,
    envelope_enabled: bool,
    envelope_loop: bool,
    envelope_start: bool,
    envelope_period: u8,
    envelope_value: u8,
    envelope_volume: u8,
    constant_volume: u8
}

struct DMC {
    enabled: bool,
    value: u8,
    sample_address: u16,
    sample_length: u16,
    current_address: u16,
    current_length: u16,
    shfit_register: u8,
    bit_count: u8,
    tick_period: u8,
    tick_value: u8,
    looping: bool,
    irq: bool
}

struct APU {
    channel: mpsc::channel,
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: DMC,
    cycle: u16,
    frame_period: u8,
    frame_value: u8,
    frame_irq: bool
}