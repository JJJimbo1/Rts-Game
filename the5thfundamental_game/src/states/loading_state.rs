pub mod loading_state {
    use std::path::Path;

    use bevy::{gltf::Gltf, prelude::*};
    use the5thfundamental_common::*;
    use qloader::*;
    use ronfile::*;
    use crate::*;


    pub fn game_loading_state_on_enter_system_set() -> SystemSet {
        SystemSet::on_enter(GameState::Loading)
            .with_system(game_loading_on_enter)
            .with_system(load_object_prefabs)
            .with_system(load_map_prefabs)
    }

    pub fn game_loading_state_on_update_system_set() -> SystemSet {
        SystemSet::on_update(GameState::Loading)
            .with_system(game_loading_update)
    }

    pub fn game_loading_state_on_exit_system_set() -> SystemSet {
        SystemSet::on_exit(GameState::Loading)
            .with_system(game_loading_on_exit)
            .with_system(cleanup_entities)
    }

    pub fn game_loading_on_enter(server : Res<AssetServer>, mut materials : ResMut<Assets<ColorMaterial>>, mut commands : Commands) {
        let models = QLoader::<GltfAsset, AssetServer>::new(PROJECT_ROOT_DIRECTORY.to_string(), "assets/models".to_string()).with_all_loaded_with(&server);
        let colliders = QLoader::<ColliderAsset, ()>::new(PROJECT_ROOT_DIRECTORY.to_string(), "assets/models".to_string()).with_all_loaded();
        let textures = QLoader::<ImageAsset, AssetServer>::new(PROJECT_ROOT_DIRECTORY.to_string(), "assets/textures".to_string()).with_all_loaded_with(&server);
        let fonts = QLoader::<FontAsset, AssetServer>::new(PROJECT_ROOT_DIRECTORY.to_string(), "assets/fonts".to_string()).with_all_loaded_with(&server);

        commands.insert_resource(models);
        commands.insert_resource(colliders);
        commands.insert_resource(textures);
        commands.insert_resource(fonts);

        commands.insert_resource(RonFile::load_or_default::<CameraSettings, &str>(&format!("{}/config/camera_settings.ron", *PROJECT_ROOT_DIRECTORY)));
        commands.insert_resource(MenuSettings { font_size : 1.0 });
        commands.insert_resource(ButtonMaterials {
            normal: materials.add(TEXT_COLOR_NORMAL.into()),
            hovered: materials.add(TEXT_COLOR_HOVER.into()),
            pressed: materials.add(TEXT_COLOR_PRESS.into()),
        });
        commands.insert_resource(CameraRaycast::default());
        commands.insert_resource(ContextFocus(None));
    }

    pub fn game_loading_update(mut state: ResMut<State<GameState>>) {
        match state.overwrite_set(GameState::MainMenu) { _ => { } }
    }

    pub fn game_loading_on_exit(

    ) {

    }

    pub struct GltfAsset(pub Handle<Gltf>);

    impl QLoad<AssetServer> for GltfAsset {
        const PATHTYPE : PathType = PathType::EXRelative;
        fn extensions() -> Vec<&'static str> {
            vec!["glb", "gltf"]
        }
        fn load_with<S : AsRef<Path>>(path : S, server : &AssetServer) -> Result<Self, QLoaderError> {
            // println!("{}", path.as_ref().display());
            let handle = server.load(path.as_ref().to_path_buf());
            Ok(Self(handle))
        }
    }

    pub struct ImageAsset(pub Handle<Image>);

    impl QLoad<AssetServer> for ImageAsset {
        const PATHTYPE : PathType = PathType::EXRelative;
        fn extensions() -> Vec<&'static str> {
            vec!["png"]
        }
        fn load_with<S : AsRef<Path>>(path : S, server : &AssetServer) -> Result<Self, QLoaderError> {
            // println!("{}", path.as_ref().display());
            // println!("{}", &path.as_ref().display());
            let handle = server.load(path.as_ref().to_path_buf());
            Ok(Self(handle))
        }
    }

    pub struct FontAsset(pub Handle<Font>);

    impl QLoad<AssetServer> for FontAsset {
        const PATHTYPE : PathType = PathType::EXRelative;
        fn extensions() -> Vec<&'static str> {
            vec!["ttf"]
        }
        fn load_with<S : AsRef<Path>>(path : S, server : &AssetServer) -> Result<Self, QLoaderError> {
            // println!("{}", path.as_ref().display());
            let handle = server.load(path.as_ref().to_path_buf());
            Ok(Self(handle))
        }
    }
}