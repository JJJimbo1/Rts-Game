pub use server_validate_system::*;
mod server_validate_system {
    use amethyst::{core::Transform, ecs::{Read, System, SystemData, Write, WriteStorage}, network::simulation::{NetworkSimulationEvent},
        prelude::*, shrev::{EventChannel, ReaderId}
    };
    use log::error;
    use the5thfundamental_common::{Identifiers, InstantiationData, ObjectType, SEPARATOR, ServerCommand, ServerCommands, Snowflake, TeamPlayer, InitRequests};
    use crate::*;

    #[derive(Default)]
    pub struct ServerValidateSystemDesc;

    impl<'a, 'b> SystemDesc<'a, 'b, ServerValidateSystem> for ServerValidateSystemDesc {
        fn build(self, world: &mut World) -> ServerValidateSystem {
            // Creates the EventChannel<NetworkEvent> managed by the ECS.
            <ServerValidateSystem as System<'_>>::SystemData::setup(world);
            // Fetch the change we just created and call `register_reader` to get a
            // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
            // channel.
            let reader = world
                .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
                .register_reader();

                ServerValidateSystem::new(reader)
        }
    }

    /// A simple system that receives a ton of network events.
    pub struct ServerValidateSystem {
        reader: ReaderId<NetworkSimulationEvent>,
    }

    impl ServerValidateSystem {
        pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
            Self { reader }
        }
    }

    impl<'a> System<'a> for ServerValidateSystem {
        type SystemData = (
            Read<'a, EventChannel<NetworkSimulationEvent>>,
            Write<'a, Identifiers>,
            WriteStorage<'a, Snowflake>,
            Write<'a, InitRequests>,
        );
        fn run(&mut self, (event, mut idents, mut snowflakes, mut inits): Self::SystemData) {
            for event in event.read(&mut self.reader) {
                match event {
                    NetworkSimulationEvent::Message(_addr, payload) => {
                        for s in String::from_utf8_lossy(&payload.to_vec()).split(SEPARATOR) { if s == "" {continue;}
                            match bincode::deserialize::<ServerCommands>(s.as_bytes()) {
                                Ok(x) => {
                                    for c in x.commands.iter() {
                                        match *c {
                                            ServerCommand::Validate(sf1, sf2) => {
                                                match idents.get_entity(sf1) {
                                                    Some(e) => {
                                                        //info!("Validating {} to {}", sf1, sf2);
                                                        match snowflakes.insert(e, sf2) { _ => { }}
                                                        idents.insert(sf2, e);
                                                    },
                                                    None => { }
                                                }
                                            },
                                            ServerCommand::CreateBuilding(sf, id) => {
                                                println!("{:?}", ServerCommand::decode(id));
                                                inits.request(ObjectType::Building,
                                                    ServerCommand::decode(id).replace('\0', ""),
                                                InstantiationData{
                                                    transform : Transform::default(),
                                                    spawn_point : None,
                                                    end_point : None,
                                                    team_player : TeamPlayer::new(1, 0),
                                                    multiplayer : true,
                                                    had_identifier : false,
                                                })
                                            }
                                            ServerCommand::CreateUnit(sf, id) => {
                                                inits.request(ObjectType::Unit,
                                                    ServerCommand::decode(id).replace('\0', ""),
                                                InstantiationData{
                                                    transform : Transform::default(),
                                                    spawn_point : None,
                                                    end_point : None,
                                                    team_player : TeamPlayer::new(1, 0),
                                                    multiplayer : true,
                                                    had_identifier : false,
                                                })
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
                    _ => {}
                }
            }
        }
    }
}