use inovo_rs::socket::*;
use std::net::SocketAddr;
use std::thread;

fn client(addr: SocketAddr) -> Result<(), String> {
    let mut client = Stream::connect(50005, addr, None, None)?;
    let mut i = 0;
    loop {
        client.write(format!("{}", i))?;
        i += 1;
        let _ = client.read()?;
    }
}

#[test]
fn socket_test() -> Result<(), String> {
    let mut listener = Listener::new(50004)?;

    let addr = listener.addr()?;

    thread::spawn(move || client(addr));

    let mut stream = listener.accept(None, None)?;

    for _ in 0..100 {
        let msg = stream.read()?;
        println!("{}", msg);
        stream.write("a")?;
    }
    Ok(())
}
