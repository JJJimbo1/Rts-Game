pub mod economy;
pub mod queue;
pub mod resource;

pub use economy::*;
pub use queue::*;
pub use resource::*;

use bevy::{platform::collections::HashMap, prelude::*};
use crate::*;

#[derive(Default)]
pub struct ProductionPlugin;

impl ProductionPlugin {
    fn resource_adder_system(
        time: Res<Time>,
        mut actors: ResMut<Commanders>,
        query: Query<(&TeamPlayer, &EconomicObject)>
    ) {
        let mut add: HashMap<TeamPlayer, (u32, f64)> = HashMap::new();
        for a in actors.commanders.iter() {
            add.insert(*a.0, (0, 0.0));
        }
        query.iter().for_each(|(tp, res)| {
            if let Some(x) = add.get_mut(tp) {
                x.0 += 1;
                x.1 += res.resource_gen - res.resource_drain;
            }
        });
        for (id, actor) in actors.commanders.iter_mut() {
            let mut to_add = add[id];
            to_add.1 *= time.delta_secs() as f64;
            actor.economy.add_resources(to_add);
        }
    }

    fn queue_system(
        time: Res<Time>,
        mut actors: ResMut<Commanders>,
        mut queues: Query<(&TeamPlayer, &mut Queues)>
    ) {
        queues.iter_mut().for_each(|(team_player, mut queues)| {
            if let Some(actor) = actors.commanders.get_mut(team_player) {
                for queue in queues.queues.values_mut() {
                    if let Some(stack_data) = queue.next() {
                        let cost_this_frame = stack_data.cost as f64 / stack_data.time_to_build.as_secs_f64() * queue.time_left(time.delta_secs_f64());
                        if actor.economy.remove_resources(cost_this_frame) && { queue.update(time.delta_secs_f64()); queue.is_ready() } {
                            let data = queue.advance().unwrap();
                            queue.push_to_buffer(data);
                        }
                    }
                }
            }
        })
    }
}

impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        let _actors = app.world_mut().get_resource_or_insert_with(|| Commanders::default()).clone();

        app
            .add_systems(Update, (
                Self::resource_adder_system,
                Self::queue_system.after(Self::resource_adder_system))
            )
        ;
    }
}
