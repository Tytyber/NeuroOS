use x86_64::instructions::port::Port;

const SERIAL_PORT: u16 = 0x3F8;
const LINE_STATUS: u16 = SERIAL_PORT + 5;

pub fn init_serial() {
    unsafe {
        Port::<u8>::new(SERIAL_PORT + 1).write(0x00); // disable interrupts
        Port::<u8>::new(SERIAL_PORT + 3).write(0x80); // enable DLAB

        // 38400 baud
        Port::<u8>::new(SERIAL_PORT + 0).write(0x03);
        Port::<u8>::new(SERIAL_PORT + 1).write(0x00);

        Port::<u8>::new(SERIAL_PORT + 3).write(0x03); // 8 bits, no parity
        Port::<u8>::new(SERIAL_PORT + 2).write(0xC7); // FIFO
        Port::<u8>::new(SERIAL_PORT + 4).write(0x0B); // IRQs enabled, RTS/DSR set
    }
}

fn ready() -> bool {
    unsafe {
        Port::<u8>::new(LINE_STATUS).read() & 0x20 != 0
    }
}

pub fn print_byte(b: u8) {
    while !ready() {}

    unsafe {
        Port::<u8>::new(SERIAL_PORT).write(b);
    }
}

pub fn print_str(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            print_byte(b'\r');
        }
        print_byte(b);
    }
}

pub fn print_hex(mut v: u64) {
    let mut buf = [0u8; 16];

    for i in (0..16).rev() {
        let d = (v & 0xF) as u8;
        buf[i] = match d {
            0..=9 => b'0' + d,
            _ => b'A' + (d - 10),
        };
        v >>= 4;
    }

    for b in buf {
        print_byte(b);
    }
}