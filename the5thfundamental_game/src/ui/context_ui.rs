use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use the5thfundamental_common::*;

use crate::{*, utility::assets::{FontAssets, ImageAssets}};

#[derive(Debug, Clone, Copy)]
pub enum ActiveTab {
    None,
    Tab(ActiveQueue),
}

impl ActiveTab {
    pub fn unwrap(&self) -> ActiveQueue {
        match self {
            Self::None => { panic!(); }
            Self::Tab(queue) => { *queue }
        }
    }
}

impl From<ActiveQueue> for ActiveTab {
    fn from(active_queue: ActiveQueue) -> Self {
        Self::Tab(active_queue)
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct UiInfo {
    pub display_name: String,
    pub description: String,
    pub cost: f64,
    pub time_to_build: f64,
    pub power_drain: Option<f64>,
}


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct ContextFocus(pub Option<Entity>);

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct ContextMenu {
    container : Entity,
    // building_tab : Entity,
    // unit_tab : Entity,
    active_tab : ActiveTab,
    list_container : Entity,
    list_icons : Vec<Entity>,
}

impl ContextMenu {
    pub fn show_items(&self,
        entity : Entity,
        queue : &Queue,
        mut texts : Query<&mut Text>,
        mut ui_colors : Query<&mut BackgroundColor>,
        mut context_menu_buttons : Query<&mut ContextMenuButtonsEvent>,
        mut visible_query: Query<&mut Visibility>,
        children_query: Query<&Children>,
    ) {
        set_visible(true, self.list_container, &mut visible_query);
        let stacks = queue.zip_queue.stacks();
        let count = stacks.len().clamp(0, 9);
        // println!("{}", count);
        for i in 0..count {
            let stack_data = stacks[i];
            set_visible(true, self.list_icons[i], &mut visible_query);

            if let (Ok(children), Ok(mut but)) = (children_query.get(self.list_icons[i]), context_menu_buttons.get_mut(self.list_icons[i])) {
                let empty = queue.buffer.height(&stack_data) == 0;
                if !empty && stack_data.buffered {
                    *but = ContextMenuButtonsEvent::BeginPlaceBufferedButton(Some((entity, *stack_data)));
                } else {
                    *but = ContextMenuButtonsEvent::BeginButton(Some((entity, self.active_tab.unwrap(), *stack_data)));
                }
                for child in children.iter() {
                    if let Ok(mut text) = texts.get_mut(*child) {
                        text.sections[0].value = format!("{}: {}", stack_data.object_type, queue.zip_queue.height(stack_data));
                    } else if let Ok(mut texture) = ui_colors.get_mut(*child) {
                        if !empty && stack_data.buffered {
                            *texture = GREEN.into();
                        } else {
                            *texture = BLACK.into();
                        }
                    }
                    // else { println!("1"); }
                }
            }
            // else { println!("2"); }
        }
        for i in count..9 {
            set_visible(false, self.list_icons[i], &mut visible_query);
        }
    }
}

impl Menu for ContextMenu {
    fn main_container(&self) -> Entity {
        self.container
    }
}

pub fn create_context_menu(
    settings : Res<MenuSettings>,
    mut font_assets: Res<FontAssets>,
    mut image_assets: Res<ImageAssets>,
    mut nine_patches : ResMut<Assets<NinePatchBuilder<()>>>,
    mut materials : ResMut<Assets<ColorMaterial>>,
    mut commands : Commands,
) {
    let font = font_assets.roboto.clone();
    let font_size = FONT_SIZE_SMALL * settings.font_size / 1.5;

    let mut entity_commands = commands.spawn(NodeBundle {
        style : Style {
            position_type : PositionType::Absolute,
            position : UiRect {
                top : Val::Px(50.0),
                right : Val::Px(50.0),
                ..Default::default()
            },
            size : Size { width : Val::Px(300.0), height : Val::Px(600.0) },
            justify_content : JustifyContent::Center,
            ..Default::default()
        },
        background_color : DARK_BACKGROUND_COLOR.into(),
        visibility : Visibility { is_visible : true},
        ..Default::default()
    });
    entity_commands.insert(BlocksRaycast);

    let container_entity = entity_commands.id();

    let mut x_value : f32 = 10.0;
    let mut y_value : f32 = 10.0;

    let mut structures_tab = None;
    let mut support_structures_tab = None;
    let mut infantry_tab = None;
    let mut vehicle_tab = None;
    let mut aircraft_tab = None;
    let mut watercraft_tab = None;
    let mut technology_tab = None;
    let mut transformation_entity = None;
    let mut list_entity = None;
    // let mut unit_list_entity = None;
    entity_commands.with_children(|parent| {
        structures_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::StructuresTab)); y_value += 40.0;
        support_structures_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::SupportStructuresTab)); x_value += 72.5; y_value -= 40.0;
        infantry_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::InfantryTab)); y_value += 40.0;
        vehicle_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::VehiclesTab)); x_value += 72.5; y_value -= 40.0;
        aircraft_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::AircraftTab)); y_value += 40.0;
        watercraft_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::WatercraftTab)); x_value += 72.5; y_value -= 40.0;
        technology_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::TechnologyTab)); y_value += 40.0;
        transformation_entity = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtonsEvent::TranformationTab)); y_value += 40.0;
        list_entity = Some(create_list(parent, &mut materials, y_value));
    });

    let mut x : f32 = 10.0;
    let mut y : f32 = 10.0;
    let mut roll : u8 = 0;
    let mut icons = Vec::new();
    //*Buildings

    for _ in 0..9 {
        commands.entity(list_entity.unwrap()).with_children(|parent| {
            let icon = parent.spawn(ButtonBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    position : UiRect {
                        left : Val::Px(x),
                        top : Val::Px(y),
                        ..Default::default()
                    },
                    size : Size { width: Val::Px(80.0), height: Val::Px(80.0) },
                    justify_content : JustifyContent::Center,
                    align_items : AlignItems::Center,
                    ..Default::default()
                },
                background_color : BLACK.into(),
                visibility : Visibility { is_visible : true, },
                ..Default::default()
            }).insert(ContextMenuButtonsEvent::BeginButton(None)).insert(BlocksRaycast)
            .with_children(|parent| {
                parent.spawn(NinePatchBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
                        ..Default::default()
                    },
                    nine_patch_data : NinePatchData {
                        texture: image_assets.white_box.clone(),
                        nine_patch: nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2)),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(BlocksRaycast);
                parent.spawn(TextBundle {
                    style : Style {
                        position_type : PositionType::Absolute,
                        ..Default::default()
                    },
                    text : Text::from_section(
                        "",
                        TextStyle {
                            font : font.clone(),
                            font_size,
                            color : TEXT_COLOR_NORMAL,
                        },
                    ),
                    visibility : Visibility { is_visible : true, },
                    ..Default::default()
                });
            }).insert(BlocksRaycast).id();
            icons.push(icon);
        });


        roll+=1;
        if roll >= 3 {
            y += 90.0;
            x = 10.0;
            roll = 0;
        } else {
            x += 90.0;
        }
    }

    commands.insert_resource(ContextMenu{
        container : container_entity,
        // building_tab : structures_tab.unwrap(),
        // unit_tab : support_structures_tab.unwrap(),
        active_tab : ActiveTab::None,
        list_container : list_entity.unwrap(),
        list_icons : icons,
    })
}

