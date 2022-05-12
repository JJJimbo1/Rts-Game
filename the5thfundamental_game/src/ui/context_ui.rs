pub mod context_menu {
    use bevy::prelude::*;

    use the5thfundamental_common::{MasterQueue, Queues};
    use qloader::*;

    use crate::*;

    #[derive(Debug, Clone, Copy)]
    pub struct ContextFocus(pub Option<Entity>);

    #[derive(Clone)]
    pub enum ActiveTab {
        None,
        Building,
        Unit,
    }

    // #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    // pub enum Content {
    //     Title,
    //     Body,
    // }

    #[derive(Clone)]
    pub struct ContextMenu {
        container : Entity,
        building_tab : Entity,
        unit_tab : Entity,
        active_tab : ActiveTab,
        list_container : Entity,
        list_icons : Vec<Entity>,
    }

    impl Menu for ContextMenu {
        fn main_container(&self) -> Entity {
            self.container
        }
    }

    ///Chain to populate
    pub fn create_context_menu(
        settings : Res<MenuSettings>,
        queues : Res<MasterQueue>,
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

        let mut building_tab = None;
        let mut unit_tab = None;
        let mut list_entity = None;
        // let mut unit_list_entity = None;
        entity_commands.with_children(|parent| {
            building_tab = Some(create_tab(parent, &mut materials, &mut x_value, y_value, ContextMenuButtons::BuildingTab));
            unit_tab = Some(create_tab(parent, &mut materials, &mut x_value, y_value, ContextMenuButtons::UnitTab));
            y_value += 40.0;
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

        for b in 0..9 {
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
                }).insert(ContextMenuButtons::BuildBuildingButton(None)).insert(BlocksRaycast)
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
            building_tab : building_tab.unwrap(),
            unit_tab : unit_tab.unwrap(),
            active_tab : ActiveTab::None,
            list_container : list_entity.unwrap(),
            list_icons : icons,
        })
    }

    fn create_tab(
        parent : &mut ChildBuilder,
        materials : &mut Assets<ColorMaterial>,
        x : &mut f32,
        y : f32,
        button : ContextMenuButtons,
    ) -> Entity {
        let tab = parent.spawn_bundle(ButtonBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position: Rect {
                    left : Val::Px(*x),
                    top : Val::Px(y),
                    ..Default::default()
                },
                size: Size::new(Val::Px(62.5), Val::Px(30.0)),
                ..Default::default()
            },
            color : UiColor(LIGHT_BACKGROUND_COLOR.into()),
            ..Default::default()
        }).insert(button).insert(BlocksRaycast).id();
        *x += 72.5;
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

        // mut styles : Query<&mut Style>,

        master_queues : Res<MasterQueue>,
        queues : Query<&Queues>,

        mut texts : Query<&mut Text>,
        mut colors : Query<&mut UiColor>,
        mut ctx_buttons : Query<&mut ContextMenuButtons>,

        mut visible_query: Query<&mut Visibility>,
        children_query: Query<&Children>,
    ) {
        match focus.0 {
            Some(e) => {
                menu.open(&mut visible_query, &children_query);

                match menu.active_tab {
                    ActiveTab::None => {
                        set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                    },
                    ActiveTab::Building => {
                        if let Ok(q) = queues.get(e) {
                            if let Some(bq) = q.building_queue.clone() {
                                set_visible_recursive(true, menu.list_container, &mut visible_query, &children_query);
                                let stacks = bq.stacks();
                                let count = { if stacks.len() < 9 { stacks.len()} else { 9 } };
                                for i in 0..count {
                                    set_visible_recursive(true, menu.list_icons[i], &mut visible_query, &children_query);
                                    if let Some(bud) = master_queues.building_uis.get(&stacks[i].id) {
                                        if let (Ok(children), Ok(mut but)) = (children_query.get(menu.list_icons[i]), ctx_buttons.get_mut(menu.list_icons[i])) {
                                            let empty = if bq.data().buffer.iter().filter(|f| *f == stacks[i]).count() == 0 {
                                                *but = ContextMenuButtons::BuildBuildingButton(Some((e, bud.stack_id.id.clone())));
                                                true
                                            } else {
                                                *but = ContextMenuButtons::BeginPlaceBuildingButton(Some((e, bud.stack_id.id.clone())));
                                                false
                                            };
                                            for child in children.iter() {
                                                if let Ok(mut text) = texts.get_mut(*child) {
                                                    text.sections[0].value = format!("{}: {}", bud.display_name.clone(), bq.height(&stacks[i]));
                                                } else if let Ok(mut texture) = colors.get_mut(*child) {
                                                    if empty {
                                                        *texture = BLACK.into();
                                                    } else {
                                                        *texture = GREEN.into();
                                                    }
                                                } else { println!("1"); }
                                            }
                                        } else { println!("2"); }
                                    } else { println!("3"); }
                                }
                                for i in count..9 {
                                    set_visible_recursive(false, menu.list_icons[i], &mut visible_query, &children_query);
                                }
                            } else {

                                set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                            }
                        } else {
                            set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                        }
                    },
                    ActiveTab::Unit => {
                        if let Ok(q) = queues.get(e) {
                            if let Some(uq) = q.unit_queue.clone() {
                                set_visible_recursive(true, menu.list_container, &mut visible_query, &children_query);
                                let stacks = uq.stacks();
                                let count = { if stacks.len() < 9 { stacks.len()} else { 9 } };
                                for i in 0..count {
                                    set_visible_recursive(true, menu.list_icons[i], &mut visible_query, &children_query);
                                    if let Some(uud) = master_queues.unit_uis.get(&stacks[i].id) {
                                        if let (Ok(children), Ok(mut but)) = (children_query.get(menu.list_icons[i]), ctx_buttons.get_mut(menu.list_icons[i])) {
                                            *but = ContextMenuButtons::TrainUnitButton(Some((e, uud.stack_id.id.clone())));
                                            for child in children.iter() {
                                                if let Ok(mut text) = texts.get_mut(*child) {
                                                    text.sections[0].value = format!("{}: {}", uud.display_name.clone(), uq.height(&stacks[i]));
                                                }
                                            }
                                        }
                                    }
                                }
                                for i in count..9 {
                                    set_visible_recursive(false, menu.list_icons[i], &mut visible_query, &children_query);
                                }
                            } else {
                                set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                            }
                        } else {
                            set_visible_recursive(false, menu.list_container, &mut visible_query, &children_query);
                        }
                    }
                }
            },
            None => {
                menu.close(&mut visible_query, &children_query);
            }
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
        master_queues : Res<MasterQueue>,
        mut menu : ResMut<ContextMenu>,
        mut current_placement : ResMut<CurrentPlacement>,
        mut context_menu_events : EventReader<ContextMenuButtons>,
        mut queues : Query<&mut Queues>,
    ) {
        for event in context_menu_events.iter() {
            match event.clone() {
                ContextMenuButtons::BuildingTab => {
                    menu.active_tab = ActiveTab::Building;
                },
                ContextMenuButtons::UnitTab => {
                    menu.active_tab = ActiveTab::Unit;
                },
                ContextMenuButtons::BuildBuildingButton(id) => {
                    if let Some(x) = id {
                        if let (Ok(mut q), Some(s)) = (queues.get_mut(x.0), master_queues.building_uis.get(&x.1)) {
                            if let Some(bq) = &mut q.building_queue {
                                if bq.is_empty() {
                                    bq.data_mut().timer = s.stack_id.time_to_build.as_secs_f64();
                                }
                                bq.raise_stack(s.stack_id.clone(), 1);
                            }
                        }
                    }
                },
                ContextMenuButtons::BeginPlaceBuildingButton(id) => {
                    if let Some(x) = id {
                        if current_placement.status == PlacementStatus::Idle {
                            if let Some(uud) = master_queues.building_uis.get(&x.1) {
                                current_placement.status = PlacementStatus::Began;
                                current_placement.constructor = Some(x.0);
                                current_placement.data = Some(uud.stack_id.clone());
                            }
                        }
                    }
                }
                ContextMenuButtons::TrainUnitButton(id) => {
                    if let Some(x) = id {
                        if let (Ok(mut q), Some(s)) = (queues.get_mut(x.0), master_queues.unit_uis.get(&x.1)) {
                            if let Some(bq) = &mut q.unit_queue {
                                if bq.is_empty() {
                                    bq.data_mut().timer = s.stack_id.time_to_build.as_secs_f64();
                                }
                                bq.raise_stack(s.stack_id.clone(), 1);
                            }
                        }
                    }
                }
            }
        }
    }
}