pub mod map_selection_ui {

    use std::collections::HashMap;

    use amethyst::{assets::{AssetStorage, Handle, Loader}, core::{HiddenPropagate, Parent}, ecs::Entity, ui::*};
    use the5thfundamental_common::{MetaData, QLoader};
    use walkdir::*;
    use log::*;

    use crate::*;

    #[derive(Debug, Clone, Copy)]
    pub struct MapSelectionUi {
        pub container : Entity,
        pub maps : Entity,
        pub back_button : Entity,
    }

    impl MapSelectionUi {
        pub fn new(world : &mut World) -> Self {
            let font = world.read_resource::<QLoader<Handle<FontAsset>>>().get("Roboto-Black").cloned().unwrap_or_else(|| get_default_font(
                &world.read_resource::<Loader>(),
                &world.read_resource::<AssetStorage<FontAsset>>())
            );

            let font_size = FONT_SIZE_HEADER;
            
            let container_transform = UiTransform::new(
                String::from("map_selection_container"),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                0.0,
                0.0,
                0.0,
                430.0,
                1440.0,
            );

            let container_background = UiImage::SolidColor(DARK_BACKGROUND_COLOR);

            let container = world.create_entity_unchecked()
                .with(container_transform)
                .with(container_background)
                .with(HiddenPropagate::new())
            .build();

            let maps_transform = UiTransform::new(
                String::from("maps_container"),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                430.0,
                0.0,
                0.0,
                3010.0,
                1440.0,
            );

            let maps_background = UiImage::SolidColor(BLACK);

            let maps = world.create_entity_unchecked()
                .with(Parent::new(container))
                .with(maps_transform)
                .with(maps_background)
            .build();

            let back_button = UiButtonBuilder::<(), u32>::default()
                //.with_id(1)
                .with_position(0.0, 300.0)
                .with_layer(0.01)
                .with_size(font_size * 10.0, font_size)
                .with_tab_order(0)
                .with_anchor(Anchor::BottomMiddle)
                .with_stretch(Stretch::NoStretch)
                .with_text("Back")
                .with_text_color(TEXT_COLOR_GENERAL)
                .with_font(font.clone())
                .with_font_size(font_size)
                .with_line_mode(LineMode::Single)
                .with_align(Anchor::Middle)
                .with_image(UiImage::SolidColor(EMPTY_COLOR))
                .with_parent(container)
                // .with_press_sound(None)
                // .with_release_sound(None)
                // .with_hover_sound(None)
                .with_press_text_color(TEXT_COLOR_PRESS)
                // .with_press_image(None)
                .with_hover_text_color(TEXT_COLOR_HOVER)
                // .with_hover_image(None)
                //.with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 1.0]))
            .build_from_world(world);

            Self {
                container,
                maps,
                back_button : back_button.1.image_entity,
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
}