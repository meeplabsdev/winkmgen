use elsa::{FrozenVec, vec::Iter};
use tree_sitter::Node;

use crate::get_from_range;

#[allow(non_camel_case_types)]
pub type pEntity<'a> = &'a Entity<'a>;

#[allow(non_camel_case_types)]
pub type vEntity<'a> = EntityList<'a>;

pub struct EntityArena<'a> {
    entities: FrozenVec<Box<Entity<'a>>>,
}

impl<'a> EntityArena<'a> {
    pub fn new() -> EntityArena<'a> {
        EntityArena {
            entities: FrozenVec::new(),
        }
    }

    pub fn push(&'a self, entity: Entity<'a>) -> pEntity<'a> {
        let index = self.entities.len();
        self.entities.push(Box::new(entity));
        let current = &self.entities[index];
        for child in &current.children.0 {
            child.parent.push(current);
        }
        current
    }
}

pub struct Entity<'a> {
    node: Node<'a>,
    parent: vEntity<'a>,
    children: vEntity<'a>,
    content: String,
    kind: u16,
}

impl<'a> Entity<'a> {
    pub fn new(node: Node<'a>, children: Vec<pEntity<'a>>) -> Self {
        Self {
            node,
            parent: EntityList::new(),
            children: EntityList::from(children),
            content: get_from_range(node.byte_range()),
            kind: node.kind_id(),
        }
    }

    pub fn parent(&'a self) -> Option<pEntity<'a>> {
        self.parent.get(0)
    }

    pub fn ancestor(&'a self, depth: usize) -> Option<pEntity<'a>> {
        let mut res = self;
        for _ in 0..depth {
            res = res.parent()?
        }
        Some(res)
    }

    pub fn depth_first_ascend(&'a self, kind: &[u16], max_depth: u16) -> Option<pEntity<'a>> {
        fn search<'a>(
            node: pEntity<'a>,
            kind: &[u16],
            max_depth: u16,
            depth: u16,
        ) -> Option<pEntity<'a>> {
            if max_depth > 0 && depth > max_depth {
                return None;
            }

            if kind.contains(&node.kind) {
                return Some(node);
            }

            match search(node.parent()?, kind, max_depth, depth + 1) {
                Some(c) => return Some(c),
                None => {}
            }

            None
        }

        search(self, kind, max_depth, 0)
    }

    pub fn child(&'a self, index: usize) -> Option<pEntity<'a>> {
        self.children.get(index)
    }

    pub fn children(&'a self) -> vEntity<'a> {
        self.children.clone()
    }

    pub fn depth_first_descend(&'a self, kind: &[u16], max_depth: u16) -> Option<pEntity<'a>> {
        fn search<'a>(
            node: pEntity<'a>,
            kind: &[u16],
            max_depth: u16,
            depth: u16,
        ) -> Option<pEntity<'a>> {
            if max_depth > 0 && depth > max_depth {
                return None;
            }

            if kind.contains(&node.kind) {
                return Some(node);
            }

            for child in node.children.iter() {
                match search(child, kind, max_depth, depth + 1) {
                    Some(c) => return Some(c),
                    None => {}
                }
            }

            None
        }

        search(self, kind, max_depth, 0)
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn kind(&self) -> u16 {
        self.kind
    }
}

#[derive(Clone)]
pub struct EntityList<'a>(FrozenVec<pEntity<'a>>);

impl<'a> EntityList<'a> {
    pub fn new() -> Self {
        Self(FrozenVec::new())
    }

    pub fn from(items: Vec<pEntity<'a>>) -> Self {
        Self(FrozenVec::from(items))
    }

    pub fn push(&'a self, item: pEntity<'a>) {
        self.0.push(item);
    }

    pub fn get(&'a self, index: usize) -> Option<pEntity<'a>> {
        self.0.get(index)
    }

    pub fn iter(&'a self) -> Iter<'a, pEntity<'a>> {
        self.0.iter()
    }

    pub fn len(&'a self) -> usize {
        self.0.len()
    }

    pub fn fits(&'a self, size: usize) -> bool {
        self.len() > size
    }

    pub fn to_vec(&'a self) -> Vec<pEntity<'a>> {
        self.0.clone().into_vec()
    }

    pub fn half_open(&'a self, start: usize, end: usize) -> Vec<pEntity<'a>> {
        if !self.fits(end) {
            return Vec::new();
        }

        self.to_vec()[start..end].to_vec()
    }

    pub fn half_open_offset(&'a self, front: usize, back: usize) -> Vec<pEntity<'a>> {
        self.half_open(front, self.len() - back)
    }
}
