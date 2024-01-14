use std::collections::HashMap;
use qloader::QLoader;
use walkdir::*;
use log::*;
use crate::*;

pub fn create_map_selection_ui(
    settings : Res<MenuSettings>,
    textures : Res<QLoader<ImageAsset, AssetServer>>,
    fonts : Res<QLoader<FontAsset, AssetServer>>,
    mut commands : Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let main_menu = MapSelectionUi::new(&settings, &textures, &fonts, &mut materials, &mut commands);
    commands.insert_resource(main_menu);
}

#[derive(Debug, Clone, Copy)]
pub struct MapSelectionUi {
    // pub side_bar: Entity,
    // pub backround: Entity,

    pub container : Entity,
    // pub maps : Entity,
    // pub back_button : Entity,
}



impl MapSelectionUi {
    pub fn new(
        settings : &MenuSettings,
        textures : &QLoader<ImageAsset, AssetServer>,
        fonts : &QLoader<FontAsset, AssetServer>,
        materials: &mut Assets<ColorMaterial>,
        commands : &mut Commands,
    ) -> Self {
        let font = fonts.get("Roboto-Black").unwrap().0.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position : UiRect {
                    left : Val::Px(PRIMARY_MENU_MARGIN),
                    ..Default::default()
                },
                size: Size::new(Val::Px(MENU_WIDTH), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color : UiColor(DARK_BACKGROUND_COLOR.into()),
            visibility : Visibility { is_visible : true},
            ..Default::default()
        });
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position : UiRect {
                        left : Val::Px(PRIMARY_MENU_MARGIN),
                        top: Val::Px(100.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(MENU_WIDTH), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color : UiColor(DARK_BACKGROUND_COLOR.into()),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            })
        });

        Self {
            container,
            // maps,
            // back_button : back_button.1.image_entity,
        }
    }

    /*pub fn populate(mut self, maps_folder : &str) -> Self {
        let mut maps = Vec::<LevelMetaData>::new();
        let entries = WalkDir::new(maps_folder);

        for file in entries {
            match file {
                Ok(x) => {
                    match LevelMetaData::load(x.into_path()) {
                        Ok(y) => {
                            maps.push(y)
                        },
                        Err(e) => {
                            info!("{}", e);
                        }
                    }
                },
                Err(e) => {
                    info!("{}", e);
                }
            }
        }

        self.levels = maps;
        self
    }*/

    pub fn generate_menu(&mut self, world : &mut World) {

        /*let mut texture_loader = world.write_resource::<TextureLoader>();

        let texture = texture_loader.load("white_dotted_box").unwrap();*/

        //let loader = world.read_resource::<Loader>();
        //let storage = world.read_resource::<AssetStorage<FontAsset>>();

        // let trans = UiTransform::new(String::from("What"), Anchor::MiddleRight, Anchor::MiddleRight, 0.0, 0.0, 0.0, 100.0, 800.0);
        // let text = UiText::new(get_default_font(&loader, &storage), String::from("what"), [0.5, 0.5, 0.8, 1.0], 40.0, LineMode::Wrap, Anchor::Middle);
        // let te = UiImage::NineSlice{
        //     x_start : 0,
        //     y_start : 0,
        //     width : 36,
        //     height : 36,
        //     left_dist : 12,
        //     right_dist : 12,
        //     top_dist : 12,
        //     bottom_dist : 12,
        //     tex : texture.clone(),
        //     texture_dimensions : [36, 36]
        // };


        // world.create_entity_unchecked()
        //     .with(trans)
        //     .with(te)
        //     .with(text)
        //     .build();
    }

    pub fn handle_click(&self, target : Entity, world : &mut World) {
        
    }
}

impl Menu for MapSelectionUi {
    fn main_container(&self) -> Entity {
        self.container
    }
}