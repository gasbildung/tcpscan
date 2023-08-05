use std::net::TcpStream;
use std::env;

fn is_port_open(address: &str, port: u16) -> bool {
    if let Ok(_) = TcpStream::connect((address, port)) {
        return true;
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1{
        return;
    }
    let address = &args[1];
    for port in 1..65535{
        let address = address.to_string();
        if is_port_open(&address, port) {
                println!("{}:{}", &address, port);
            }
        }
}
