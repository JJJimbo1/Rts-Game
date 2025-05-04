use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crossbeam_channel::{unbounded, bounded, Sender, Receiver};
use pathing::*;

use crate::{Navigation, OptOut, Slim};

#[derive(Resource, Deref)]
pub struct PFStreamInput(Sender<(Entity, Vec2, Vec2)>);

#[derive(Resource, Deref)]
pub struct PFStreamOutput(Receiver<(Entity, Vec<Vec2>)>);

#[derive(Resource, Deref)]
pub struct PFStreamReset(Sender<(GridMap, GridSpace)>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum PathFindingSystems {
    GridSpaceUpdateSystem,
    PathFindingSystem,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub struct GridMap(pub DS2Map);

impl GridMap {
    fn path_find(&self, start: GridPos, end: GridPos) -> Option<Vec<GridPos>> {
        self.0.find_path(start, end)
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub struct GridSpace {
    pub offset: Vec2,
    pub scale: Vec2,
}

impl GridSpace {
    pub fn new(
    ) -> Self {
        Self::default()
    }

    pub fn position_to_index(&self, position: Vec2) -> (isize, isize) {
        (((position.x - self.offset.x) / self.scale.x).round() as isize,
        ((position.y - self.offset.y) / self.scale.y).round() as isize)
    }

    pub fn index_to_position(&self, index: (isize, isize)) -> Vec2 {
        Vec2::new(
            index.0 as f32 * self.scale.x + self.offset.x,
            index.1 as f32 * self.scale.y + self.offset.y
        )
    }
}

impl Default for GridSpace {
    fn default() -> Self {
        Self {
            offset: Vec2::default(),
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
#[derive(Serialize, Deserialize)]
pub enum PathFinder {
    #[default]
    Idle,
    Queued(Vec2, Vec2),
    Ready(Vec<Vec2>),
    ReQueue(Vec<Vec2>, Vec2, Vec2),
}

impl PathFinder {
    pub fn trip(&self) -> Option<(Vec2, Vec2)> {
        match self {
            PathFinder::Queued(start, end) | PathFinder::ReQueue(_, start, end) => { Some((*start, *end)) },
            _ => { None }
        }
    }

    pub fn set_trip(&mut self, (start, end): (Vec2, Vec2)) {
        match self {
            PathFinder::Idle => { *self = PathFinder::Queued(start, end); },
            PathFinder::Queued(_, _) => { *self = PathFinder::Queued(start, end); },
            PathFinder::Ready(path) => { *self = PathFinder::ReQueue(path.clone(), start, end); },
            PathFinder::ReQueue(path, _, _) => { *self = PathFinder::ReQueue(path.clone(), start, end); },
        }
    }

    pub fn path(&self) -> Option<Vec<Vec2>> {
        match self {
            PathFinder::Ready(path) | PathFinder::ReQueue(path, _, _) => { Some(path.clone()) },
            _ => { None }
        }
    }

    pub fn path_mut(&mut self) -> Option<&mut Vec<Vec2>> {
        match self {
            PathFinder::Ready(path) | PathFinder::ReQueue(path, _, _) => { Some(path) },
            _ => { None }
        }
    }
}

impl Slim for PathFinder {
    fn slim(&self) -> Option<Self> {
        match self {
            Self::Idle => None,
            _ => Some(self.clone()),
        }
    }
}

pub struct PathFindingPlugin;

impl PathFindingPlugin {
    pub fn setup(
        map: Res<GridMap>,
        space: Res<GridSpace>,
        mut commands: Commands,
    ) {
        let (input, reader) = unbounded::<(Entity, Vec2, Vec2)>();
        let (sender, output) = unbounded::<(Entity, Vec<Vec2>)>();
        let (reset, resets) = bounded::<(GridMap, GridSpace)>(1);
        let mut map = (*map).clone();
        let mut space = space.clone();
        std::thread::spawn(move || {
            loop {
                for (grid_map, grid_space) in resets.try_iter() {
                    map = grid_map;
                    space = grid_space;
                }
                for (entity, start, end) in reader.try_iter() {
                    let start_index = space.position_to_index(start);
                    let end_index = space.position_to_index(end);
                    if start_index == end_index {
                        let _ = sender.try_send((entity, Vec::new()));
                    }
                    let path = map
                        .path_find(start_index, end_index)
                        .map(|mut nodes| {
                            nodes.remove(0);
                            nodes
                                .iter()
                                .map(|n| space.index_to_position((n.0, n.1)))
                                .collect()
                        })
                        .unwrap_or(Vec::default());
                    let _ = sender.try_send((entity, path));
                }
            }
        });
        commands.insert_resource(PFStreamInput(input));
        commands.insert_resource(PFStreamOutput(output));
        commands.insert_resource(PFStreamReset(reset));
    }

    pub fn grid_update(
        grid: Res<GridMap>,
        space: Res<GridSpace>,
        reset: Res<PFStreamReset>,
    ) {
        if grid.is_changed() || space.is_changed() {
            let _ = reset.try_send((grid.clone(), space.clone()));
        }
    }

    pub fn path_finding_system(
        input: Res<PFStreamInput>,
        output: Res<PFStreamOutput>,
        mut path_finders: ParamSet<(
            Query<(Entity, &PathFinder), (Changed<PathFinder>, Without<OptOut<Navigation>>)>,
            Query<&mut PathFinder>,
        )>,
    ) {
        path_finders.p0().iter().for_each(|(entity, pathfinder)| {
            if let Some((start, end)) = pathfinder.trip() {
                let _ = input.try_send((entity, start, end));
            }
        });

        output.try_iter().for_each(|(entity, path)| {
            let mut p1 = path_finders.p1();
            let Ok(mut path_finder) = p1.get_mut(entity) else { return; };
            *path_finder = PathFinder::Ready(path);
        })
    }
}

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Self::setup)
            .add_systems(Update, (
                Self::grid_update
                    .in_set(PathFindingSystems::GridSpaceUpdateSystem)
                    .run_if(resources_exist),
                Self::path_finding_system
                    .in_set(PathFindingSystems::PathFindingSystem)
                    .after(PathFindingSystems::GridSpaceUpdateSystem)
                    .run_if(resources_exist)
                )
            );
    }
}

fn resources_exist(
    grid_map: Option<Res<GridMap>>,
    grid_space: Option<Res<GridSpace>>,
) -> bool {
    grid_map.is_some() && grid_space.is_some()
}