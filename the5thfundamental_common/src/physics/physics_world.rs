pub mod physics_world {

    use bevy::{self, math::Vec3, prelude::{Color, Entity, GlobalTransform, Query, Res, ResMut, Transform, Image, Assets}, render::camera::Camera, window::Windows};

    use bevy_prototype_debug_lines::DebugLines;
    use ncollide3d::{
        na::Point3,
        query::Ray,
    };

    use crate::*;

    type PlayerLayers = Vec<(Entity, Vec3)>;
    type TeamLayers = Vec<PlayerLayers>;

    ///TODO:Just about every function in here needs to be optimised. Right now they're testing against every other collider, which is just awful for performance.
    #[derive(Default)]
    pub struct PhysicsWorld {
        layers : Vec<TeamLayers>,
    }

    impl PhysicsWorld {

        pub fn new(team_players : Query<&TeamPlayer>) -> Self {
            let sorted_layers = TeamPlayerWorld::sort(team_players);

            let mut layers : Vec<TeamLayers> = Vec::with_capacity(sorted_layers.len());

            for team in 0..sorted_layers.len() {
                layers.insert(team, Vec::with_capacity(sorted_layers[team].len()));
                for _ in 0..sorted_layers[team].len() {
                    layers[team].insert(team, Vec::with_capacity(100));
                }
            }

            Self {
                layers
            }
        }

        pub fn populate(&mut self, team_players : Query<&TeamPlayer>, query : Query<(Entity, &Transform, &TeamPlayer)>) {
            self.layers.clear();
            let layers = TeamPlayerWorld::sort(team_players);

            self.layers = Vec::with_capacity(layers.len());
            for team in 0..layers.len() {
                self.layers.insert(team, Vec::with_capacity(layers[team].len()));

                for player in 0..layers[team].len() {
                    self.layers[team].insert(player, Vec::with_capacity(100));
                }
            }

            query.for_each(|(ent, tran, tp)| {
                self.layers[tp.team()][tp.player()].push((ent, tran.translation));
            });
        }

        pub fn populated(mut self, team_players : Query<&TeamPlayer>, query : Query<(Entity, &Transform, &TeamPlayer)>) -> Self {
            self.populate(team_players, query);
            self
        }

        pub fn ray_cast(query : &Query<(&Transform, &Collider)>, ray : Ray<f32>) -> Option<RayCastResult> {

            let mut result : Option<RayCastResult> = None;
            let mut closest : f32 = f32::MAX;

            query.for_each(|(tran, col)| {
                let res = col.toi_with_ray(&ray, (tran.translation, tran.rotation));
                match res {
                    Some(x) => {
                        if x.len < closest {
                            closest = x.len;
                            result = Some(x);
                        }
                    },
                    None => { }
                }
            });

            return result;
        }

        pub fn box_cast(&self, extents : (f32, f32, f32, f32), player : TeamPlayer, cam : &Camera, windows : &Windows, cam_transform : &GlobalTransform) -> Vec<Entity> {
            match self.layers.get(player.team()) {
                Some(x) => {
                    match x.get(player.player()) {
                        Some(y) => {
                            let mut results : Vec<Entity> = Vec::new();
                            for col in y.iter() {
                                let center = cam.world_to_screen(windows, cam_transform, col.1);
                                if let Some(center) = center {
                                    if center.x >= extents.0 && center.x <= extents.1 && center.y >= extents.2 && center.y <= extents.3 {
                                        results.push(col.0);
                                    }
                                }
                            }

                            results
                        },
                        None => { println!("something is wrong with team sorter"); Vec::new() }
                    }
                },
                None => { println!("something is wrong with team sorter"); Vec::new() }
            }
        }

        pub fn highlight_single(entity : Entity, query : &Query<(&Transform, &Collider)>, debug : &mut DebugLines) {
            match query.get(entity) {
                Ok((tran, col)) => {
                    match &col.collider() {
                        ColliderType::Box { cuboid : _ } => {

                        }
                        ColliderType::Sphere { ball : _ } => {

                        },
                        ColliderType::Cylinder { cylinder : _ } => {

                        },
                        ColliderType::Mesh { vertex_buffer : _, index_buffer, trimesh } => {
                            let points = trimesh.clone().transformed(&(tran.translation, tran.rotation).convert()).points().to_owned();
                            for tri in index_buffer {
                                let point1 = Point3::new(points[tri[0]].x, points[tri[0]].y, points[tri[0]].z);
                                let point2 = Point3::new(points[tri[1]].x, points[tri[1]].y, points[tri[1]].z);
                                let point3 = Point3::new(points[tri[2]].x, points[tri[2]].y, points[tri[2]].z);

                                debug.line_colored(
                                    Vec3::new(point1[0], point1[1], point1[2]),
                                    Vec3::new(point2[0], point2[1], point2[2]),
                                    0.0,
                                    Color::rgba(0.1, 0.35, 0.45, 1.0),
                                );

                                debug.line_colored(
                                    Vec3::new(point2[0], point2[1], point2[2]),
                                    Vec3::new(point3[0], point3[1], point3[2]),
                                    0.0,
                                    Color::rgba(0.1, 0.35, 0.45, 1.0),
                                );

                                debug.line_colored(
                                    Vec3::new(point3[0], point3[1], point3[2]),
                                    Vec3::new(point1[0], point1[1], point1[2]),
                                    0.0,
                                    Color::rgba(0.1, 0.35, 0.45, 1.0),
                                );
                            }
                        }
                    }
                },
                Err(_) => { }
            }
        }

        pub fn highlight_selected(query : &Query<(&Transform, &Collider, &Selectable)>, debug : &mut DebugLines) {
            query.for_each(|(tran, col, sel)| {
                //TODO: Might be an issue later... idk tho
                if !sel.selected { return; }
                debug.line_colored(
                    Vec3::new(tran.translation.x, tran.translation.y, tran.translation.z),
                    Vec3::new(tran.translation.x, tran.translation.y + col.extent() + 10.0, tran.translation.z),
                    0.0,
                    Color::rgba(1.0, 0.15, 0.15, 1.0)
                );
            });
        }
    }
}