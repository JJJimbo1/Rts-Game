use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport}, renet::{ConnectionConfig, DefaultChannel, RenetClient}, RenetClientPlugin};
use crate::*;

const PROTOCOL_ID: u64 = 7;
const FIXED_TIMESTEP: f64 = 1.0 / 20.0;

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind(CLIENT_ADDRESS).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = uuid::Uuid::new_v4().as_u64_pair().0;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

pub struct ClientPlugin;

impl ClientPlugin {
    fn send_messages(
        mut client_events: EventReader<ClientRequest>,
        mut client: ResMut<RenetClient>,
    ) {
        let messages: Vec<ClientRequest> = client_events.read().cloned().collect();
        if messages.len() == 0 { return; };
        let payload: ClientRequests = messages.into();
        let Ok(bytes) = bincode::serialize(&payload) else { return; };
        println!("SENDING");
        client.send_message(DefaultChannel::ReliableOrdered, bytes);
    }

    fn read_messages(
        mut client_events: EventWriter<ServerCommand>,
        mut client: ResMut<RenetClient>,
        mut load_events: EventWriter<LoadObjects>,
    ) {
        let mut loop_count = 0;
        while let Some(Some(bytes)) = (loop_count < 20).then(|| client.receive_message(DefaultChannel::ReliableOrdered)) {
            println!("RECIEVING");
            let Ok(payload) = bincode::deserialize::<ServerCommands>(&bytes) else { continue; };
            for messege in payload.commands {

                client_events.write(messege.clone());
                match messege {
                    ServerCommand::Empty => {},
                    ServerCommand::SpawnObject(event) => {
                        load_events.write(event.into());
                    }
                }
            }
            loop_count += 1;
        }
    }

}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (client, transport) = new_renet_client();
        app
            .add_event::<ClientRequest>()
            .add_event::<ServerCommand>()
            .insert_resource(client)
            .insert_resource(transport)
            .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP))
            .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(FixedUpdate, (Self::send_messages, Self::read_messages))
        ;
    }
}
