pub mod context_ui;
pub mod main_menu_ui;
pub mod debug_ui;
// pub mod map_selection_ui;
pub mod gameplay_ui;
pub mod health_bar_ui;
pub mod ui_plugin;

pub use context_ui::*;
pub use main_menu_ui::*;
pub use debug_ui::*;
// pub use map_selection_ui::*;
pub use gameplay_ui::*;
pub use health_bar_ui::*;
pub use ui_plugin::*;

use bevy::prelude::*;
use t5f_common::{StackData, ActiveQueue};

pub trait Menu {
    fn main_container(&self) -> Entity;
    fn open(&self, visible_query: &mut Query<(&mut Visibility, &InheritedVisibility)>) -> bool {
        let close = !self.is_open(visible_query);
        set_visible(true, self.main_container(), visible_query);
        close
    }

    fn close(&self, visible_query: &mut Query<(&mut Visibility, &InheritedVisibility)>) -> bool {
        let open = self.is_open(visible_query);
        set_visible(false, self.main_container(), visible_query);
        open
    }

    fn toggle(&self, visible_query: &mut Query<(&mut Visibility, &InheritedVisibility)>) {
        if self.is_open(visible_query) {
            self.close(visible_query);
        } else {
            self.open(visible_query);
        }
    }

    fn is_open(&self, visible_query: &mut Query<(&mut Visibility, &InheritedVisibility)>) -> bool {
        if let Ok((_, comp)) = visible_query.get_mut(self.main_container()) {
            return comp.get();
        }
        return true;
    }
}

pub fn set_visible(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<(&mut Visibility, &InheritedVisibility)>
) {
    if let Ok((mut visible, _)) = visible_query.get_mut(entity) {
        match is_visible {
            true => *visible = Visibility::Inherited,
            false => *visible = Visibility::Hidden
        }
    }

}

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

#[derive(Resource)]
pub struct MenuSettings {
    pub font_size: f32,
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            font_size: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
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

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum MainMenuButtons {
    TopMenu(TopMenuButtonsEvent),
    Campaign(CampaignButtonsEvent),
    Skirmish(SkirmishButtonsEvent),
}

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub enum TopMenuButtonsEvent {
    Campaign,
    Skirmish,
    Options,
    Quit,
}

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub enum CampaignButtonsEvent {
    Continue,
    LevelSelect,
    LoadGame,
    CustomGame,
    Back,
}

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub enum SkirmishButtonsEvent {
    Continue,
    NewGame,
    LoadGame,
    Back,
}

#[derive(Debug, Clone)]
#[derive(Component, Event)]
pub enum ContextMenuButtonsEvent {
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

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct InactiveButton;

pub use constants::*;
pub mod constants {
    use bevy::render::color::Color;

    pub static MENU_WIDTH : f32 = 480.0;
    pub static MENU_HEIGHT : f32 = 1080.0;

    pub static PRIMARY_MENU_MARGIN : f32 = 0.0;
    pub static SECONDARY_MENU_MARGIN : f32 = PRIMARY_MENU_MARGIN + MENU_WIDTH;

    pub static DARK_BACKGROUND_COLOR : Color = Color::rgba_linear(0.03, 0.03, 0.03, 0.9);
    pub static LIGHT_BACKGROUND_COLOR : Color = Color::rgba_linear(0.7, 0.7, 0.7, 0.9);
    pub static BLACK : Color = Color::rgba_linear(0.00, 0.00, 0.00, 1.0);
    pub static GREEN : Color = Color::rgba_linear(0.0, 1.0, 0.0, 1.0);
    pub static EMPTY_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);

    pub static LIGHT_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.25);
    pub static MEDIUM_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.45);
    pub static HARD_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.75);

    pub static FONT_SIZE_SMALL : f32 = 20.0;
    pub static FONT_SIZE_MEDIUM : f32 = 30.0;
    pub static FONT_SIZE_LARGE : f32 = 40.0;
    pub static FONT_SIZE_EXTRA_LARGE : f32 = 60.0;

    pub static FONT_SIZE_HEADER_MUL : f32 = 2.0;

    pub static TEXT_COLOR_NORMAL : Color = Color::rgba_linear(0.8, 0.8, 0.8, 1.0);
    pub static TEXT_COLOR_UNUSED : Color = Color::rgba_linear(0.2, 0.2, 0.2, 1.0);
    pub static TEXT_COLOR_HOVER : Color = Color::rgba_linear(0.5, 0.8, 0.3, 1.0);
    pub static TEXT_COLOR_PRESS : Color = Color::rgba_linear(0.1, 0.4, 0.9, 1.0);
}