pub use multiplayer_bundle::*;
mod multiplayer_bundle {

    use amethyst::{core::bundle::SystemBundle, ecs::prelude::{
            DispatcherBuilder, World,
        }, error::Error, prelude::SystemDesc};
    use crate::{
        ServerRequestSystem, /*ServerMessageSystemDesc, */ServerValidateSystemDesc,
    };

    #[derive(Default)]
    pub struct MultiplayerBundle;

    impl<'a, 'b> SystemBundle<'a, 'b> for MultiplayerBundle {
        fn build(self, world : &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
            builder.add(ServerRequestSystem, "server_request_system", &[]);
            //builder.add(ServerMessageSystemDesc::default().build(world), "server_message_system", &[]);
            builder.add(ServerValidateSystemDesc::default().build(world), "server_validate_system", &[]);
            Ok(())
        }
    }
}