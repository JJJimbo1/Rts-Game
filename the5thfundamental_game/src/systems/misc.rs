pub use misc::*;
mod misc {

    use bevy::{prelude::*, render::camera::Camera};
    use qloader::QLoader;
    use the5thfundamental_common::*;
    use crate::*;

    pub fn misc_system_set(set : SystemSet) -> SystemSet {
        set
            .with_system(health_bar_update_system)
            .with_system(health_bar_cleanup_system)
    }

    pub fn health_bar_update_system(
        textures : Res<QLoader<ImageAsset, AssetServer>>,
        mut materials : ResMut<Assets<ColorMaterial>>,
        windows : Res<Windows>,
        images : Res<Assets<Image>>,
        camera : Res<CameraController>,

        mut commands : Commands,

        add_health_bars : Query<(Entity, &Health), Without<HealthBar>>,
        health_bars : Query<(&Transform, &Health, &HealthBar)>,
        mut styles : Query<&mut Style>,
        mut visibles : Query<&mut Visibility>,
        children : Query<&Children>,
        cameras : Query<(&Camera, &GlobalTransform)>,
    ) {
        let mut ents_to_add : Vec<(Entity, u32)> = Vec::new();

        add_health_bars.for_each(|(ent, hel)| {
            ents_to_add.push((ent, (hel.max_health() / 250.0).ceil() as u32));
        });

        // println!("{}", ents_to_add.len());
        for (e, s) in ents_to_add.iter() {
            let health_bar = HealthBar::new(*s, &textures, &mut materials, &mut commands);
            commands.entity(*e).insert(health_bar);
        }

        let cam = cameras.get(camera.camera).unwrap();
        health_bars.for_each(|(tran, hel, bar)| {
            if hel.is_full_health() {
                bar.close(&mut visibles, &children);
            } else {
                bar.open(&mut visibles, &children);
                match cam.0.world_to_screen(&windows, &images, cam.1, tran.translation) {
                    Some(point) => {
                        if let Ok(mut s) = styles.get_mut(bar.main_container()) {
                            let point = point + bar.offset();
                            s.position.left = Val::Px(point.x);
                            //TODO: find some way to get how far up the screen to put the health bar.
                            s.position.bottom = Val::Px(point.y + 50.0);
                        }
                    },
                    None => { },
                }
                bar.adjust_bar_percent(hel.health_percent(), &mut styles);
            }
        });
    }

    pub fn health_bar_cleanup_system(mut dirty_entities : ResMut<DirtyEntities>, query : Query<&HealthBar>, mut commands : Commands) {
        for e in dirty_entities.entities.iter() {
            println!("deleting{:?}", e);
            commands.get_or_spawn(*e).despawn_recursive();
            // commands.entity(*e).despawn_recursive();
            if let Ok(x) = query.get(*e) {
                commands.get_or_spawn(x.main_container()).despawn_recursive();
            }
        }
        dirty_entities.entities.clear();
    }
}