use std::{
    ops::{Index, IndexMut, Range},
    slice::{Iter, IterMut},
};

/// Double sided vec. Note that the first value inserted with "push_front" will be considered the "center" of the vec.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DoubleSidedVec<T> {
    center_index : usize,
    items : Vec<T>,
}

impl<T> DoubleSidedVec<T> {
    pub fn new() -> Self {
        Self {
            center_index : 0,
            items : Vec::new(),
        }
    }

    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            center_index : 0,
            items : Vec::with_capacity(capacity),
        }
    }

    pub fn from_elem(element : T, count : usize) -> Self
        where T : Clone
    {
        Self {
            center_index : count / 2,
            items : vec![element; count],
        }
    }

    pub fn from_elem_center(element : T, count : usize, center : usize) -> Self
        where T : Clone
    {
        Self {
            center_index : center,
            items : vec![element; count],
        }
    }

    pub fn from_vec(center_index : usize, items : Vec<T>) -> Self {
        Self {
            center_index,
            items,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.center_index = 0;
    }

    pub fn set_center(&mut self, center : usize) {
        self.center_index = center;
    }

    pub fn push_back(&mut self, value : T) {
        self.items.insert(0, value);
        self.center_index += 1;
    }

    pub fn push_front(&mut self, value : T) {
        self.items.push(value);
    }

    #[cfg(feature = "index_usize")]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn lens(&self) -> (isize, isize) {
        (-(self.center_index as isize), self.items.len() as isize - self.center_index as isize)
    }

    pub fn range(&self) -> Range<isize> {
        -(self.center_index as isize)..(self.items.len() as isize - self.center_index as isize)
    }

    pub fn get(&self, index : isize) -> Option<&T> {
        if (self.center_index as isize + index) < 0 { return None; }
        self.items.get((self.center_index as isize + index) as usize)
    }

    pub fn get_mut(&mut self, index : isize) -> Option<&mut T> {
        if (self.center_index as isize + index) < 0 { return None; }
        self.items.get_mut((self.center_index as isize + index) as usize)
    }

    pub fn iter(&self) -> Iter<T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.items.iter_mut()
    }
}

impl<T> Index<isize> for DoubleSidedVec<T> {
    type Output = T;
    fn index(&self, index: isize) -> &Self::Output {
        &self.items[(self.center_index as isize + index) as usize]
    }
}

impl<T> IndexMut<isize> for DoubleSidedVec<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        &mut self.items[(self.center_index as isize + index) as usize]
    }
}

#[cfg(feature = "index_usize")]
impl<T> Index<usize> for DoubleSidedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[cfg(feature = "index_usize")]
impl<T> IndexMut<usize> for DoubleSidedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

#[macro_export]
macro_rules! double_vec {
    ($elem:expr; $n:expr) => (
        $crate::DoubleSidedVec::from_elem($elem, $n)
    );
    ($($x:expr), + $(,)?) => (
        {
            let mut items = Vec::new();

            $(items.push($x);)*

            $crate::DoubleVec {
                center_index : items.len() / 2,
                items
            }
        }
    );
    ($elem:expr; $n:expr; $c:expr) => (
        $crate::DoubleSidedVec::from_elem_center($elem, $n, $c)
    );
    ($($x:expr), + $(,)?; $c:expr) => (
        {
            let mut items = Vec::new();

            $(items.push($x);)*

            $crate::DoubleVec {
                center_index : $c,
                items,
            }
        }
    );
}