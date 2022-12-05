



use std::marker::PhantomData;
use bevy::{prelude::*, ecs::schedule::ShouldRun};
use crate::{Pather, DefaultPather, path_finding_system, GridSpace, grid_space_update_system, PathingGridMap, PathingGridSpace, PathFinder, OGrid};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(RunCriteriaLabel)]
pub enum GridExists {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum PathFindingSystems {
    GridSpaceUpdateSystem,
    PathFindingSystem,
}

#[derive(Debug, Clone, Copy)]
pub struct PathFindingPlugin<
    PF: PathFinder,
    PG: Resource + PathingGridMap = OGrid,
    PP: Resource + PathingGridSpace = GridSpace,
    PS: Resource + Pather = DefaultPather,
> {
    pf: PhantomData<PF>,
    pg: PhantomData<PG>,
    pp: PhantomData<PP>,
    ps: PhantomData<PS>,
}

impl<
    PF: PathFinder,
    PG: Resource + PathingGridMap,
    PP: Resource + PathingGridSpace,
    PS: Resource + Pather,
> Default for PathFindingPlugin<PF, PG, PP, PS> {
    fn default() -> Self {
        Self {
            pf: PhantomData,
            pg: PhantomData,
            pp: PhantomData,
            ps: PhantomData,
        }
    }
}

impl<
    PF: PathFinder,
    PG: Resource + PathingGridMap,
    PP: Resource + PathingGridSpace,
    PS: Resource + Pather,
> Plugin for PathFindingPlugin<PF, PG, PP, PS> {
    fn build(&self, app: &mut App) {
        // app.world.get_resource_or_insert_with(|| GridSpace::default());
        app
            .add_system(grid_space_update_system::<PG, PP>
                .label(PathFindingSystems::GridSpaceUpdateSystem)
                .with_run_criteria(resources_exist::<PG, PP, PS>))

            .add_system(path_finding_system::<PG, PP, PS, PF>
                .label(PathFindingSystems::PathFindingSystem)
                .after(PathFindingSystems::GridSpaceUpdateSystem)
                .with_run_criteria(resources_exist::<PG, PP, PS>))

        ;
    }
}

fn resources_exist<
    PG: Resource + PathingGridMap,
    PP: Resource + PathingGridSpace,
    PS: Resource + Pather,
>(
    grid_map: Option<Res<PG>>,
    grid_space: Option<Res<PG>>,
    settings: Option<Res<PG>>,
) -> ShouldRun {
    if grid_map.is_some() && grid_space.is_some() && settings.is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}