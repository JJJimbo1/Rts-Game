pub mod context_menu {
    use bevy::{prelude::*, utils::HashMap};
    use serde::{Serialize, Deserialize};

    use the5thfundamental_common::*;
    use qloader::*;

    use crate::*;

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
    pub struct ContextFocus(pub Option<Entity>);

    #[derive(Clone)]
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
            mut colors : Query<&mut UiColor>,
            mut ctx_buttons : Query<&mut ContextMenuButtons>,
            mut visible_query: Query<&mut Visibility>,
            children_query: Query<&Children>,
        ) {
            set_visible_recursive(true, self.list_container, &mut visible_query, &children_query);
            let stacks = queue.zip_queue.stacks();
            let count = stacks.len().clamp(0, 9);
            // println!("{}", count);
            for i in 0..count {
                let stack = stacks[i];
                set_visible_recursive(true, self.list_icons[i], &mut visible_query, &children_query);

                if let (Ok(children), Ok(mut but)) = (children_query.get(self.list_icons[i]), ctx_buttons.get_mut(self.list_icons[i])) {
                    let empty = queue.data.buffer.iter().filter(|f| *f == stack).count() == 0;
                    if stack.buffered && !empty {
                        *but = ContextMenuButtons::BeginPlaceBufferedButton(Some((entity, *stack)));
                    } else {
                        *but = ContextMenuButtons::BeginButton(Some((entity, self.active_tab.unwrap(), *stack)));
                    }
                    for child in children.iter() {
                        if let Ok(mut text) = texts.get_mut(*child) {
                            text.sections[0].value = format!("{}: {}", stack.object_type.id().to_string(), queue.zip_queue.height(stack));
                        } else if let Ok(mut texture) = colors.get_mut(*child) {
                            if empty {
                                *texture = BLACK.into();
                            } else {
                                *texture = GREEN.into();
                            }
                        }
                        // else { println!("1"); }
                    }
                }
                // else { println!("2"); }
            }
            for i in count..9 {
                set_visible_recursive(false, self.list_icons[i], &mut visible_query, &children_query);
            }
        }
    }

    impl Menu for ContextMenu {
        fn main_container(&self) -> Entity {
            self.container
        }
    }

    ///Chain to populate
    pub fn create_context_menu(
        settings : Res<MenuSettings>,
        textures : Res<QLoader<ImageAsset, AssetServer>>,
        fonts : Res<QLoader<FontAsset, AssetServer>>,
        mut nine_patches : ResMut<Assets<NinePatchBuilder<()>>>,
        mut materials : ResMut<Assets<ColorMaterial>>,
        mut commands : Commands,
    ) {
        let font = fonts.get("square").unwrap().0.clone();
        let font_size = FONT_SIZE_SMALL * settings.font_size / 1.5;

        let mut entity_commands = commands.spawn_bundle(NodeBundle {
            style : Style {
                position_type : PositionType::Absolute,
                position : Rect {
                    top : Val::Px(50.0),
                    right : Val::Px(50.0),
                    ..Default::default()
                },
                size : Size { width : Val::Px(300.0), height : Val::Px(600.0) },
                justify_content : JustifyContent::Center,
                ..Default::default()
            },
            color : UiColor(DARK_BACKGROUND_COLOR.into()),
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
            structures_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::StructuresTab)); y_value += 40.0;
            support_structures_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::SupportStructuresTab)); x_value += 72.5; y_value -= 40.0;
            infantry_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::InfantryTab)); y_value += 40.0;
            vehicle_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::VehiclesTab)); x_value += 72.5; y_value -= 40.0;
            aircraft_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::AircraftTab)); y_value += 40.0;
            watercraft_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::WatercraftTab)); x_value += 72.5; y_value -= 40.0;
            technology_tab = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::TechnologyTab)); y_value += 40.0;
            transformation_entity = Some(create_tab(parent, &mut materials, x_value, y_value, ContextMenuButtons::TranformationTab)); y_value += 40.0;
            list_entity = Some(create_list(parent, &mut materials, y_value));
        });

        // let mut building_list = HashMap::new();
        // building_list.insert("building_list".to_string(), (building_list_entity, None));

        // let mut unit_list = HashMap::new();
        // unit_list.insert("unit_list".to_string(), (unit_list_entity, None));
        let mut x : f32 = 10.0;
        let mut y : f32 = 10.0;
        let mut roll : u8 = 0;
        let mut icons = Vec::new();
        //*Buildings

        for _ in 0..9 {
            commands.entity(list_entity.unwrap()).with_children(|parent| {
                let icon = parent.spawn_bundle(ButtonBundle {
                    style : Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Px(x),
                            top : Val::Px(y),
                            ..Default::default()
                        },
                        size : Size { width: Val::Px(80.0), height: Val::Px(80.0) },
                        justify_content : JustifyContent::Center,
                        align_items : AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(BLACK.into()),
                    visibility : Visibility { is_visible : true, },
                    ..Default::default()
                }).insert(ContextMenuButtons::BeginButton(None)).insert(BlocksRaycast)
                .with_children(|parent| {
                    parent.spawn_bundle(NinePatchBundle {
                        style : Style {
                            position_type : PositionType::Absolute,
                            size : Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
                            ..Default::default()
                        },
                        nine_patch_data : NinePatchData {
                            texture : textures.get("white_box").unwrap().0.clone(),
                            nine_patch : nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(BlocksRaycast);
                    parent.spawn_bundle(TextBundle {
                        style : Style {
                            position_type : PositionType::Absolute,
                            ..Default::default()
                        },
                        text : Text::with_section(
                            "",
                            TextStyle {
                                font : font.clone(),
                                font_size,
                                color : TEXT_COLOR_NORMAL,
                            },
                            Default::default()
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
        button : ContextMenuButtons,
    ) -> Entity {
        let tab = parent.spawn_bundle(ButtonBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position: Rect {
                    left : Val::Px(x),
                    top : Val::Px(y),
                    ..Default::default()
                },
                size: Size::new(Val::Px(62.5), Val::Px(30.0)),
                ..Default::default()
            },
            color : UiColor(LIGHT_BACKGROUND_COLOR.into()),
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
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position: Rect {
                    left : Val::Px(10.0),
                    top : Val::Px(y),
                    ..Default::default()
                },
                size: Size::new(Val::Px(280.0), Val::Px(280.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color : UiColor(LIGHT_BACKGROUND_COLOR.into()),
            ..Default::default()
        }).insert(BlocksRaycast).id()
    }

    pub fn context_menu_update_system(
        menu : Res<ContextMenu>,
        focus : Res<ContextFocus>,
        queueses : Query<&Queues>,

        texts : Query<&mut Text>,
        colors : Query<&mut UiColor>,
        ctx_buttons : Query<&mut ContextMenuButtons>,

        mut visible_query: Query<&mut Visibility>,
        children_query: Query<&Children>,
    ) {

        if let Some((entity, queues)) = focus.0.and_then(|e| queueses.get(e).map_or(None, |q| Some((e, q)))) {
            menu.open(&mut visible_query, &children_query);
            // println!("{}", queues.count());
            match get_queue(queues, menu.active_tab) {
                Some(x) => {
                    menu.show_items(entity, x, texts, colors, ctx_buttons, visible_query, children_query);
                },
                None => {
                    set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                }
            }
        } else {
            menu.close(&mut visible_query, &children_query);
        }
    }

    pub fn context_menu_event_writer_system(
        mut menu : ResMut<ContextMenu>,
        mut context_menu_events : EventWriter<ContextMenuButtons>,
        interaction_query: Query<
            (&Interaction, &ContextMenuButtons, &Visibility),
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

    pub fn context_menu_event_reader_system(
        // master_queues : Res<MasterQueue>,
        mut menu : ResMut<ContextMenu>,
        mut current_placement : ResMut<CurrentPlacement<CLICK_BUFFER>>,
        mut context_menu_events : EventReader<ContextMenuButtons>,
        mut queueses : Query<&mut Queues>,
    ) {
        for event in context_menu_events.iter() {
            match event.clone() {
                ContextMenuButtons::StructuresTab => { menu.active_tab = ActiveQueue::Structures.into(); }
                ContextMenuButtons::SupportStructuresTab => { menu.active_tab = ActiveQueue::SupportStructures.into(); }
                ContextMenuButtons::InfantryTab => { menu.active_tab = ActiveQueue::Infantry.into(); }
                ContextMenuButtons::VehiclesTab => { menu.active_tab = ActiveQueue::Vehicles.into(); }
                ContextMenuButtons::AircraftTab => { menu.active_tab = ActiveQueue::Aircraft.into(); }
                ContextMenuButtons::WatercraftTab => { menu.active_tab = ActiveQueue::Watercraft.into(); }
                ContextMenuButtons::TechnologyTab => { menu.active_tab = ActiveQueue::Technology.into(); }
                ContextMenuButtons::TranformationTab => { menu.active_tab = ActiveQueue::Transformation.into(); }
                ContextMenuButtons::BeginButton(id) => {
                    if let Some((entity, tab, stack_data)) = id {
                        if let Ok(mut queues) = queueses.get_mut(entity) {
                            if let Some(queue) = queues.get_mut(tab) {
                                queue.enqueue(stack_data);
                            }
                        }
                    }
                },
                ContextMenuButtons::BeginPlaceBufferedButton(id) => {
                    if let Some((entity, stack_data)) = id {
                        if !current_placement.placing() {
                            current_placement.constructor = Some(entity);
                            current_placement.data = Some(stack_data);
                            current_placement.status = PlacementStatus::Began;
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
                match tab {
                    ActiveQueue::Structures => { queues.structures_queue.as_ref() }
                    ActiveQueue::SupportStructures => { queues.support_structures_queue.as_ref() }
                    ActiveQueue::Infantry => { queues.infantry_queue.as_ref() }
                    ActiveQueue::Vehicles => { queues.vehicle_queue.as_ref() }
                    ActiveQueue::Aircraft => { queues.aircraft_queue.as_ref() }
                    ActiveQueue::Watercraft => { queues.watercraft_queue.as_ref() }
                    ActiveQueue::Technology => { queues.technology_queue.as_ref() }
                    ActiveQueue::Transformation => { queues.transformation_queue.as_ref() }
                }
            }
        }
    }

    pub fn get_queue_mut(queues: &mut Queues, tab: ActiveTab) -> Option<&mut Queue> {
        match tab {
            ActiveTab::None => { None }
            ActiveTab::Tab(tab) => {
                match tab {
                    ActiveQueue::Structures => { queues.structures_queue.as_mut() }
                    ActiveQueue::SupportStructures => { queues.support_structures_queue.as_mut() }
                    ActiveQueue::Infantry => { queues.infantry_queue.as_mut() }
                    ActiveQueue::Vehicles => { queues.vehicle_queue.as_mut() }
                    ActiveQueue::Aircraft => { queues.aircraft_queue.as_mut() }
                    ActiveQueue::Watercraft => { queues.watercraft_queue.as_mut() }
                    ActiveQueue::Technology => { queues.technology_queue.as_mut() }
                    ActiveQueue::Transformation => { queues.transformation_queue.as_mut() }
                }
            }
        }
    }
}