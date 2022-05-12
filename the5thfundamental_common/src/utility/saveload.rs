pub use saveload::*;
mod saveload {

    use std::{fmt, fs::{File, OpenOptions}, io::Write, path::Path};

    use bevy::{math::{Quat, Vec3}, prelude::{Component, Entity, Query, Transform}};
    use ron::{de::from_reader, extensions::Extensions, ser::{PrettyConfig, to_string_pretty}};
    use serde::{Serialize, Deserialize};
    use bimap::BiMap;
    use snowflake::ProcessUniqueId;
    use bevy_pathfinding::{PathFinder, Path as FPath};
    use crate::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Manifest {
        //maps : Vec<Map>
    }

    #[derive(Component)]
    pub struct SaveObject {
        pub otype : ObjectType,
        pub prefab : SmallBuffer,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveTags {
        pub health : Option<Health>,
        pub velocity : Option<Velocity>,
        pub finder : Option<PathFinder>,
        pub path : Option<FPath>,
        pub queue : Option<Queues>,
        pub weapons : Option<WeaponSet>,
    }

    impl SaveTags {
        pub fn empty() -> Self {
            Self {
                health : None,
                velocity : None,
                finder : None,
                path : None,
                queue : None,
                weapons : None,
            }
        }
        pub fn is_empty(&self) -> bool {
            self.health.is_none() && self.velocity.is_none() && self.path.is_none() && self.queue.is_none() && self.weapons.is_none()
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveMap(pub String);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SerdeTransform {
        position : (f32, f32, f32),
        rotation : (f32, f32, f32, f32),
        scale : (f32, f32, f32),
    }

    impl SerdeTransform {
        pub fn new_from_transform(trans : &Transform) -> Self {
            let position = (trans.translation.x, trans.translation.y, trans.translation.z);
            let rotation = {
                let angle = trans.rotation.to_axis_angle();
                (angle.0.x, angle.0.y, angle.0.z, angle.1)
            };
            let scale = (trans.scale.x, trans.scale.y, trans.scale.z);
            Self {
                position,
                rotation,
                scale,
            }
        }

        pub fn to_transform(&self) -> Transform {
            let position = Vec3::new(self.position.0, self.position.1, self.position.2);
            let rotation = Quat::from_axis_angle(Vec3::new(self.rotation.0, self.rotation.1, self.rotation.2), self.rotation.3);
            let scale = Vec3::new(self.scale.0, self.scale.1, self.scale.2);
            let mut trans = Transform::default();
            trans.translation = position;
            trans.rotation = rotation;
            trans.scale = scale;
            trans
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveBuilding {
        pub prefab : String,
        pub transform : SerdeTransform,
        pub teamplayer : TeamPlayer,
        pub id : Option<SnowFlake>,
        pub save_tags : Option<SaveTags>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveUnit {
        pub prefab : String,
        pub transform : SerdeTransform,
        pub teamplayer : TeamPlayer,
        pub id : Option<SnowFlake>,
        pub save_tags : Option<SaveTags>,
    }

    ///Target Maximum : 125,829,120
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SaveFile {
        #[serde(skip)]
        pub id_converter : BiMap<SnowFlake, SnowFlake>,
        pub actors : Actors,
        pub map : String,
        pub buildings : Vec<SaveBuilding>,
        pub units : Vec<SaveUnit>,
        pub multiplayer : bool,
    }


    impl SaveFile {
        pub fn new(idents : &Identifiers, actors : &Actors, save_map : &SaveMap, query : &Query<(Entity, &SaveObject, &Transform, &TeamPlayer, &Health, Option<&Velocity>, Option<&PathFinder>, Option<&FPath>, Option<&Queues>, Option<&WeaponSet>)>) -> Self {

            let mut buildings = Vec::new();
            let mut units = Vec::new();

            query.for_each(|(ent, sob, tran, tp, hel, vel, finder, fpath, que, wpn)| {
                if let Some(sf) = idents.get_unique_id(ent) {
                    let save_tags = {
                        let health : Option<Health> = if hel.is_full_health() { None } else { Some(*hel) };

                        let velocity : Option<Velocity> = if let Some(v) = vel {
                            if v.is_zero() { None } else { Some(*v) }
                        } else { None };

                        let queue = if let Some(q) = que {
                            if q.is_empty() { None } else { Some(q.clone()) }
                        } else { None };

                        let weapons = if let Some(w) = wpn {
                            if w.no_targets() { None } else { Some(w.clone()) }
                        } else { None };

                        let st = SaveTags{
                            health,
                            velocity,
                            finder : finder.cloned(),
                            path : fpath.cloned(),
                            queue,
                            weapons,

                        };

                        if st.is_empty() { None } else { Some(st) }
                    };

                    match sob.otype {
                        ObjectType::Building => {
                            buildings.push(
                                SaveBuilding
                                {
                                    prefab : LimitedBuffer::to_string(&sob.prefab),
                                    transform : SerdeTransform::new_from_transform(tran),
                                    teamplayer : *tp,
                                    id : Some(sf),
                                    save_tags,
                                }
                            )
                        },
                        ObjectType::Unit => {
                            units.push(
                                SaveUnit
                                {
                                    prefab : LimitedBuffer::to_string(&sob.prefab),
                                    transform : SerdeTransform::new_from_transform(tran),
                                    teamplayer : *tp,
                                    id : Some(sf),
                                    save_tags,
                                }
                            )
                        }
                    }
                }
            });

            Self{
                id_converter : BiMap::new(),
                actors : actors.clone(),
                map : save_map.0.clone(),
                buildings,
                units,
                multiplayer : false,
            }
        }
    }

    pub fn save_to_file<S : Serialize, P : AsRef<Path>>(item : &S, path : P) -> Result<(), SaveLoadError> {
        let mut _f = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(path) {
                Ok(mut x) => {
                    match x.set_len(0) {
                        Ok(_) => { },
                        Err(e) => { log::error!("{}", e); }
                    }
                    let pretty = PrettyConfig::new()
                        .depth_limit(usize::MAX)
                        .extensions(Extensions::IMPLICIT_SOME);
                    let s = to_string_pretty(item, pretty).expect("Serialization failed");

                    // match x.write(s.as_bytes()) {
                    match bincode::serialize(item) {
                        Ok(i) => {
                            // return Ok(())
                            match x.write(&i) {
                                Ok(_) => { return Ok(()); },
                                Err(e) => { log::error!("{}", e); return Err(SaveLoadError::FileWriteError); }
                            }
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            return Err(SaveLoadError::FileWriteError);
                        }
                    }
                },
                Err(e) => { log::error!("{}", e); return Err(SaveLoadError::FileWriteError); }
        };
    }

    pub fn load_from_file<D : std::fmt::Debug, P : AsRef<Path>>(path : P) -> Result<D, SaveLoadError> where for<'de> D : Deserialize<'de> {
        match OpenOptions::new()
            .read(true)
            .write(false)
            .open(&path) {
                Ok(x) => {
                    let tf = from_reader::<File, D>(x);
                    match tf {
                        Ok(t) => { return Ok(t); }
                        Err(e) => {
                            println!("{}", e);
                            // return Err(SaveLoadError::FileReadError)
                        }
                    }
                },
                Err(_) => { }
        };
        match OpenOptions::new()
            .read(true)
            .write(false)
            .open(path) {
                Ok(x) => {
                    let d = bincode::deserialize_from::<File, D>(x);
                    match d {
                        Ok(t) => { return Ok(t); }
                        Err(_) => { return Err(SaveLoadError::FileReadError); }
                    }
                },
                Err(_) => { }
        };
        return Err(SaveLoadError::FileReadError);
    }

    #[derive(Debug)]
    pub enum SaveLoadError {
        MapNotFoundError(String),
        ModelNotFoundError(String),
        ObjNotFoundError(String),
        ColliderError(ColliderError),
        FileWriteError,
        FileReadError,
    }

    impl fmt::Display for SaveLoadError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::MapNotFoundError(s) => {
                    write!(f, "Map file <{}> either not found or currupted", s)
                },
                Self::ModelNotFoundError(s) => {
                    write!(f, "Gltf file <{}> either not found or currupted", s)
                },
                Self::ObjNotFoundError(s) => {
                    write!(f, "Obj file <{}> either not found or currupted", s)
                },
                Self::ColliderError(e) => {
                    write!(f, "{}", e)
                },
                Self::FileWriteError => {
                    write!(f, "Failed to read or serialize file")
                },
                Self::FileReadError => {
                    write!(f, "Failed to write to file")
                }
            }
        }
    }
}