use socketcan::{embedded_can, CanSocket, Socket};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut can_socket = CanSocket::open("can0")?;
    loop {
        if let Ok(frame) = embedded_can::nb::Can::receive(&mut can_socket) {
            println!("{:?}", frame);
        }
    }
}
