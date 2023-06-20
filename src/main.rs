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
            match (frame, frame.id()) {
                (CanFrame::Data(data_frame), Id::Standard(id)) => match id.as_raw() {
                    0x21 => {
                        match (
                            data_frame
                                .data()
                                .get(0..4)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                            data_frame
                                .data()
                                .get(4..8)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                        ) {
                            (Some(Ok(accel_x)), Some(Ok(accel_y))) => {
                                println!("accel x: {accel_x}, accel y: {accel_y}")
                            }
                            _ => println!("{:?}", frame),
                        }
                    }
                    0x22 => {
                        match (
                            data_frame
                                .data()
                                .get(0..4)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                            data_frame
                                .data()
                                .get(4..8)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                        ) {
                            (Some(Ok(accel_z)), Some(Ok(gyro_x))) => {
                                println!("accel z: {accel_z}, gyro x: {gyro_x}")
                            }
                            _ => println!("{:?}", frame),
                        }
                    }
                    0x23 => {
                        match (
                            data_frame
                                .data()
                                .get(0..4)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                            data_frame
                                .data()
                                .get(4..8)
                                .map(|f| f.try_into().map(f32::from_le_bytes)),
                        ) {
                            (Some(Ok(gyro_y)), Some(Ok(gyro_z))) => {
                                println!("gyro y: {gyro_y}, gyro z: {gyro_z}")
                            }
                            _ => println!("{:?}", frame),
                        }
                    }
                    0x24 => {
                        match (data_frame
                            .data()
                            .get(0..4)
                            .map(|f| f.try_into().map(f32::from_le_bytes)),)
                        {
                            (Some(Ok(temp)),) => {
                                println!("temp: {temp}")
                            }
                            _ => println!("{:?}", frame),
                        }
                    }
                    _ => println!("{:?}", frame),
                },
                (_, _) => println!("{:?}", frame),
            }
        }
    }
}
