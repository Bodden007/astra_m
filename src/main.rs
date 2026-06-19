//! # Astra M — Modbus TCP клиент для опроса датчика давления
//!
//! Эта программа подключается к Modbus TCP-устройству (например, OWEN 210),
//! циклически читает два регистра (32-битное число с плавающей точкой) и выводит результат
//! в консоль. Используется "сырой" Modbus TCP без сторонних библиотек — только стандартный
//! ввод/вывод Rust.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, Instant};

mod app;
use app::build_tcp::build_request as build;
use app::converter::{
    converter_words_to_f32::words_to_f32,
    formater::{read_u8, read_u16, read_u64},
};
use app::models::{read_parm::read_parm, tcp_parm::TcpParm};

/// Точка входа в программу.
///
/// Устанавливает TCP-соединение с Modbus-устройством, включает `TCP_NODELAY`
/// и запускает бесконечный цикл опроса регистров.
fn main() -> std::io::Result<()> {
    println!("Выйти из программы можно комбинацией Ctrl+C");
    let ip = read_parm("IP адрес: ");
    let port = read_parm("Port: ");
    let host = format!("{}:{}", ip, port);
    println!("Host = {}", host);
    let slave_id = read_parm("Slave ID: ");
    let slave_id = read_u8(&slave_id);
    let function_code = read_parm(
        "Функция чтения: \n
        3 - Read Holding Registers \n
        4 - Read Input Registers: ",
    );
    let function_code = read_u8(&function_code);
    let start_address = read_parm("Start address: ");
    let start_adress = read_u16(&start_address);
    let register_count = read_parm("Register count: ");
    let register_count = read_u16(&register_count);
    let time_cicl = read_parm("Время опроса мс: ");
    let time_cicl = read_u64(&time_cicl);

    println!(
        "{}, {}, {}, {}, {}, {}",
        host, slave_id, function_code, start_adress, register_count, time_cicl
    );

    let tcp_parm = TcpParm::new_tcp_parm(
        host,
        slave_id,
        function_code,
        start_adress,
        register_count,
        time_cicl,
    );

    println!("Rust Raw Modbus TCP Test");

    // Открываем TCP-соединение к Modbus-устройству
    let mut stream = TcpStream::connect(tcp_parm.host)?;

    // Отключаем алгоритм Нейгла — отправляем данные немедленно
    stream.set_nodelay(true)?;
    println!("TCP_NODELAY enabled");

    println!("Connected");

    // Счётчик транзакций (инкрементируется с каждым запросом, с переполнением)
    let mut transaction_id: u16 = 0;
    // Счётчик циклов опроса
    let mut cycle: u64 = 0;

    // Бесконечный цикл опроса Modbus-регистров
    loop {
        cycle += 1;
        transaction_id = transaction_id.wrapping_add(1);

        // Засекаем общее время цикла
        let total_sw = Instant::now();

        // Формируем Modbus TCP-запрос (Read Input Registers, функция 0x04)
        let request = build(
            transaction_id,
            tcp_parm.slave_id,
            tcp_parm.start_address,
            tcp_parm.register_count,
            tcp_parm.function_code,
        );

        println!("{cycle} | TX");

        // Отправляем запрос в сокет
        stream.write_all(&request)?;

        println!("{cycle} | WAIT HEADER");

        // Засекаем время ожидания заголовка
        let header_sw = Instant::now();

        // Читаем 7-байтовый заголовок MBAP (Modbus Application Protocol header)
        let mut header = [0u8; 7];
        stream.read_exact(&mut header)?;

        println!(
            "{cycle} | HEADER OK | {} ms",
            header_sw.elapsed().as_millis()
        );

        // Парсим поля MBAP-заголовка
        let response_tid = u16::from_be_bytes([header[0], header[1]]); // Transaction ID
        let protocol_id = u16::from_be_bytes([header[2], header[3]]); // Protocol ID (должен быть 0)
        let length = u16::from_be_bytes([header[4], header[5]]); // Длина оставшихся данных (PDU + Unit ID)
        let unit_id = header[6]; // Unit ID (Slave ID)

        println!("{cycle} | WAIT BODY");

        // Засекаем время ожидания тела ответа (PDU)
        let body_sw = Instant::now();

        // Читаем PDU (Protocol Data Unit): length - 1 байт (минус Unit ID)
        let mut pdu = vec![0u8; (length - 1) as usize];
        stream.read_exact(&mut pdu)?;

        println!("{cycle} | BODY OK | {} ms", body_sw.elapsed().as_millis());

        // Проверяем, что Transaction ID ответа совпадает с отправленным
        if response_tid != transaction_id {
            panic!(
                "Wrong transaction id {} expected {}",
                response_tid, transaction_id
            );
        }

        // Protocol ID всегда должен быть 0 для Modbus TCP
        if protocol_id != 0 {
            panic!("Wrong protocol id {}", protocol_id);
        }

        // Проверяем, что Unit ID совпадает с ожидаемым Slave ID
        if unit_id != tcp_parm.slave_id {
            panic!("Wrong slave id {}", unit_id);
        }

        // Если первый байт PDU == 0x84 — это Modbus-исключение (0x80 | функция 0x04)
        if pdu[0] == 0x84 {
            panic!("Modbus exception {}", pdu[1]);
        }

        // Извлекаем два 16-битных регистра (big-endian)
        let pressure_lo = u16::from_be_bytes([pdu[2], pdu[3]]); // Младшие 16 бит
        let pressure_hi = u16::from_be_bytes([pdu[4], pdu[5]]); // Старшие 16 бит

        // Собираем 32-bit float из двух регистров
        let value = words_to_f32(pressure_hi, pressure_lo);

        println!(
            "{cycle} | DONE | total={} ms | value={:.3}",
            total_sw.elapsed().as_millis(),
            value
        );

        println!();

        // Пауза перед следующим циклом опроса
        thread::sleep(Duration::from_millis(tcp_parm.time_cicl));
    }
}
