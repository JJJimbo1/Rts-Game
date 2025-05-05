pub mod multiplayer_state {


    use crate::*;

    #[derive(Default)]
    pub struct MultiplayerState<'a, 'b>{
        pub dispatcher : Option<Dispatcher<'a, 'b>>,
    }

    impl<'a, 'b, 'c, 'd> State<GameData<'a, 'b>, StateEvent<Bindings>> for MultiplayerState<'c, 'd> {
        fn on_start(&mut self, data: States<'_, GameData<'_, '_>>) {
            let States { world, .. } = data;
            let mut dispatcher_builder = DispatcherBuilder::new();

            TcpNetworkBundle::new(None, 20048)
                .build(world, &mut dispatcher_builder)
                .expect("Failed to register TcpNetworkBundle");

            MultiplayerBundle::default()
                .build(world, &mut dispatcher_builder)
                .expect("Failed to register MultiplayerBundle");

            let mut dispatcher = dispatcher_builder.with_pool((*world.read_resource::<ArcThreadPool>()).clone()).build();
            dispatcher.setup(world);

            self.dispatcher = Some(dispatcher);
        }

        fn shadow_update(&mut self, _data: States<'_, GameData<'_, '_>>) {
            if let Some(d) = self.dispatcher.as_mut() {
                d.dispatch(_data.world);
            }
        }
    }

    // #[derive(Default)]
    // pub struct ServerCommandSystemDesc;

    // impl<'a, 'b> SystemDesc<'a, 'b, ServerCommandSystem> for ServerCommandSystemDesc {
    //     fn build(self, world: &mut World) -> ServerCommandSystem {
    //         // Creates the EventChannel<NetworkEvent> managed by the ECS.
    //         <ServerCommandSystem as System<'_>>::SystemData::setup(world);
    //         // Fetch the change we just created and call `register_reader` to get a
    //         // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
    //         // channel.
    //         let reader = world
    //             .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
    //             .register_reader();

    //         ServerCommandSystem::new(reader)
    //     }
    // }

    // /// A simple system that receives a ton of network events.
    // struct ServerCommandSystem {
    //     reader: ReaderId<NetworkSimulationEvent>,
    // }

    // impl ServerCommandSystem {
    //     pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
    //         Self { reader }
    //     }
    // }

    // impl<'a> System<'a> for ServerCommandSystem {
    //     type SystemData = (
    //         Read<'a, NetworkSimulationTime>,
    //         Write<'a, TransportResource>,
    //         Read<'a, EventChannel<NetworkSimulationEvent>>,
    //         Read<'a, Identifiers>,
    //         WriteStorage<'a, Snowflake>,
    //     );
    //     fn run(&mut self, (sim_time, mut net, event, idents, mut snowflakes): Self::SystemData) {
    //         for event in event.read(&mut self.reader) {
    //             match event {
    //                 NetworkSimulationEvent::Message(_addr, payload) => {
    //                     match bincode::deserialize::<ServerCommands>(payload) {
    //                         Ok(x) => {
    //                             warn!("LOL");
    //                             for c in x.commands.iter() {
    //                                 match *c {
    //                                     ServerCommand::Validate(sf1, sf2) => {
    //                                         println!("Old: {:?}, New: {:?}", sf1, sf2);
    //                                         match idents.get_entity(sf1) {
    //                                             Some(e) => {
    //                                                 match snowflakes.insert(e, sf2) { _ => { }}
    //                                             },
    //                                             None => { }
    //                                         }
    //                                     },
    //                                     ServerCommand::Message(m) => {
    //                                         println!("{}", ServerCommand::parse_message(m));
    //                                     }
    //                                     _ => { }
    //                                 }
    //                             }
    //                         },
    //                         Err(e) => {
    //                             error!("{}", e);
    //                         }
    //                     }
    //                 },
    //                 NetworkSimulationEvent::RecvError(e) => {
    //                     error!("Recv Error: {:?}", e);
    //                 }
    //                 NetworkSimulationEvent::SendError(e, msg) => {
    //                     error!("Send Error: {:?}, {:?}", e, msg);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }

    // #[derive(Default)]
    // pub struct ServerRequestSystemDesc;

    // impl<'a, 'b> SystemDesc<'a, 'b, ServerRequestSystem> for ServerRequestSystemDesc {
    //     fn build(self, world: &mut World) -> ServerRequestSystem {
    //         // Creates the EventChannel<NetworkEvent> managed by the ECS.
    //         <ServerRequestSystem as System<'_>>::SystemData::setup(world);
    //         // Fetch the change we just created and call `register_reader` to get a
    //         // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
    //         // channel.
    //         let reader = world
    //             .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
    //             .register_reader();

    //         ServerRequestSystem::new(reader)
    //     }
    // }

    // /// A simple system that receives a ton of network events.
    // struct ServerRequestSystem {
    //     reader: ReaderId<NetworkSimulationEvent>,
    // }

    // impl ServerRequestSystem {
    //     pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
    //         Self { reader }
    //     }
    // }

    // impl<'a> System<'a> for ServerRequestSystem {
    //     type SystemData = (
    //         Read<'a, NetworkSimulationTime>,
    //         Write<'a, TransportResource>,
    //         Read<'a, EventChannel<NetworkSimulationEvent>>,
    //         Write<'a, ServerRequests>,
    //     );
    //     fn run(&mut self, (sim_time, mut net, event, mut requests): Self::SystemData) {

    //         let server_addr = "127.0.0.1:50150".parse().unwrap();
    //         net.write(server_addr, &bincode::serialize(requests.deref()).unwrap().as_slice());
    //         requests.commands.clear();
    //         // for _frame in sim_time.sim_frames_to_run() {
    //         //     //info!("Sending message for sim frame {}.", frame);
    //         //     let payload = String::from("Hey There!");
    //         //     net.write(server_addr, payload.as_bytes());
    //         // }

    //         // for event in event.read(&mut self.reader) {
    //         //     match event {
    //         //         NetworkSimulationEvent::Message(_addr, payload) => info!("Payload: {:?}", payload),
    //         //         NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
    //         //         NetworkSimulationEvent::Disconnect(addr) => info!("Server Disconnected: {}", addr),
    //         //         NetworkSimulationEvent::RecvError(e) => {
    //         //             error!("Recv Error: {:?}", e);
    //         //         }
    //         //         NetworkSimulationEvent::SendError(e, msg) => {
    //         //             error!("Send Error: {:?}, {:?}", e, msg);
    //         //         }
    //         //         _ => {}
    //         //     }
    //         // }
    //     }
    // }
}