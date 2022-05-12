pub use main_menu_ui::*;
mod main_menu_ui {

    use bevy::prelude::*;
    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    pub enum UiSystems {
        ButtonUpdaterSystem,
        MainMenuButtonEventWriterSystem,
    }

    // pub fn ui_system_set(set : SystemSet) -> SystemSet {
    //     set.label(SystemSets::MainMenuUi)
    //         .with_system(button_updater_system.system().label(UiSystems::ButtonUpdaterSystem))
    //         .with_system(main_menu_button_event_writer_system.system().label(UiSystems::MainMenuButtonEventWriterSystem).after(UiSystems::ButtonUpdaterSystem))

    // }

    pub fn button_updater_system(
        interaction_query: Query<
            (&Interaction, &Children, &Visibility),
            (Changed<Interaction>, With<Button>, Without<InactiveButton>),
        >,
        mut text_query: Query<&mut Text>,
    ) {
        interaction_query.for_each(|(interaction, children, visible)| {
            children.iter().for_each(|e| {
                if let Ok(mut text) = text_query.get_mut(*e) {
                    match *interaction {
                        Interaction::Clicked => {
                            if visible.is_visible {
                                text.sections.iter_mut().for_each(|ts| ts.style.color = TEXT_COLOR_PRESS);
                            }
                        }
                        Interaction::Hovered => {
                            if visible.is_visible {
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
    
    pub fn main_menu_button_event_writer_system(
        mut main_menu_button_events : EventWriter<TopMenuButtons>,
        mut campaign_button_events : EventWriter<CampaignButtons>,
        mut skirmish_button_events : EventWriter<SkirmishButtons>,
        interaction_query: Query<
        (&Interaction, &MainMenuButtons, &Visibility),
        (Changed<Interaction>, With<Button>)>
    ) {
        interaction_query.for_each(|(int, but, visible)| {
            if !visible.is_visible { return; }
            match *int {
                Interaction::Clicked => {
                    match but {
                        MainMenuButtons::TopMenu(mmb) => {
                            main_menu_button_events.send(*mmb);
                        },
                        MainMenuButtons::Campaign(cb) => {
                            campaign_button_events.send(*cb);
                        }
                        MainMenuButtons::Skirmish(sb) => {
                            skirmish_button_events.send(*sb);
                        },

                    }
                },
                Interaction::Hovered => { },
                Interaction::None => { }
            }
        })
    }
}