use core::panic;
use std::mem::swap;

use crate::tree::{RbElem, RbTree, enums::RbColor};

impl RbTree {
    pub(crate) fn tree_insert(&mut self, key: u32) -> usize {
        if self.nodes.is_empty() {
            // Empty, just push and return
            self.nodes.push(RbElem {
                key,
                parent: None,
                left: None,
                right: None,
                color: RbColor::Red,
            });

            self.root = Some(0);
        } else {
            // Traverse tree, then push
            let mut x = self.root;
            let mut y: Option<usize> = None;
            let mut z = RbElem {
                key,
                parent: None,
                left: None,
                right: None,
                color: RbColor::Red,
            };

            let z_idx = self.nodes.len();

            while let Some(n) = x {
                if let Some(node) = self.nodes.get(n) {
                    y = Some(n);
                    x = if key < node.key {
                        node.left
                    } else {
                        node.right
                    };
                } else {
                    panic!("invalid index in traversal");
                }
            }

            if let Some(p) = y {
                let parent = self.nodes.get_mut(p).unwrap();

                // Determine which child is 'z'
                if z.key < parent.key {
                    parent.left = Some(z_idx);
                } else {
                    parent.right = Some(z_idx);
                }

                z.parent = Some(p);
            } else {
                // New node is the root
                self.root = Some(self.nodes.len());
            }

            self.nodes.push(z);
        }

        self.nodes.len() - 1
    }

    fn rb_insert_fix(&mut self, idx: usize) {
        let mut idx = idx;
        let mut p_idx: usize;

        if let Some(i) = self.get_parent_idx(idx) {
            p_idx = i;
        } else {
            self.set_color(idx, RbColor::Black);
            return;
        }

        while self.get_color(p_idx).is_some_and(|c| c == RbColor::Red) {
            // Can unwrap because of the red-black properties
            let g_idx = self.get_parent_idx(p_idx).unwrap();
            let p_dir = self.classify(p_idx).unwrap();
            let p_dir_inv = p_dir.invert();

            // The parent is a left child

            // Get right uncle
            let u_idx = self.get_child_idx(g_idx, p_dir_inv);
            let u_color = u_idx
                .and_then(|i| self.get_color(i))
                .unwrap_or(RbColor::Black);

            if u_color == RbColor::Red {
                // Uncle is red, invert colors
                self.set_color(p_idx, RbColor::Black);
                self.set_color(u_idx.unwrap(), RbColor::Black);

                // Only invert the grandparent if its not the root.
                if !self.is_root(g_idx) {
                    self.set_color(g_idx, RbColor::Red);
                }

                // Move up the tree
                idx = g_idx;

                // Since the root is always black, we can set p_idx to point to
                // g_idx in case g_idx is the root, which will make the while loop terminate.
                p_idx = self.get_parent_idx(g_idx).unwrap_or(g_idx);
            } else {
                if self.classify(idx) == Some(p_dir_inv) {
                    // If the node is a right child, rotate left
                    swap(&mut idx, &mut p_idx);
                    self.rotate(idx, p_dir);
                }

                self.set_color(p_idx, RbColor::Black);
                self.set_color(g_idx, RbColor::Red);
                self.rotate(g_idx, p_dir_inv);
            }
        }
    }

    pub fn insert(&mut self, key: u32) {
        let idx = self.tree_insert(key);
        self.rb_insert_fix(idx);
    }

    pub fn insert_all(&mut self, values: &[u32]) {
        for k in values.iter() {
            self.insert(*k);
        }
    }
}
