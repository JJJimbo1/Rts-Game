
use std::collections::VecDeque;
use bevy::prelude::*;
use hashbrown::HashSet;
use crate::{Pather, PathFinder, Path, PathingGridMap, PathingGridSpace};

pub fn grid_space_update_system<
    PG: Resource + PathingGridMap,
    PP: Resource + PathingGridSpace,
> (
    mut grid : ResMut<PG>,
    mut space : ResMut<PP>,
) {
    if grid.is_changed() {
        space.grid_space().width = grid.grid_map().grid().size_x();
        space.grid_space().length = grid.grid_map().grid().size_y();

        let x_offset = ((grid.grid_map().grid().size_x() % 2) as f32 / 2.0 - 0.5).abs();
        let y_offset = ((grid.grid_map().grid().size_y() % 2) as f32 / 2.0 - 0.5).abs();
        space.grid_space().even_offset = Vec2::new(x_offset, y_offset);
    }
}

pub fn path_finding_system<
    PG: Resource + PathingGridMap,
    PP: Resource + PathingGridSpace,
    PS: Resource + Pather,
    PF: Send + Sync + PathFinder
> (
    mut queue: Local<VecDeque<Entity>>,
    mut grid: ResMut<PG>,
    mut space: ResMut<PP>,
    mut settings: ResMut<PS>,
    mut path_finders: ParamSet<(
        Query<(&PF, &mut Path)>,
        Query<(Entity, &PF, &mut Path), Changed<PF>>,
    )>,
) {
    settings.reset();
    let mut ents = HashSet::new();

    while let Some(ent) = queue.pop_front() {
        if let Ok((path_finder, mut path)) = path_finders.p0().get_mut(ent) {
            if settings.run() {
                let start_index = space.grid_space().position_to_index(path_finder.start());
                let end_index = space.grid_space().position_to_index(path_finder.end());
                if start_index == end_index { continue; }
                let start_cell = *grid.grid_map().get_cell(start_index.0, start_index.1).unwrap();
                let end_cell = *grid.grid_map().get_cell(end_index.0, end_index.1).unwrap();
                path.0 = grid.grid_map().find_path_and_cache(start_cell, end_cell).map(|mut nodes| { nodes.remove(0); nodes.iter().map(|n| space.grid_space().index_to_position((n.x, n.z))).collect() }).unwrap_or(Vec::default());
                settings.complete(&mut *path);
                ents.insert(ent);
            } else {
                queue.push_front(ent);
                break;
            }
        }
    }

    path_finders.p1().for_each_mut(|(entity, path_finder, mut path)| {
        if !ents.contains(&entity) && settings.run() {
            let start_index = space.grid_space().position_to_index(path_finder.start());
            let end_index = space.grid_space().position_to_index(path_finder.end());
            if start_index == end_index { return; }
            let start_cell = *grid.grid_map().get_cell(start_index.0, start_index.1).unwrap();
            let end_cell = *grid.grid_map().get_cell(end_index.0, end_index.1).unwrap();
            path.0 = grid.grid_map().find_path_and_cache(start_cell, end_cell).map(|mut nodes| { nodes.remove(0); nodes.iter().map(|n| space.grid_space().index_to_position((n.x, n.z))).collect() }).unwrap_or(Vec::default());
            settings.complete(&mut *path);
        } else {
            path.0.clear();
            queue.push_back(entity);
        }
    });
}