use socketcan::{embedded_can, CanFrame, CanSocket, EmbeddedFrame, Id, Socket};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut can_socket = CanSocket::open("can0")?;
    loop {
        if let Ok(frame) = embedded_can::nb::Can::receive(&mut can_socket) {
            match frame {
                CanFrame::Data(d) => match d.id() {
                    Id::Standard(id) => match id.as_raw() {
                        0x21 => match (&(d.data())[0..2]).try_into().map(i16::from_le_bytes) {
                            Ok(temp) => println!("Temp: {}", (temp as f64) / 340.00 + 36.53),
                            Err(err) => println!("{}", err),
                        },
                        _ => println!("{:?}", frame),
                    },
                    _ => {
                        println!("{:?}", frame)
                    }
                },
                f => println!("{:?}", f),
            }
        }
    }
}
