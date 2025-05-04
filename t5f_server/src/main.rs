use std::{io::Write, net::TcpListener};

use bevy::prelude::*;
use t5f_server::ServerPlugin;

fn main() {
    // let listener = TcpListener::bind("0.0.0.0:40256").unwrap();

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             stream.write(&[9, 23, 5]).unwrap();

    //         },
    //         Err(_) => { }
    //     }

    //     println!("Connection established!");
    // }
    App::new()

    .add_plugins((MinimalPlugins, ServerPlugin))
    .run();
}