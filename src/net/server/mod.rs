use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{renet::{RenetServer, DefaultChannel, ConnectionConfig}, netcode::{NetcodeServerTransport, ServerConfig, ServerAuthentication, NetcodeServerPlugin}, RenetServerPlugin};

const PROTOCOL_ID: u64 = 7;
const FIXED_TIMESTEP: f64 = 1.0 / 20.0;

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "0.0.0.0:40256".parse().unwrap();
    println!("{:?}", public_addr);
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}

pub struct ServerPlugin;

impl ServerPlugin {
    fn read_messages(
        mut server: ResMut<RenetServer>,
    ) {
        for id in server.clients_id() {
            let mut loop_count = 0;
            while let Some(Some(bytes)) = (loop_count < 50).then(|| server.receive_message(id, DefaultChannel::ReliableOrdered)) {
                println!("MESSAGE RECIEVED");
                server.broadcast_message_except(id, DefaultChannel::ReliableOrdered, bytes);
                loop_count += 1;
            }
        }
    }

}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (server, transport) = new_renet_server();
        app
            .insert_resource(server)
            .insert_resource(transport)
            .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP))
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(FixedUpdate, Self::read_messages)
        ;
    }
}