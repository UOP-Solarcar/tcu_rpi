use clap::Parser;
use socketcan::{embedded_can, CanFrame, CanSocket, EmbeddedFrame, Id, Socket};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of CAN device
    ifname: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut can_socket = CanSocket::open(&args.ifname)?;
    loop {
        if let Ok(frame) = embedded_can::nb::Can::receive(&mut can_socket) {
            match frame {
                CanFrame::Data(d) => match d.id() {
                    Id::Standard(id) => match id.as_raw() {
                        0x21 => match (
                            d.data()[0..2].try_into().map(i16::from_le_bytes),
                            d.data()[2..4].try_into().map(i16::from_le_bytes),
                            d.data()[4..6].try_into().map(i16::from_le_bytes),
                            d.data()[6..8].try_into().map(i16::from_le_bytes),
                        ) {
                            (Ok(temp), Ok(accel_x), Ok(accel_y), Ok(accel_z)) => {
                                println!("Temp: {}", (temp as f64) / 340.00 + 36.53);
                                println!(
                                    "accelX: {} | accelY: {} | accelZ: {}",
                                    accel_x, accel_y, accel_z
                                );
                            }
                            e => println!("{:?}", e),
                        },
                        0x22 => match (
                            d.data()[0..2].try_into().map(i16::from_le_bytes),
                            d.data()[2..4].try_into().map(i16::from_le_bytes),
                            d.data()[4..6].try_into().map(i16::from_le_bytes),
                        ) {
                            (Ok(gyro_x), Ok(gyro_y), Ok(gyro_z)) => {
                                println!(
                                    "gyroX: {} | gyroY: {} | gyroZ: {}",
                                    gyro_x, gyro_y, gyro_z,
                                );
                            }
                            e => println!("{:?}", e),
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
