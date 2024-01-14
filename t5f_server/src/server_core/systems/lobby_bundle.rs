pub use lobby_bundle::*;
mod lobby_bundle {
    use amethyst::{core::bundle::SystemBundle, ecs::prelude::{
            DispatcherBuilder, World, System, Read, Write,
        }, error::Error, network::simulation::{NetworkSimulationEvent, TransportResource, tcp::TcpNetworkResource}, shrev::{EventChannel, ReaderId}};
    use log::{error, info};
    use the5thfundamental_common::{SEPARATOR, ServerCommand, ServerCommands, ServerRequest, ServerRequests, Snowflake};

    #[derive(Default)]
    pub struct LobbyBundle;

    impl<'a, 'b> SystemBundle<'a, 'b> for LobbyBundle {
        fn build(self, world : &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
            builder.add(ClientConnectionSystem::new(world), "client_connection_system", &[]);
            //builder.add(ClientMessageSystem::new(world), "client_message_system", &["client_connection_system"]);
            builder.add(ClientValidateSystem::new(world), "client_validate_system", &["client_connection_system"]);
            Ok(())
        }
    }

    use super::super::super::{
        Lobby, ID,
    };

    pub struct ClientConnectionSystem {
        reader : ReaderId<NetworkSimulationEvent>,
    }

    impl ClientConnectionSystem {
        pub fn new(world : &mut World) -> Self {
            let reader = world
                .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
                .register_reader();
            Self {
                reader,
            }
        }
    }

    impl<'a> System<'a> for ClientConnectionSystem {
        type SystemData = (
            Write<'a, TransportResource>,
            Read<'a, EventChannel<NetworkSimulationEvent>>,
            Write<'a, Lobby>,
        );

        fn run(&mut self, (mut net, channel, mut lobby) : Self::SystemData) {
            for event in channel.read(&mut self.reader) {
                match event {
                    NetworkSimulationEvent::Connect(addr) => {
                        log::warn!("Bruh look at this duuuuude");
                        match lobby.allowed_to_join(*addr) {
                            Ok(_) => {
                                info!("Client Connected: {}", addr);
                                match lobby.add_player(*addr) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        error!("{}", e);
                                    }
                                }
                                /*for p in lobby.players().iter() {
                                    let mut cos = ServerCommands::default();
                                    cos.commands.push(ServerCommand::Message(ServerCommand::encode("Player Connected to Server")));
                                    net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                    net.send(*p.0, SEPARATOR.as_bytes());
                                }*/
                            },
                            Err(e) => {
                                match e {
                                    crate::core::LobbyError::PlayerIsBanned => {
                                        let mut cos = ServerCommands::default();
                                        /*cos.commands.push(ServerCommand::Message(ServerCommand::encode("You are not allowed to join the server")));
                                        net.send(*addr, &bincode::serialize(&cos).unwrap().as_slice());
                                        net.send(*addr, SEPARATOR.as_bytes());*/
                                        //net_re.drop_stream(*addr);
                                    },
                                    _ => { }
                                }
                            }
                        }
                    },
                    NetworkSimulationEvent::Disconnect(addr) => {
                        info!("Client Disconnected: {}", *addr);
                        match lobby.remove_player(ID::Address(*addr)) {
                            Ok(_) => {},
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub struct ClientMessageSystem {
        reader : ReaderId<NetworkSimulationEvent>,
    }

    impl ClientMessageSystem {
        pub fn new(world : &mut World) -> Self {
            let reader = world
                .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
                .register_reader();
            Self {
                reader,
            }
        }
    }

    /*impl<'a> System<'a> for ClientMessageSystem {
        type SystemData = (
            Write<'a, TransportResource>,
            Read<'a, EventChannel<NetworkSimulationEvent>>,
            Read<'a, Lobby>,
        );

        fn run(&mut self, (mut net, channel, lobby) : Self::SystemData) {
            for event in channel.read(&mut self.reader) {
                match event {
                    NetworkSimulationEvent::Message(addr, bytes) => {
                        println!("{}", bytes.len());
                        for s in String::from_utf8_lossy(&bytes.to_vec()).split(SEPARATOR) { if s == "" {continue;}
                            match bincode::deserialize::<ServerRequests>(s.as_bytes()) {
                                Ok(x) => {
                                    for r in x.commands {
                                        match r {
                                            ServerRequest::Message(m) => {
                                                for p in lobby.players().iter() {
                                                    if addr != p.0 {
                                                        let mut cos = ServerCommands::default();
                                                        cos.commands.push(ServerCommand::Message(m));
                                                        net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                                        net.send(*p.0, SEPARATOR.as_bytes());
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }*/

    pub struct ClientValidateSystem {
        reader : ReaderId<NetworkSimulationEvent>,
    }

    impl ClientValidateSystem {
        pub fn new(world : &mut World) -> Self {
            let reader = world
                .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
                .register_reader();
            Self {
                reader,
            }
        }
    }

    impl<'a> System<'a> for ClientValidateSystem {
        type SystemData = (
            Write<'a, TransportResource>,
            Read<'a, EventChannel<NetworkSimulationEvent>>,
            Read<'a, Lobby>,
        );

        fn run(&mut self, (mut net, channel, lobby) : Self::SystemData) {
            for event in channel.read(&mut self.reader) {
                match event {
                    NetworkSimulationEvent::Message(addr, bytes) => {
                        for s in String::from_utf8_lossy(&bytes.to_vec()).split(SEPARATOR) { if s == "" {continue;}
                            match bincode::deserialize::<ServerRequests>(s.as_bytes()) {
                                Ok(x) => {
                                    for r in x.commands {
                                        match r {
                                            ServerRequest::CreateBuilding(sf, id) => {
                                                println!("{:?}", ServerCommand::decode(id));
                                                let snow = Snowflake::new();
                                                for p in lobby.players().iter() {

                                                    if addr != p.0 {
                                                        let mut cos = ServerCommands::default();
                                                        //cos.commands.push(ServerCommand::Message(ServerCommand::encode(&format!("Create Building with Id: {}", snow))));
                                                        cos.commands.push(ServerCommand::CreateBuilding(sf, id));
                                                        net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                                        net.send(*p.0, SEPARATOR.as_bytes());
                                                    } else {
                                                        let mut cos = ServerCommands::default();
                                                        //cos.commands.push(ServerCommand::Message(ServerCommand::encode(&format!("Validate Building with old Id: {} with new Id: {}", sf, snow))));
                                                        cos.commands.push(ServerCommand::Validate(sf, snow));
                                                        net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                                        net.send(*p.0, SEPARATOR.as_bytes());
                                                    }
                                                }
                                            },
                                            ServerRequest::CreateUnit(sf, id) => {
                                                let snow = Snowflake::new();
                                                for p in lobby.players().iter() {

                                                    if addr != p.0 {
                                                        let mut cos = ServerCommands::default();
                                                        //cos.commands.push(ServerCommand::Message(ServerCommand::encode(&format!("Create Unit with Id: {}", snow))));
                                                        cos.commands.push(ServerCommand::CreateUnit(sf, id));
                                                        net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                                        net.send(*p.0, SEPARATOR.as_bytes());
                                                    } else {
                                                        let mut cos = ServerCommands::default();
                                                        //cos.commands.push(ServerCommand::Message(ServerCommand::encode(&format!("Validate Unit with old Id: {} with new Id: {}", sf, snow))));
                                                        cos.commands.push(ServerCommand::Validate(sf, snow));
                                                        net.send(*p.0, &bincode::serialize(&cos).unwrap().as_slice());
                                                        net.send(*p.0, SEPARATOR.as_bytes());
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}