pub mod lobby_state {
    use std::net::IpAddr;

    use amethyst::{core::{ArcThreadPool, SystemBundle}, ecs::{Dispatcher, DispatcherBuilder}, prelude::*};

    use crate::core::{ClientConnectionSystem, Lobby, LobbyBundle};

    #[derive(Default)]
    pub struct LobbyState<'a, 'b> {
        pub dispatcher : Option<Dispatcher<'a, 'b>>,
    }

    impl<'a, 'b> SimpleState for LobbyState<'a, 'b> {
        fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
            let StateData { world, .. } = data;

            let mut lobby = Lobby::default();
            lobby.ban(IpAddr::V4("127.0.0.1".parse().unwrap()));
            //world.insert(lobby);

            let mut dis_b = DispatcherBuilder::new();

            LobbyBundle::default()
                .build(world, &mut dis_b)
                .expect("Failed to register LobbyBundle");

            let mut disp = dis_b.with_pool((*world.read_resource::<ArcThreadPool>()).clone()).build();
            disp.setup(world);

            self.dispatcher = Some(disp);
        }
        fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
            let StateData { world, .. } = data;
            if let Some(x) = self.dispatcher.as_mut() {
                x.dispatch(&world)
            }
            SimpleTrans::None
        }
    }
}