fn create_tab(
    parent : &mut ChildBuilder,
    materials : &mut Assets<ColorMaterial>,
    x : f32,
    y : f32,
    button : ContextMenuButtonsEvent,
) -> Entity {
    let tab = parent.spawn(ButtonBundle {
        style: Style {
            position_type : PositionType::Absolute,
            position: UiRect {
                left : Val::Px(x),
                top : Val::Px(y),
                ..Default::default()
            },
            size: Size::new(Val::Px(62.5), Val::Px(30.0)),
            ..Default::default()
        },
        background_color : LIGHT_BACKGROUND_COLOR.into(),
        ..Default::default()
    }).insert(button).insert(BlocksRaycast).id();
    // *x += 72.5;
    tab
}

fn create_list(
    parent : &mut ChildBuilder,
    materials : &mut Assets<ColorMaterial>,
    y : f32
) -> Entity {
    parent.spawn(NodeBundle {
        style: Style {
            position_type : PositionType::Absolute,
            position: UiRect {
                left : Val::Px(10.0),
                top : Val::Px(y),
                ..Default::default()
            },
            size: Size::new(Val::Px(280.0), Val::Px(280.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color : LIGHT_BACKGROUND_COLOR.into(),
        ..Default::default()
    }).insert(BlocksRaycast).id()
}

pub fn context_menu_update(
    menu : Res<ContextMenu>,
    focus : Res<ContextFocus>,
    queueses : Query<&Queues>,

    texts : Query<&mut Text>,
    colors : Query<&mut BackgroundColor>,
    ctx_buttons : Query<&mut ContextMenuButtonsEvent>,

    mut visible_query: Query<&mut Visibility>,
    children_query: Query<&Children>,
) {

    if let Some((entity, queues)) = focus.0.and_then(|e| queueses.get(e).map_or(None, |q| Some((e, q)))) {
        menu.open(&mut visible_query);
        // println!("{}", queues.count());
        match get_queue(queues, menu.active_tab) {
            Some(x) => {
                menu.show_items(entity, x, texts, colors, ctx_buttons, visible_query, children_query);
            },
            None => {
                set_visible(false, menu.list_container, &mut visible_query);
            }
        }
    } else {
        menu.close(&mut visible_query);
    }
}

pub fn context_menu_event_writer(
    // mut menu : ResMut<ContextMenu>,
    mut context_menu_events : EventWriter<ContextMenuButtonsEvent>,
    interaction_query: Query<
        (&Interaction, &ContextMenuButtonsEvent, &Visibility),
        (Changed<Interaction>, With<Button>)
    >,
) {
    interaction_query.for_each(|(int, but, visible)| {
        if !visible.is_visible { return; }
        match int {
            Interaction::Clicked => {
                context_menu_events.send(but.clone());
            },
            Interaction::Hovered => { },
            Interaction::None => { }
        }
    });
}

pub fn context_menu_event_reader(
    // master_queues : Res<MasterQueue>,
    mut context_menu_events : EventReader<ContextMenuButtonsEvent>,
    mut menu : ResMut<ContextMenu>,
    mut current_placement : ResMut<CurrentPlacement<CLICK_BUFFER>>,
    mut queueses : Query<&mut Queues>,
) {
    for event in context_menu_events.iter() {
        match event.clone() {
            ContextMenuButtonsEvent::StructuresTab => { menu.active_tab = ActiveQueue::Structures.into(); }
            ContextMenuButtonsEvent::SupportStructuresTab => { menu.active_tab = ActiveQueue::SupportStructures.into(); }
            ContextMenuButtonsEvent::InfantryTab => { menu.active_tab = ActiveQueue::Infantry.into(); }
            ContextMenuButtonsEvent::VehiclesTab => { menu.active_tab = ActiveQueue::Vehicles.into(); }
            ContextMenuButtonsEvent::AircraftTab => { menu.active_tab = ActiveQueue::Aircraft.into(); }
            ContextMenuButtonsEvent::WatercraftTab => { menu.active_tab = ActiveQueue::Watercraft.into(); }
            ContextMenuButtonsEvent::TechnologyTab => { menu.active_tab = ActiveQueue::Technology.into(); }
            ContextMenuButtonsEvent::TranformationTab => { menu.active_tab = ActiveQueue::Transformation.into(); }
            ContextMenuButtonsEvent::BeginButton(id) => {
                if let Some((entity, tab, stack_data)) = id {
                    if let Ok(mut queues) = queueses.get_mut(entity) {
                        if let Some(queue) = queues.queues.get_mut(&tab) {
                            queue.enqueue(stack_data);
                        }
                    }
                }
            },
            ContextMenuButtonsEvent::BeginPlaceBufferedButton(id) => {
                if let Some((entity, stack_data)) = id {
                    if !current_placement.placing() {
                        // current_placement.constructor = Some(entity);
                        // current_placement.data = Some(stack_data);
                        let ppi = PrePlacementInfo {
                            constructor: entity,
                            queue: menu.active_tab.unwrap(),
                            data: stack_data,
                        };
                        current_placement.status = PlacementStatus::Began(ppi);
                    }
                }
            }
        }
        // println!("{:?}", menu.active_tab);
    }
}

pub fn get_queue(queues: &Queues, tab: ActiveTab) -> Option<&Queue> {
    match tab {
        ActiveTab::None => { None },
        ActiveTab::Tab(tab) => {
            queues.queues.get(&tab)
        }
    }
}

pub fn get_queue_mut(queues: &mut Queues, tab: ActiveTab) -> Option<&mut Queue> {
    match tab {
        ActiveTab::None => { None }
        ActiveTab::Tab(tab) => {
            queues.queues.get_mut(&tab)
        }
    }
}