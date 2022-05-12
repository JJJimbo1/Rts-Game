use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy},
    network::simulation::{tcp::TcpNetworkBundle},
    prelude::*,
    utils::application_root_dir,
    Result,
};
use std::net::{
    TcpListener,
};
mod core;
use crate::core::*;

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let listener = TcpListener::bind("0.0.0.0:50150")?;
    listener.set_nonblocking(true)?;

    let assets_dir = application_root_dir()?.join("./");

    let game_data = GameDataBuilder::default()
        .with_bundle(TcpNetworkBundle::new(Some(listener), 40096))?;

    let mut game = Application::build(assets_dir, LobbyState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, u32::MAX)
        .with_fixed_step_length(Duration::new(0, 20_000_000))
        .build(game_data)?;
    game.run();
    Ok(())
}