use bevy::{input::mouse::MouseButtonInput, prelude::*, reflect::List};
use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct UiHit<const U: usize> {
    pub hitting : [bool; U],
    pub holding : bool,
}

impl<const U: usize> UiHit<U> {
    pub fn hit(&self) -> bool {
        self.hitting[0] || self.holding
    }
}

#[derive(Component)]
pub struct BlocksRaycast;

pub fn ui_hit_detection_system(
    mut ui_hit : ResMut<UiHit<CLICK_BUFFER>>,

    mut input : EventReader<MouseButtonInput>,

    interaction_query: Query<
        (&Interaction, &Visibility),
        (Changed<Interaction>, With<BlocksRaycast>),
    >,
) {
    *ui_hit.hitting.last_mut().unwrap() = false;
    for b in 1..ui_hit.hitting.len() {
        ui_hit.hitting[b-1] = ui_hit.hitting[b]
    }
    
    interaction_query.for_each(|(int, vis)| {
        if vis.is_visible {
            match int {
                Interaction::Clicked => {
                    for b in 0..ui_hit.hitting.len() {
                        ui_hit.hitting[b] = true;
                    }
                    ui_hit.holding = true;
                },
                // Interaction::None => {
                //     ui_hit.hit = true;
                //     ui_hit.holding = true;
                //     for b in 0..counter.len() {
                //         counter[b] = true;
                //     }
                // }
                _ => { }
            }
        }
    });
    for event in input.iter() {
        match event.state {
            bevy::input::ElementState::Released => {
                if event.button == MouseButton::Left {
                    if ui_hit.holding {
                        ui_hit.holding = false;
                        for b in 0..ui_hit.hitting.len() {
                            ui_hit.hitting[b] = true;
                        }
                    }
                }
            },
            _ => { }
        }
    }
}