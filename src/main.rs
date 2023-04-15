use socketcan::{CanSocket, Socket};

fn main() {
    let can_socket = CanSocket::open("can0").unwrap();
    loop {
        if let Ok(frame) = can_socket.read_frame() {
            println!("{:?}", frame);
        }
    }
}
