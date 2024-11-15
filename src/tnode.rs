use crate::{Dimension, Node};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct TNode<I: Clone, const S: usize, D: Clone + Dimension<S>>
{
    space: D,
    count: u32,
    capacity: u8,
    depth_limit: u8,
    nodes: Option<Box<Vec<Self>>>,
    objects: Vec<(I, D)>,
}

impl<I: Clone, const S: usize, D: Clone + Dimension<S>> TNode<I, S, D>
{
    pub fn new(rect: D, capacity: u8, depth: u8) -> Self {
        Self {
            space: rect,
            count: 0,
            capacity,
            depth_limit: depth,
            nodes: None,
            objects: Vec::with_capacity(capacity as usize),
        }
    }

    fn push_to(&mut self, id: I, rect: D) -> bool {
        match &mut self.nodes {
            Some(nodes) => {
                let mut b = false;
                nodes.iter_mut().for_each(|n| b = b | n.insert(id.clone(), rect.clone()));
                b
            }
            None => false,
        }
    }
}

impl<I: Clone, const S: usize, D: Clone + Dimension<S>> Node<I, S, D> for TNode<I, S, D> {
    fn insert(&mut self, id: I, other: D) -> bool {
        if !self.space.overlaps(&other) {
            return false;
        }
        if self.depth_limit <= 0 {
            self.objects.push((id, other.clone()));
            if self.space.contains_center(&other) {
                self.count += 1;
                return true;
            } else {
                return false;
            }
        }
        if self.nodes.is_some() {
            return self.push_to(id, other);
        }
        let contains = self.space.contains_center(&other);
        if self.count >= self.capacity.into() && contains {
            self.subdivide();
            return self.push_to(id, other);
        }
        self.objects.push((id, other));
        if contains {
            self.count += 1;
            return true;
        } else {
            return false;
        }
    }
    fn subdivide(&mut self) {
        self.nodes = Some(Box::new(
            self.space
                .subdivisions()
                .iter()
                .map(|d| Self::new(d.clone(), self.capacity, self.depth_limit - 1))
                .collect::<Vec<Self>>()
                .try_into()
                .unwrap_or_else(|_| panic!("A node has created an incorrect number of subdivisions")),
        ));
        for i in 0..self.objects.len() {
            let o = self.objects[i].clone();
            self.push_to(o.0, o.1);
        }
        self.count = 0;
    }
    fn search(&self, area: &D, buffer: &mut Vec<(I, D)>) {
        if self.space.overlaps(area) {
            match &self.nodes {
                Some(x) => {
                    for i in 0..x.len() {
                        x[i].search(area, buffer);
                    }
                }
                None => buffer.append(&mut self.objects.clone()),
            }
        }
    }
    fn search_with<OF>(&self, overlaps: &OF, buffer: &mut Vec<(I, D)>)
    where
        OF: Fn(&D) -> bool,
    {
        if overlaps(&self.space) {
            match &self.nodes {
                Some(x) => {
                    for i in 0..x.len() {
                        x[i].search_with(overlaps, buffer);
                    }
                }
                None => buffer.append(&mut self.objects.clone()),
            }
        }
    }
    fn clear(&mut self) {
        match &mut self.nodes {
            Some(x) => {
                for i in x.iter_mut() {
                    i.clear();
                }
            }
            None => {
                self.objects.clear();
                self.count = 0;
            }
        }
    }
}
