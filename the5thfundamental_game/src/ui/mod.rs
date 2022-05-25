pub mod context_ui;
pub mod main_menu_ui;
pub mod debug_ui;
// pub mod map_selection_ui;
pub mod gameplay_ui;
pub mod health_bar_ui;

pub use context_ui::context_menu::*;
pub use main_menu_ui::main_menu_ui::*;
pub use debug_ui::frame_data_ui::*;
// pub use map_selection_ui::map_selection_ui::*;
pub use gameplay_ui::gameplay_ui::*;
pub use health_bar_ui::*;

use bevy::prelude::*;
use the5thfundamental_common::{StackData, ActiveQueue};
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