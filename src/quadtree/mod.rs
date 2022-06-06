use std::borrow::Borrow;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

use bevy::ecs::entity::Entity;
pub use bevy::math::Vec2;

pub use bounds::*;
pub use location::*;

mod bounds;
pub mod iter;
mod location;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorKind {
    OutOfBounds(Bounds, Location),
}

impl ErrorKind {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            OutOfBounds(_, _) => "out of bounds",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Options {
    /// Target capacity of a leaf before it is split in nodes. Note that a leaf
    /// may contain more items when `max_depth` is reached.
    pub capacity: usize,

    pub max_depth: Option<u8>,
    pub min_size: Option<Vec2>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            capacity: 4,
            max_depth: None,
            min_size: None,
        }
    }
}

#[allow(dead_code)]
pub enum Region {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

impl Region {
    #[inline(always)]
    pub fn index(&self) -> usize {
        use Region::*;
        return match self {
            NorthWest => 0,
            NorthEast => 1,
            SouthEast => 2,
            SouthWest => 3,
        };
    }
}

impl Into<usize> for Region {
    #[inline(always)]
    fn into(self) -> usize { self.index() }
}

pub(crate) enum Body {
    Empty,
    Leaf(Vec<(Location, Entity)>),
    Node([QuadTree; 4]), // 4 regions
}

pub struct QuadTree {
    pub(crate) bounds: Bounds,
    pub(crate) body: Box<Body>,
    options: Options,
    depth: u8,
}

impl QuadTree {
    #[inline]
    pub fn new(bounds: Bounds, options: Options) -> Self {
        Self {
            bounds,
            options,
            body: Box::new(Body::Empty),
            depth: 0,
        }
    }

    #[inline]
    fn new_region(bounds: Bounds, options: Options, depth: u8) -> Self {
        Self {
            bounds,
            options,
            body: Box::new(Body::Empty),
            depth: depth + 1,
        }
    }

    /// Bounds, or area, in which the `QuadTree` operates.
    #[inline(always)]
    pub fn bounds(&self) -> Bounds { self.bounds }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn options(&self) -> Options { self.options }

    /// Indicates if the `QuadTree` contains any inserted elements.
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return match self.body.deref() {
            Body::Empty => true,
            _ => false
        };
    }

    /// Indicates if the `QuadTree` is a leaf (lowest possible body type).
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        return match self.body.deref() {
            Body::Leaf(_) => true,
            _ => false
        };
    }

    /// Indicates if `location` is inside, or intersects with the `QuadTree`'s
    /// `bounds`.
    #[inline]
    pub fn contains(&self, location: Location) -> bool {
        match location {
            Location::Point(point) => self.bounds.contains(point),
            Location::Area(bounds) => self.bounds.intersects(bounds),
        }
    }

    /// Insert `entity` at `location`.
    pub fn insert(&mut self, location: Location, value: Entity) -> Result<(), ErrorKind> {
        if !self.contains(location) {
            return Err(ErrorKind::OutOfBounds(self.bounds, location));
        }

        match self.body.deref_mut() {
            // quadtree is empty, make it a leaf
            Body::Empty => {
                let mut elems = Vec::with_capacity(self.options.capacity);
                elems.push((location, value));
                self.body = Box::new(Body::Leaf(elems));
            }

            // quadtree is a leaf, make it a node
            Body::Leaf(elems) => {
                elems.push((location, value));
                if elems.len() <= self.options.capacity
                    || self.depth >= self.options.max_depth.unwrap_or(255) {
                    // return when map is not over capacity or when max depth is reached
                    return Ok(());
                }
                if let Some(min_size) = self.options.min_size {
                    if self.bounds.width() <= (min_size.x * 2.0) || self.bounds.height() <= (min_size.y * 2.0) {
                        return Ok(());
                    }
                }

                let center = self.bounds.center();
                let mut regions = [
                    // Region::NorthWest
                    Self::new_region(
                        Bounds::from_corners(self.bounds.top_left(), center),
                        self.options,
                        self.depth + 1,
                    ),
                    // Region::NorthEast
                    Self::new_region(
                        Bounds::from_corners(center, self.bounds.top_right()),
                        self.options,
                        self.depth + 1,
                    ),
                    // Region::SouthEast
                    Self::new_region(
                        Bounds::from_corners(center, self.bounds.bottom_right()),
                        self.options,
                        self.depth + 1,
                    ),
                    // Region::SouthWest
                    Self::new_region(
                        Bounds::from_corners(self.bounds.bottom_left(), center),
                        self.options,
                        self.depth + 1,
                    ),
                ];

                for (loc, val) in elems.iter() {
                    let _ = regions[0].insert(*loc, *val);
                    let _ = regions[1].insert(*loc, *val);
                    let _ = regions[2].insert(*loc, *val);
                    let _ = regions[3].insert(*loc, *val);
                }

                self.body = Box::new(Body::Node(regions));
            }

            // quadtree is already a node, try to insert in any of its the regions
            Body::Node(regions) => {
                let _ = regions[0].insert(location, value);
                let _ = regions[1].insert(location, value);
                let _ = regions[2].insert(location, value);
                let _ = regions[3].insert(location, value);
            }
        };
        return Ok(());
    }

    /// Count and return the amount of inserted items among all leafs.
    #[allow(dead_code)]
    #[inline]
    pub fn count(&self) -> usize {
        return match self.body.deref() {
            Body::Empty => { 0 }
            Body::Leaf(map) => { map.len() }
            Body::Node(regions) => {
                let mut size = 0;
                for region in regions {
                    size += region.count();
                }
                size
            }
        };
    }

    #[inline]
    pub fn elements(&self) -> Option<Vec<(Location, Entity)>> {
        return match self.body.deref() {
            Body::Empty => { None }
            Body::Leaf(elems) => { Some(elems.clone()) }
            Body::Node(_) => {
                // todo get + merge elems from underlying regions
                None
            }
        };
    }

    #[allow(dead_code)]
    #[inline]
    pub fn region(&self, region: Region) -> Option<&QuadTree> {
        return match self.body.deref() {
            Body::Node(regions) => {
                Some(&regions[region.index()])
            }
            _ => None
        };
    }

    #[inline]
    pub fn regions(&self) -> Vec<&QuadTree> {
        let mut vec = Vec::new();
        get_regions(&mut vec, self);
        return vec;
    }

    // pub fn iter(&self) -> CombinationIterator {
    //     let mut vec = Vec::<Combination>::new();
    //     fill_combination_iterator(&mut vec, self);
    //     RegionsIterator { iter: vec.into_iter() }
    // }

    // pub fn for_each(&self) {
    //
    // }
    //
    // pub fn par_for_each(&self) {
    //
    // }
}

fn get_regions<'a>(dest: &mut Vec<&'a QuadTree>, tree: &'a QuadTree) {
    match tree.body.deref() {
        Body::Empty => {}
        Body::Leaf(_) => { dest.push(tree); }
        Body::Node(regions) => {
            get_regions(dest, regions[0].borrow());
            get_regions(dest, regions[1].borrow());
            get_regions(dest, regions[2].borrow());
            get_regions(dest, regions[3].borrow());
        }
    };
}