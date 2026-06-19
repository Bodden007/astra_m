/// Формирует Modbus TCP-запрос на чтение входных регистров (функция 0x04).
///
/// # Аргументы
///
/// * `transaction_id` — уникальный идентификатор транзакции (инкрементируется с каждым запросом).
/// * `slave_id` — Modbus Slave ID (Unit ID) устройства.
/// * `start_address` — начальный адрес регистров для чтения.
/// * `register_count` — количество читаемых регистров.
///
/// # Возвращает
///
/// 12-байтовый массив, содержащий полный Modbus TCP-фрейм:
/// - [0..2]  — Transaction ID
/// - [2..4]  — Protocol ID (всегда 0)
/// - [4..6]  — Length (количество байт после Length-поля)
/// - [6]     — Unit ID (Slave ID)
/// - [7]     — Function Code (0x04 — Read Input Registers)
/// - [8..10] — Starting Address
/// - [10..12] — Quantity of Registers
pub fn build_request(
    transaction_id: u16,
    slave_id: u8,
    start_address: u16,
    register_count: u16,
    function_code: u8,
) -> [u8; 12] {
    let mut buffer = [0u8; 12];

    // Transaction ID (2 байта, big-endian)
    buffer[0..2].copy_from_slice(&transaction_id.to_be_bytes());

    // Protocol ID (2 байта, всегда 0 для Modbus TCP)
    buffer[2..4].copy_from_slice(&0u16.to_be_bytes());

    // Length (2 байта, big-endian) — количество байт после этого поля:
    // 1 (Unit ID) + 1 (Function Code) + 2 (Start Address) + 2 (Quantity) = 6
    buffer[4..6].copy_from_slice(&6u16.to_be_bytes());

    // Unit ID (Slave ID)
    buffer[6] = slave_id;
    // Function Code: 0x04 = Read Input Registers
    buffer[7] = function_code;

    // Starting Address (2 байта, big-endian)
    buffer[8..10].copy_from_slice(&start_address.to_be_bytes());

    // Quantity of Registers (2 байта, big-endian)
    buffer[10..12].copy_from_slice(&register_count.to_be_bytes());

    buffer
}
