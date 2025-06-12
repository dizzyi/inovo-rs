use inovo_rs::socket::*;
use std::net::SocketAddr;
use std::thread;
use tracing::info;

const MSG_COUNT: u16 = 100;
const SERVER_PORT: u16 = 50003;

fn client(port: u16, addr: SocketAddr) -> Result<(), std::io::Error> {
    let mut client = Stream::connect(port, addr)?;
    for i in 0..MSG_COUNT {
        client.write(format!("{} send {}", port, i))?;
        let _ = client.read()?;
    }
    Ok(())
}

fn handle(mut stream: Stream) -> Result<(), std::io::Error> {
    for i in 0..MSG_COUNT {
        let _ = stream.read()?;
        stream.write(format!("server response {}", i))?;
    }
    Ok(())
}

#[test]
fn socket_test() -> Result<(), std::io::Error> {
    let mut listener = Listener::new(SERVER_PORT)?;

    let addr = listener.addr()?;

    let ports = 50004..50009;

    let mut handles = vec![];

    for port in ports.clone() {
        handles.push(thread::spawn(move || client(port, addr)));
    }

    info!("Spawned all client threads.");

    for _ in ports {
        let stream = listener.accept().unwrap();
        handles.push(thread::spawn(move || handle(stream)));
    }

    info!("Spawned all handle theads.");

    for h in handles {
        h.join().unwrap().unwrap();
    }

    Ok(())
}
