use std::{mem::swap, panic, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RbColor {
    Red,
    Black
}

pub enum ChildType {
    Left,
    Right
}

#[derive(Debug)]
pub struct RbElem {
    key: u32,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    color: RbColor
}

#[derive(Debug)]
pub struct RbTree {
    root: Option<usize>,
    nodes: Vec<RbElem>
}

impl RbTree {
    pub fn new() -> RbTree {
        RbTree { root: None, nodes: vec![] }
    }

    fn get_mut(&mut self, idx:usize) -> &mut RbElem {
        self.nodes.get_mut(idx).expect("invalid index")
    }

    fn get_parent_idx(&self, idx: usize) -> Option<usize> {
        self.nodes.get(idx)
            .expect("Invalid index")
            .parent
    }

    fn get_child_idx(&self, idx: usize, child_type: ChildType) -> Option<usize> {
        self.nodes.get(idx).and_then(|e| {
            match child_type {
                ChildType::Left => e.left,
                ChildType::Right => e.right
            }
        })
    }

    fn get_color(&self, idx: usize) -> Option<RbColor> {
        self.nodes.get(idx).map(|e| e.color)
    }

    fn set_color(&mut self, idx: usize, color: RbColor) {
        self.nodes.get_mut(idx).and_then(|e| {
            e.color = color;
            Some(())
        });
    }

    fn set_child_index(&mut self, idx: usize, value: Option<usize>, child_type: ChildType) {
        self.nodes.get_mut(idx).and_then(|e| {
            match child_type {
                ChildType::Left => e.left = value,
                ChildType::Right => e.right = value
            }

            Some(())
        });
    }

    fn set_parent_index(&mut self, idx: usize, value: Option<usize>) {
        if value.is_some_and(|v| v >= self.nodes.len()) {
            panic!("invalid index");
        }
        
        self.get_mut(idx).parent = value;

        self.get_parent_idx(idx).and_then(|p_idx| {
            self.nodes.get_mut(p_idx).and_then(|e| {
                e.parent = value;

                Some(())
            })
        });
    }

    fn is_root(&self, idx: usize) -> bool {
        if let Some(r) = self.root {
            idx == r
        }
        else {
            false
        }
    }

    fn rotate_left(&mut self, idx: usize) {
        let r_idx = self.get_child_idx(idx, ChildType::Right)
            .expect("left rotation only works if there is a right child");

        if let Some(i) = self.get_child_idx(r_idx, ChildType::Left) {
            // Move left subtree of r_idx to idx
            self.set_parent_index(i, Some(idx));
            self.set_child_index(idx, Some(i), ChildType::Right);

            let g_idx = self.get_parent_idx(idx);
            self.set_parent_index(r_idx, g_idx);

            if let Some(j) = g_idx {
                // Update child values for grandparent
                if self.get_child_idx(j, ChildType::Left)
                    .is_some_and(|k| k == r_idx) {
                    self.set_child_index(j, Some(r_idx), ChildType::Left);
                }
                else {
                    self.set_child_index(j, Some(r_idx), ChildType::Right);
                }
            }
            else {
                self.root = Some(r_idx);
            }
        }
        else {
            self.set_child_index(idx, None, ChildType::Right);
        }

        self.set_child_index(r_idx, Some(idx), ChildType::Left);
        self.set_parent_index(idx, Some(r_idx));
    }

    fn rotate_right(&mut self, idx: usize) {

    }

    fn tree_insert(&mut self, key: u32) -> usize {
        if self.nodes.is_empty() {
            // Empty, just push and return
            self.nodes.push(
                RbElem { 
                    key, 
                    parent: None, 
                    left: None, 
                    right: None,
                    color: RbColor::Red
                });
            
            self.root = Some(0);
        }
        else {
            // Traverse tree, then push
            let mut x = self.root;
            let mut y: Option<usize> = None;
            let mut z = RbElem { 
                key, 
                parent: None, 
                left: None, 
                right: None,
                color: RbColor::Red
            };

            let z_idx = self.nodes.len();

            while let Some(n) = x {
                if let Some(node) = self.nodes.get(n) {
                    y = Some(n);
                    x = if key < node.key { node.left } else { node.right };
                }
                else {
                    panic!("invalid index in traversal");
                }
            }

            if let Some(p) = y {
                let parent = self.nodes.get_mut(p).unwrap();

                // Determine which child is 'z'
                if z.key < parent.key {
                    parent.left = Some(z_idx);
                }
                else {
                    parent.right = Some(z_idx);
                }

                z.parent = Some(p);
            }
            else {
                // New node is the root
                self.root = Some(self.nodes.len());
            }

            self.nodes.push(z);
                
        }
        
        self.nodes.len() - 1
    }

    fn rb_insert_fix(&mut self, idx: usize) {
        let mut idx = idx;
        let mut p_idx = self.get_parent_idx(idx).unwrap();
        
        while self.get_color(p_idx).is_some_and(|c| c == RbColor::Red) {
            // Can unwrap because of the red-black properties
            let g_idx = self.get_parent_idx(p_idx).unwrap();

            if self.get_child_idx(g_idx, ChildType::Left)
                .is_some_and(|i| i == p_idx) {
                // The parent is a left child
                
                // Get right uncle
                let u_idx = self.get_child_idx(g_idx, ChildType::Right);
                let u_color = u_idx.and_then(|i| self.get_color(i));
                
                if u_color.is_some_and(|c| c == RbColor::Red) {
                    let g_root = self.is_root(g_idx);

                    // Uncle is red, invert colors
                    self.set_color(p_idx, RbColor::Black);

                    // Only invert the grandparent if its not the root.
                    if !g_root { self.set_color(g_idx, RbColor::Red); }

                    // Only invert uncle if it exists (since a null leaf is always black)
                    if let Some(i) = u_idx {
                        self.set_color(i, RbColor::Black);
                    }

                    // Move up the tree
                    idx = g_idx;
                    
                    // Since the root is always black, we can set p_idx to point to
                    // g_idx in case g_idx is the root, which will make the while loop terminate.
                    p_idx = self.get_parent_idx(g_idx).unwrap_or(g_idx);
                }
                else {
                    if self.get_child_idx(p_idx, ChildType::Right)
                        .is_some_and(|i| i == idx) {
                        // If the node is a right child, rotate left
                        swap(&mut idx, &mut p_idx);
                        self.rotate_left(idx);
                    }
                    
                    self.set_color(p_idx, RbColor::Black);
                    self.set_color(g_idx, RbColor::Red);
                    self.rotate_right(g_idx);
                }
            }
            else {
                // The parent is a right child
            }
        }
    }

    pub fn insert(&mut self, key: u32) {
        let idx = self.tree_insert(key);
        self.rb_insert_fix(idx);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_key(tree: &RbTree, idx: usize) -> u32 {
        tree.nodes.get(idx)
            .expect("invalid index")
            .key
    }

    fn create_tree(values: &[u32]) -> RbTree {
        let mut tree = RbTree::new();

        for value in values {
            tree.tree_insert(*value);
        }

        tree
    }

    #[test]
    pub fn tree_insert_test() {
        let tree = create_tree(&[5, 3, 4]);

        let five = tree.nodes.get(0).unwrap();
        let five_l = five.left.unwrap();
        let five_lr=tree.nodes.get(five_l).unwrap().right.unwrap();

        assert_eq!(five.key, 5);
        assert_eq!(five.right, None);
        assert_eq!(get_key(&tree, five_l), 3);
        assert_eq!(get_key(&tree, five_lr), 4);
    }

    #[test]
    pub fn get_parent() {
        let tree = create_tree(&[1, 2]);

        let one = tree.nodes.get(0).unwrap();
        let two = tree.nodes.get(one.right.unwrap()).unwrap();

        assert_eq!(two.parent, tree.get_parent_idx(one.right.unwrap()));
    }

    #[test]
    pub fn set_parent() {
        let mut tree = create_tree(&[2, 1, 3]);

        let idx_3 = tree.get_child_idx(0, ChildType::Right).unwrap();
        let idx_2 = tree.get_child_idx(0, ChildType::Left).unwrap();
        let el_3 = tree.nodes.get(idx_3).unwrap();

        // Change parent of 3 from 1 to 2
        tree.set_parent_index(idx_3, Some(idx_2));
    }

    #[test]
    pub fn get_child() {
        let tree = create_tree(&[1, 2]);

        let two = tree.get_child_idx(0, ChildType::Right).unwrap();

        assert_eq!(two, 1);
        assert_eq!(get_key(&tree, two), 2);
    }

    #[test]
    pub fn get_color() {
        let mut tree = create_tree(&[1, 2]);
        let two = tree.get_mut(1);

        assert_eq!(two.color, RbColor::Red);

        two.color = RbColor::Black;

        assert_eq!(two.color, RbColor::Black);
    }
}