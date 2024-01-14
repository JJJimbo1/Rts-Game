pub use message_system::*;
mod message_system {

    use amethyst::{ecs::{Read, System, SystemData, Write, WriteStorage}, network::simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource},
        prelude::*, shrev::{EventChannel, ReaderId}
    };
    use log::{error, info};
    use the5thfundamental_common::{Identifiers, SEPARATOR, ServerCommand, ServerCommands, Snowflake};

    #[derive(Default)]
    pub struct ServerMessageSystemDesc;

    impl<'a, 'b> SystemDesc<'a, 'b, ServerMessageSystem> for ServerMessageSystemDesc {
        fn build(self, world: &mut World) -> ServerMessageSystem {
            // Creates the EventChannel<NetworkEvent> managed by the ECS.
            <ServerMessageSystem as System<'_>>::SystemData::setup(world);
            // Fetch the change we just created and call `register_reader` to get a
            // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
            // channel.
            let reader = world
                .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
                .register_reader();

                ServerMessageSystem::new(reader)
        }
    }

    pub struct ServerMessageSystem {
        reader: ReaderId<NetworkSimulationEvent>,
    }

    impl ServerMessageSystem {
        pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
            Self { reader }
        }
    }

    impl<'a> System<'a> for ServerMessageSystem {
        type SystemData = (
            Read<'a, EventChannel<NetworkSimulationEvent>>,
        );
        fn run(&mut self, (event,) : Self::SystemData) {
            for event in event.read(&mut self.reader) {
                match event {
                    NetworkSimulationEvent::Message(_addr, payload) => {
                        for s in String::from_utf8_lossy(&payload.to_vec()).split(SEPARATOR) { if s == "" {continue;}
                            match bincode::deserialize::<ServerCommands>(s.as_bytes()) {
                                Ok(x) => {
                                    for c in x.commands.iter() {
                                        match *c {
                                            ServerCommand::Message(m) => {
                                                //info!("{}", ServerCommand::decode(m));
                                            }
                                            _ => { }
                                        }
                                    }
                                },
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                        }
                    },
                    NetworkSimulationEvent::RecvError(e) => {
                        error!("Recv Error: {:?}", e);
                    }
                    NetworkSimulationEvent::SendError(e, msg) => {
                        error!("Send Error: {:?}, {:?}", e, msg);
                    }
                    _ => {}
                }
            }
        }
    }
}