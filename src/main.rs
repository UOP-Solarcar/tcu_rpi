use clap::Parser;
use socketcan::{BlockingCan, CanFrame, CanSocket, EmbeddedFrame, Id, Socket};
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
        if let Ok(frame) = BlockingCan::receive(&mut can_socket) {
            match (frame, frame.id()) {
                (CanFrame::Data(data_frame), Id::Standard(id)) => {
                    let data = data_frame.data();
                    match id.as_raw() {
                        0x6B0 => {
                            match (
                                data.get(0..=1)
                                    .map(|f| f.try_into().map(u16::from_be_bytes)),
                                data.get(2..=3)
                                    .map(|f| f.try_into().map(u16::from_be_bytes)),
                                data.get(4),
                                data.get(5..=6)
                                    .map(|f| f.try_into().map(u16::from_be_bytes)),
                                data.get(7),
                            ) {
                                (
                                    Some(Ok(pack_current)),
                                    Some(Ok(pack_inst_voltage)),
                                    Some(&pack_soc),
                                    Some(Ok(relay_state)),
                                    Some(&checksum),
                                ) => {
                                    println!("Pack current: {pack_current}, Pack Inst. Voltage: {pack_inst_voltage}, Pack SOC: {pack_soc}, Relay State: {relay_state}, Checksum: {checksum}")
                                }
                                d => println!("Frame: {:?}, Results: {d:?}", frame),
                            }
                        }
                        _ => println!(
                            "{:?}{}",
                            frame,
                            if frame.is_extended() { " ext" } else { "" }
                        ),
                    }
                }
                (_, _) => println!(
                    "{:?}{}",
                    frame,
                    if frame.is_extended() { " ext" } else { "" }
                ),
            }
        }
    }
}
