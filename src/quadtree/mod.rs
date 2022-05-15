use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy::utils::HashMap;

pub use bounds::*;
pub use iter::*;
pub use location::*;
pub use point::*;

mod bounds;
mod iter;
mod location;
mod point;

#[derive(Clone, Copy, Debug)]
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

pub struct QuadTree {
    bounds: Bounds,
    capacity: usize,
    max_depth: Option<u8>,
    body: Box<Body>,
}

enum Body {
    Empty,
    Leaf(HashMap<Entity, Location>),
    // [(Location, Entity); CAP]
    Node([QuadTree; 4]),
}

#[allow(dead_code)]
impl QuadTree {
    #[inline]
    pub fn new(bounds: Bounds, capacity: usize, max_depth: Option<u8>) -> Self {
        Self {
            bounds,
            capacity,
            max_depth,
            body: Box::new(Body::Empty),
        }
    }

    pub fn capacity(&self) -> usize { self.capacity }

    pub fn contains(&self, location: Location) -> bool {
        match location {
            Location::Point(xy) => self.bounds.contains(xy),
            Location::Area(bounds) => self.bounds.intersects(bounds),
        }
    }

    pub fn insert(&mut self, location: Location, data: Entity) -> Result<usize, ErrorKind> {
        if !self.contains(location) {
            return Err(ErrorKind::OutOfBounds(self.bounds, location));
        }
        match self.body.deref_mut() {
            // quadtree is empty, make it a leaf
            Body::Empty => {
                let mut map = HashMap::with_capacity(self.capacity);
                map.insert(data, location);
                self.body = Box::new(Body::Leaf(map));
                return Ok(1);
            }

            // quadtree is a leaf, make it a node
            Body::Leaf(map) => {
                // insert when capacity is not reached, or when Some(max_depth) == 0
                if map.len() < self.capacity || self.max_depth.unwrap_or(1) == 0 {
                    map.insert(data, location);
                    return Ok(map.len());
                }

                let max_depth: Option<u8> = if let Some(n) = self.max_depth { Some(n - 1) } else { None };

                let center = self.bounds.center();
                let mut regions = [
                    // north east
                    QuadTree::new(
                        Bounds::from_corners(self.bounds.top_left(), center),
                        self.capacity,
                        max_depth,
                    ),
                    // north west
                    QuadTree::new(
                        Bounds::from_corners(
                            Point::new(center.x, self.bounds.top()),
                            Point::new(self.bounds.right(), center.y),
                        ),
                        self.capacity,
                        max_depth,
                    ),
                    // south east
                    QuadTree::new(
                        Bounds::from_corners(
                            Point::new(self.bounds.left(), center.y),
                            Point::new(center.x, self.bounds.bottom()),
                        ),
                        self.capacity,
                        max_depth,
                    ),
                    // south west
                    QuadTree::new(
                        Bounds::from_corners(center, self.bounds.bottom_right()),
                        self.capacity,
                        max_depth,
                    ),
                ];

                for region in &mut regions {
                    for data in map.iter() {
                        let _ = region.insert(*data.1, data.0.clone());
                    }
                }

                self.body = Box::new(Body::Node(regions));
                return Ok(0);
            }

            // quadtree is a node, try to insert in any of the partitions
            Body::Node(nodes) => {
                let _ = nodes[0].insert(location, data);
                let _ = nodes[1].insert(location, data);
                let _ = nodes[2].insert(location, data);
                let _ = nodes[3].insert(location, data);
                return Ok(0);
            }
        }
    }

    pub fn query(&self, location: Location) -> Vec<(Location, Entity)> {
        Vec::new()
    }

    pub fn query_entities(&self, location: Location) -> Vec<Entity> {
        let mut res = Vec::<Entity>::new();
        for (_, e) in self.query(location).drain(..) {
            res.push(e)
        }
        return res;
    }
}

impl IntoIterator for QuadTree {
    type Item = (Bounds, HashMap<Entity, Location>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = Vec::<Self::Item>::new();
        fill_iter(&mut iter, self.bounds, self.body.deref());

        iter.into_iter()
    }
}

fn fill_iter(iter: &mut Vec<(Bounds, HashMap<Entity, Location>)>, bounds: Bounds, body: &Body) {
    match body {
        // quadtree is empty
        Body::Empty => {}

        // quadtree is a leaf
        Body::Leaf(map) => {
            iter.push((bounds, map.clone()));
        }

        // quadtree is a node
        Body::Node(nodes) => {
            fill_iter(iter, nodes[0].bounds, nodes[0].body.deref());
            fill_iter(iter, nodes[1].bounds, nodes[1].body.deref());
            fill_iter(iter, nodes[2].bounds, nodes[2].body.deref());
            fill_iter(iter, nodes[3].bounds, nodes[3].body.deref());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn qaudtree_subdivide() {
        let mut qt = QuadTree::new(
            Bounds::from_corners(Point::new(0.0, 0.0), Point::new(100.0, 100.0)),
            4,
            None,
        );
    }
}
