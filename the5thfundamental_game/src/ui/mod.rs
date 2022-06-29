pub mod context_ui;
pub mod main_menu_ui;
pub mod debug_ui;
// pub mod map_selection_ui;
pub mod gameplay_ui;
pub mod health_bar_ui;

pub use context_ui::*;
pub use main_menu_ui::*;
pub use debug_ui::*;
// pub use map_selection_ui::map_selection_ui::*;
pub use gameplay_ui::*;
pub use health_bar_ui::*;

use bevy::{prelude::*, input::mouse::MouseButtonInput};
use the5thfundamental_common::{StackData, ActiveQueue};

use crate::{utility::{TEXT_COLOR_PRESS, TEXT_COLOR_NORMAL, TEXT_COLOR_HOVER}, systems::camera::CLICK_BUFFER};
pub trait Menu {
    fn main_container(&self) -> Entity;
    fn open(&self, visible_query: &mut Query<&mut Visibility>, children_query: &Query<&Children>,) -> bool {
        let close = !self.is_open(visible_query);
        set_visible_recursive(true, self.main_container(), visible_query, children_query);
        close
    }

    fn close(&self, visible_query: &mut Query<&mut Visibility>, children_query: &Query<&Children>,) -> bool {
        let open = self.is_open(visible_query);
        set_visible_recursive(false, self.main_container(), visible_query, children_query);
        open
    }

    fn toggle(&self, visible_query: &mut Query<&mut Visibility>, children_query: &Query<&Children>,) {
        if self.is_open(visible_query) {
            self.close(visible_query, children_query);
        } else {
            self.open(visible_query, children_query);
        }
    }

    fn is_open(&self, visible_query: &mut Query<&mut Visibility>) -> bool {
        if let Ok(x) = visible_query.get_mut(self.main_container()) {
            return x.is_visible
        }
        return true;
    }
}

pub fn set_visible_recursive(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<&mut Visibility>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(is_visible, *child, visible_query, children_query);
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum MainMenuButtons {
    TopMenu(TopMenuButtons),
    Campaign(CampaignButtons),
    Skirmish(SkirmishButtons),
}

#[derive(Debug, Clone, Copy)]
pub enum TopMenuButtons {
    Campaign,
    Skirmish,
    Options,
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum CampaignButtons {
    Continue,
    LevelSelect,
    LoadGame,
    CustomGame,
    Back,
}

#[derive(Debug, Clone, Copy)]
pub enum SkirmishButtons {
    Continue,
    NewGame,
    LoadGame,
    Back,
}

#[derive(Debug, Clone)]
#[derive(Component)]
pub enum ContextMenuButtons {
    StructuresTab,
    SupportStructuresTab,
    InfantryTab,
    VehiclesTab,
    AircraftTab,
    WatercraftTab,
    TechnologyTab,
    TranformationTab,
    BeginButton(Option<(Entity, ActiveQueue, StackData)>),
    BeginPlaceBufferedButton(Option<(Entity, StackData)>),
    // BeginUnbufferedButton(Option<(Entity, ActiveTab, StackData)>),
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct InactiveButton;