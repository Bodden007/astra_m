pub fn read_parm(parm: &str) -> String {
    loop {
        println!("{}", parm);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            println!("IP адрес не валидный");
            continue;
        }

        break input.to_string();
    }
    // TcpParm {
    //     transaction_id: 1,
    //     slave_id: 1,
    //     start_address: 0,
    //     quantity_of_registers: 2,
    // }
}
