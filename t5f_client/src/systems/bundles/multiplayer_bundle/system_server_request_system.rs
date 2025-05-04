pub use server_request_system::*;
mod server_request_system {

    use amethyst::{
        ecs::prelude::{
            System, Write,
        }, network::simulation::{TransportResource}
    };
    use lazy_static::__Deref;
    use the5thfundamental_common::{SEPARATOR, ServerRequests};


    #[derive(Default)]
    pub struct ServerRequestSystem;

    impl<'a> System<'a> for ServerRequestSystem {
        type SystemData = (
            Write<'a, TransportResource>,
            Write<'a, ServerRequests>,
        );
        fn run(&mut self, (mut net, mut requests): Self::SystemData) {
            let server_addr = "127.0.0.1:50150".parse().unwrap();
            net.send(server_addr, &bincode::serialize(requests.deref()).unwrap().as_slice());
            net.send(server_addr, SEPARATOR.as_bytes());
            requests.commands.clear();
        }
    }
}