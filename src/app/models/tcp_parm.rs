pub struct TcpParm {
    pub host: String,
    pub slave_id: u8,
    pub function_code: u8,
    pub start_address: u16,
    pub register_count: u16,
    pub time_cicl: u64,
}
impl TcpParm {
    pub fn new_tcp_parm(
        host: String,
        slave_id: u8,
        function_code: u8,
        start_address: u16,
        register_count: u16,
        time_cicl: u64,
    ) -> TcpParm {
        TcpParm {
            host,
            slave_id,
            function_code,
            start_address,
            register_count,
            time_cicl,
        }
    }
}
