use std::ops::{Index, IndexMut, Range};
use crate::{double_sided_vec::*, double_vec};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Grid2<C> {
    size_x : usize,
    size_y : usize,
    center_x : usize,
    center_y : usize,
    cells : DoubleSidedVec<DoubleSidedVec<C>>,
}

impl<C> Grid2<C> {
    pub fn new(size_x : usize, size_y : usize, default : C) -> Self
        where C : Clone
    {
        Self {
            size_x,
            size_y,
            center_x : size_x / 2,
            center_y : size_y / 2,
            cells : double_vec![double_vec![default; size_y]; size_x],
        }
    }

    pub fn set_center_x(&mut self, center : usize) {
        self.center_x = center;
        self.cells.set_center(center);
    }

    pub fn set_center_y(&mut self, center : usize) {
        self.center_y = center;
        self.cells.iter_mut().for_each(|dv| dv.set_center(center));
    }

    pub fn expand_neg_x(&mut self, amount : usize)
        where C : Clone + Default
    {
        for _ in 0..amount {
            self.cells.push_back(double_vec![C::default(); self.size_y; self.center_y]);
        }
        self.size_x += amount;
        self.center_x += amount;
    }

    pub fn expand_pos_x(&mut self, amount : usize)
        where C : Clone + Default
    {
        for _ in 0..amount {
            self.cells.push_front(double_vec![C::default(); self.size_y; self.center_y]);
        }
    }

    pub fn expand_neg_y(&mut self, amount : usize)
        where C : Clone + Default
    {

        for dv in self.cells.iter_mut() {
            for _ in 0..amount {
                dv.push_back(C::default());
            }
        }
        self.size_y += amount;
        self.center_y += amount;
    }

    pub fn expand_pos_y(&mut self, amount : usize)
        where C : Clone + Default
    {
        for dv in self.cells.iter_mut() {
            for _ in 0..amount {
                dv.push_front(C::default());
            }
        }
    }

    pub fn move_cell(&mut self, locations : ((isize, isize), (isize, isize)))
        where C : Clone + Default
    {
        self.cells[locations.1.0][locations.1.1] = self.cells[locations.0.0][locations.0.1].clone();
        self.cells[locations.0.0][locations.0.1] = C::default();
    }

    pub fn swap_cells(&mut self, locations : ((isize, isize), (isize, isize)))
        where C : Clone
    {
        let temp = self.cells[locations.1.0][locations.1.1].clone();
        self.cells[locations.1.0][locations.1.1] = self.cells[locations.0.0][locations.0.1].clone();
        self.cells[locations.0.0][locations.0.1] = temp;
    }

    pub fn grid(&self) -> &DoubleSidedVec<DoubleSidedVec<C>> {
        &self.cells
    }

    ///Do **not** modify the size of the grid directly unless you really know what you're doing.
    pub fn grid_mut(&mut self) -> &DoubleSidedVec<DoubleSidedVec<C>> {
        &self.cells
    }

    pub fn get(&self, location : (isize, isize)) -> Option<&C> {
        self.cells.get(location.0).and_then(|f| f.get(location.1))
    }

    pub fn get_mut(&mut self, location : (isize, isize)) -> Option<&mut C> {
        self.cells.get_mut(location.0).and_then(|f| f.get_mut(location.1))
    }

    pub fn cells(&self) -> Vec<&C> {
        self.cells.iter().flat_map(|f| f.iter()).collect::<Vec<&C>>()
    }

    pub fn cells_mut(&mut self) -> Vec<&mut C> {
        self.cells.iter_mut().flat_map(|f| f.iter_mut()).collect::<Vec<&mut C>>()
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }

    pub fn size_y(&self) -> usize {
        self.size_y
    }

    pub fn center_x(&self) -> usize {
        self.center_x
    }

    pub fn center_y(&self) -> usize {
        self.center_y
    }

    pub fn lens_x(&self) -> (isize, isize) {
        self.cells.lens()
    }

    pub fn lens_y(&self) -> (isize, isize) {
        if let Some(x) = self.cells.get(0) {
            x.lens()
        } else {
            (0, 0)
        }
    }

    pub fn range_x(&self) -> Range<isize> {
        self.cells.range()
    }

    pub fn range_y(&self) -> Range<isize> {
        if let Some(x) = self.cells.get(0) {
            x.range()
        } else {
            0..0
        }
    }
}

impl<C> Index<(isize, isize)> for Grid2<C> {
    type Output = C;
    fn index(&self, index: (isize, isize)) -> &Self::Output {
        &self.cells[index.0][index.1]
    }
}

impl<C> IndexMut<(isize, isize)> for Grid2<C> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        &mut self.cells[index.0][index.1]
    }
}

#[cfg(feature = "index_usize")]
impl<C> Index<(usize, usize)> for Grid2<C> {
    type Output = C;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

#[cfg(feature = "index_usize")]
impl<C> IndexMut<(usize, usize)> for Grid2<C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}

impl<C> Default for Grid2<C> {
    fn default() -> Self {
        Self {
            size_x : 0,
            size_y : 0,
            center_x : 0,
            center_y : 0,
            cells : DoubleSidedVec::new(),
        }
    }
}
