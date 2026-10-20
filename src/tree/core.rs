use core::panic;
use std::{cmp::Ordering, collections::VecDeque, vec};

use crate::tree::{
    RbElem, RbTree,
    enums::{Direction, RbColor},
};

impl RbTree {
    pub fn new() -> RbTree {
        RbTree {
            root: None,
            nodes: vec![],
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut RbElem {
        self.nodes.get_mut(idx).expect("invalid index")
    }

    pub fn get(&self, idx: usize) -> &RbElem {
        self.nodes.get(idx).expect("invalid index")
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn height(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(self.root.unwrap());

        let mut height = 0;

        while !queue.is_empty() {
            height += 1;

            for _ in 0..queue.len() {
                let node = self.get(queue.pop_front().unwrap());

                if let Some(l) = node.left {
                    queue.push_back(l);
                }
    
                if let Some(r) = node.right {
                    queue.push_back(r);
                }
            }
        }

        height
    }

    pub(crate) fn get_parent_idx(&self, idx: usize) -> Option<usize> {
        self.nodes.get(idx).expect("Invalid index").parent
    }

    pub(crate) fn get_child_idx(&self, idx: usize, direction: Direction) -> Option<usize> {
        self.nodes.get(idx).and_then(|e| match direction {
            Direction::Left => e.left,
            Direction::Right => e.right,
        })
    }

    pub(crate) fn get_color(&self, idx: usize) -> Option<RbColor> {
        self.nodes.get(idx).map(|e| e.color)
    }

    pub(crate) fn classify(&self, idx: usize) -> Option<Direction> {
        let node = self.nodes.get(idx)?;
        let parent = self.get(node.parent?);

        if parent.left == Some(idx) {
            return Some(Direction::Left);
        }
        else {
            return Some(Direction::Right);
        }
    }

    pub(crate) fn set_color(&mut self, idx: usize, color: RbColor) {
        self.get_mut(idx).color = color;
    }

    pub(crate) fn set_child_index(
        &mut self,
        idx: usize,
        value: Option<usize>,
        direction: Direction,
    ) {
        if value.is_some_and(|v| v >= self.nodes.len()) {
            panic!("invalid index");
        }

        let node= self.get_mut(idx);
        match direction {
            Direction::Left => node.left = value,
            Direction::Right => node.right = value,
        }
    }

    pub(crate) fn set_parent_index(&mut self, idx: usize, value: Option<usize>) {
        if value.is_some_and(|v| v >= self.nodes.len()) {
            panic!("invalid index");
        }

        self.get_mut(idx).parent = value;
    }

    pub(crate) fn is_root(&self, idx: usize) -> bool {
        self.root
            .map(|i| idx == i)
            .unwrap_or(false)
    }

    pub(crate) fn min_idx(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut min_idx = self.root;
        let mut next_idx = self.root;

        while let Some(i) = next_idx {
            min_idx = next_idx;
            next_idx = self.get_child_idx(i, Direction::Left);
        }

        min_idx
    }

    pub(crate) fn max_idx(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut max_idx = self.root;
        let mut next_idx = self.root;

        while let Some(i) = next_idx {
            max_idx = next_idx;
            next_idx = self.get_child_idx(i, Direction::Right);
        }

        max_idx
    }

    pub(crate) fn inorder_neighbor_idx(&self, idx: usize, direction: Direction) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut successor_idx = None;
        let other_direction = direction.invert();

        if let Some(mut next_idx) = self.get_child_idx(idx, direction) {
            // If the node has a right child, find the leftmost node in its subtree.
            while let Some(left_idx) = self.get_child_idx(next_idx, other_direction) {
                next_idx = left_idx;
            }

            successor_idx = Some(next_idx);
        }
        else {
            // If the node doesn't have a right child, move up until 
            // we are coming from a left child.
            let mut current_idx = idx;
            
            while let Some(parent_idx) = self.get_parent_idx(current_idx)
            {
                if self.classify(current_idx) == Some(other_direction) {
                    successor_idx = Some(parent_idx);
                    break;
                }

                current_idx = parent_idx;
            }
        }

        successor_idx
    }

    pub fn min(&self) -> Option<&RbElem> {
        let idx = self.min_idx()?;
        Some(self.get(idx))
    }

    pub fn max(&self) -> Option<&RbElem> {
        let idx = self.max_idx()?;
        Some(self.get(idx))
    }

    pub fn predecessor(&self, key: u32) -> Option<&RbElem> {
        let node_idx = self.find_idx(key)?;
        let idx = self.inorder_neighbor_idx(node_idx, Direction::Left)?;
        Some(self.get(idx))
    }

    pub fn successor(&self, key: u32) -> Option<&RbElem> {
        let node_idx = self.find_idx(key)?;
        let idx = self.inorder_neighbor_idx(node_idx, Direction::Right)?;
        Some(self.get(idx))
    }

    pub fn find(&self, key: u32) -> Option<&RbElem> {
        let idx = self.find_idx(key)?;
        Some(self.get(idx))
    }

    pub(crate) fn find_idx(&self, key: u32) -> Option<usize> {
        let mut current_idx = self.root?;

        loop {
            let node = self.get(current_idx);

            current_idx = match key.cmp(&node.key) {
                Ordering::Equal => return Some(current_idx),
                Ordering::Less => self.get_child_idx(current_idx, Direction::Left)?,
                Ordering::Greater => self.get_child_idx(current_idx, Direction::Right)?,
            };
        }
    }

    pub(crate) fn rotate(&mut self, idx: usize, direction: Direction) {
        let other_direction = direction.invert();

        let r_idx = self
            .get_child_idx(idx, other_direction)
            .expect("left rotation only works if there is a right child");

        if let Some(i) = self.get_child_idx(r_idx, direction) {
            // Move left subtree of r_idx to idx
            self.set_parent_index(i, Some(idx));
            self.set_child_index(idx, Some(i), other_direction);
        } else {
            self.set_child_index(idx, None, other_direction);
        }

        let g_idx = self.get_parent_idx(idx);
        self.set_parent_index(r_idx, g_idx);

        if let Some(j) = g_idx {
            // Update child values for grandparent
            match self.classify(idx) {
                Some(t) => self.set_child_index(j, Some(r_idx), t),
                None => {
                    panic!("Right subtree was expected to have a parent")
                }
            }
        } else {
            self.root = Some(r_idx);
        }

        self.set_child_index(r_idx, Some(idx), direction);
        self.set_parent_index(idx, Some(r_idx));
    }
}
