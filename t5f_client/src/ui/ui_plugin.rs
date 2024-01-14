use bevy::prelude::*;
use crate::*;




pub struct ClientUIPlugin;

impl ClientUIPlugin {
    pub fn button_updater_system(
        interaction_query: Query<
            (&Interaction, &Children, &InheritedVisibility),
            (Changed<Interaction>, With<Button>, Without<InactiveButton>),
        >,
        mut text_query: Query<&mut Text>,
    ) {
        interaction_query.for_each(|(interaction, children, visible)| {
            children.iter().for_each(|e| {
                if let Ok(mut text) = text_query.get_mut(*e) {
                    match *interaction {
                        Interaction::Pressed => {
                            if visible.get() {
                                text.sections.iter_mut().for_each(|ts| ts.style.color = TEXT_COLOR_PRESS);
                            }
                        }
                        Interaction::Hovered => {
                            if visible.get() {
                                text.sections.iter_mut().for_each(|ts| ts.style.color = TEXT_COLOR_HOVER);
                            }
                        }
                        Interaction::None => {
                            text.sections.iter_mut().for_each(|ts| ts.style.color = TEXT_COLOR_NORMAL);
                        }
                    }
                }
            })
        });
    }
}

impl Plugin for ClientUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::button_updater_system);
    }
}

pub struct ClientUIPlugins;

impl PluginGroup for ClientUIPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<ClientUIPlugins>();
        let group = group
            .add(ClientUIPlugin)
            .add(ContextUIPlugin)
            .add(DebugUIPlugin)
            .add(GamePlayUIPlugin)
            .add(HealthBarUIPlugin)
            .add(MainUIPlugin);

        group
    }
}
