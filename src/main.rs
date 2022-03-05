use std::{io::{Write}, sync::Arc, net::{TcpListener, TcpStream}, thread};

use openssl::ssl::{SslAcceptor, SslMethod, SslVerifyMode, SslStream};

fn main() {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_cipher_list("PSK-AES128-CBC-SHA256:!DSS").unwrap();
    builder.set_verify(SslVerifyMode::NONE);

    builder.set_psk_server_callback(|_ssl, mut _identity, mut _psk| {
        if let Some(id) = _identity {
            println!("identity: {:?}", String::from_utf8_lossy(id));
        }
        let psk = hex::decode("123456").unwrap();
        _psk.write_all(&psk).unwrap();
        Ok(psk.len())
    });

    let acceptor = Arc::new(builder.build());
    
    let tcp_listen = TcpListener::bind("localhost:8443").unwrap();

    for s in tcp_listen.incoming() {
        match s {
            Ok(s) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut sslstream = acceptor.accept(s).unwrap(); 
                    handle_client(&mut sslstream);
                });
            }
            Err(e) => eprintln!("tcp error {:?}", e),
        }
    }
}

fn handle_client(s: &mut SslStream<TcpStream>) {
    let mut buf = [0u8; 1024];
    while let Ok(size) = s.ssl_read(&mut buf) {
        let content = String::from_utf8(buf[0..size].to_vec()).unwrap();
        println!("received: {:?}", content);
        if content.eq("finish\n") {
            s.shutdown().unwrap();
        }
    }
}