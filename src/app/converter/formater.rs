pub fn read_u8(input: &str) -> u8 {
    let value: u8 = input
        .parse()
        .expect("Ошибка: введено не число или число вне диапазона u8 (0-255)");
    value
}

pub fn read_u16(input: &str) -> u16 {
    let value: u16 = input
        .parse()
        .expect("Ошибка: введено не число или число вне диапазона u16 (0-65535)");
    value
}

pub fn read_u64(input: &str) -> u64 {
    let value: u64 = input
        .parse()
        .expect("Ошибка: введено не число или число вне диапазона u64 (0-65535)");
    value
}
