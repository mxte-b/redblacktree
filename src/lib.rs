use std::{fmt::Display, mem::swap, panic, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RbColor {
    Red,
    Black
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left, 
    Right
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
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

    fn get_child_idx(&self, idx: usize, direction: Direction) -> Option<usize> {
        self.nodes.get(idx).and_then(|e| {
            match direction {
                Direction::Left => e.left,
                Direction::Right => e.right
            }
        })
    }

    fn get_color(&self, idx: usize) -> Option<RbColor> {
        self.nodes.get(idx).map(|e| e.color)
    }

    fn classify(&self, idx: usize) -> Option<Direction> {
        self.nodes.get(idx).and_then(|e| {
            if let Some(parent) = e.parent.and_then(|i| self.nodes.get(i)){
                let child_type = if parent.left == Some(idx) { Direction::Left } else { Direction::Right };
                return Some(child_type);
            }
            else {
                return None;
            }
        })
    }

    fn set_color(&mut self, idx: usize, color: RbColor) {
        self.nodes.get_mut(idx).and_then(|e| {
            e.color = color;
            Some(())
        });
    }

    fn set_child_index(&mut self, idx: usize, value: Option<usize>, direction: Direction) {
        self.nodes.get_mut(idx).and_then(|e| {
            match direction {
                Direction::Left => e.left = value,
                Direction::Right => e.right = value
            }

            Some(())
        });
    }

    fn set_parent_index(&mut self, idx: usize, value: Option<usize>) {
        if value.is_some_and(|v| v >= self.nodes.len()) {
            panic!("invalid index");
        }
        
        self.get_mut(idx).parent = value;
    }

    fn is_root(&self, idx: usize) -> bool {
        if let Some(r) = self.root {
            idx == r
        }
        else {
            false
        }
    }

    fn rotate(&mut self, idx: usize, direction: Direction) {
        let other_direction = direction.invert();

        let r_idx = self.get_child_idx(idx, other_direction)
            .expect("left rotation only works if there is a right child");

        if let Some(i) = self.get_child_idx(r_idx, direction) {
            // Move left subtree of r_idx to idx
            self.set_parent_index(i, Some(idx));
            self.set_child_index(idx, Some(i), other_direction);
        }
        else {
            self.set_child_index(idx, None, other_direction);
        }

        let g_idx = self.get_parent_idx(idx);
        self.set_parent_index(r_idx, g_idx);

        if let Some(j) = g_idx {
            // Update child values for grandparent
            match self.classify(idx) {
                Some(t) => self.set_child_index(j, Some(r_idx), t),
                None => { panic!("Right subtree was expected to have a parent") }
            }
        }
        else {
            self.root = Some(r_idx);
        }

        self.set_child_index(r_idx, Some(idx), direction);
        self.set_parent_index(idx, Some(r_idx));
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
        let mut p_idx: usize;

        if let Some(i) = self.get_parent_idx(idx) {
            p_idx = i;
        }
        else {
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
                if !self.is_root(g_idx) { self.set_color(g_idx, RbColor::Red); }

                // Move up the tree
                idx = g_idx;
                
                // Since the root is always black, we can set p_idx to point to
                // g_idx in case g_idx is the root, which will make the while loop terminate.
                p_idx = self.get_parent_idx(g_idx).unwrap_or(g_idx);
            }
            else {
                if self.classify(idx) == Some(p_dir_inv) {
                    // If the node is a right child, rotate left
                    swap(&mut idx, &mut p_idx);
                    dbg!(&self.nodes);
                    println!("Rotating on {idx} with dir {p_dir:?}");
                    self.rotate(idx, p_dir);
                }

                dbg!(&self.nodes);
                
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

}

impl Display for RbTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
    pub fn classify() {
        let tree = create_tree(&[1,3,2]);
        let idx_3 = tree.get_child_idx(0, Direction::Right).unwrap();
        let idx_2 = tree.get_child_idx(idx_3, Direction::Left).unwrap();

        // 1 is root
        assert_eq!(tree.classify(0), None);

        // 3 is a right child
        assert_eq!(tree.classify(idx_3), Some(Direction::Right));

        // 3 is a left child
        assert_eq!(tree.classify(idx_2), Some(Direction::Left));
    }

    #[test]
    pub fn get_parent() {
        let tree = create_tree(&[1, 2]);

        let one = tree.nodes.get(0).unwrap();
        let two = tree.nodes.get(one.right.unwrap()).unwrap();

        assert_eq!(two.parent, tree.get_parent_idx(one.right.unwrap()));
    }

    #[test]
    pub fn set_parent_and_child() {
        let mut tree = create_tree(&[2, 1, 3]);

        let idx_3 = tree.get_child_idx(0, Direction::Right).unwrap();
        let idx_2 = tree.get_child_idx(0, Direction::Left).unwrap();

        // Change parent of 3 from 1 to 2
        tree.set_parent_index(idx_3, Some(idx_2));
        tree.set_child_index(idx_2, Some(idx_3), Direction::Right);
        tree.set_child_index(0, None, Direction::Right);

        // Parent value is good
        assert_eq!(tree.get_parent_idx(idx_3), Some(idx_2));

        // Is right child of correct node
        assert_eq!(tree.classify(idx_3), Some(Direction::Right));

        // Is not connected to original parent
        assert_eq!(tree.get_child_idx(0, Direction::Right), None);
    }

    #[test]
    pub fn get_child() {
        let tree = create_tree(&[1, 2]);

        let two = tree.get_child_idx(0, Direction::Right).unwrap();

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

    #[test]
    pub fn rotate_left() {
        let mut tree = create_tree(&[2, 1, 4, 3]);

        let idx_2 = 0;
        let idx_1 = tree.get_child_idx(idx_2, Direction::Left).unwrap();
        let idx_4 = tree.get_child_idx(idx_2, Direction::Right).unwrap();
        let idx_3 = tree.get_child_idx(idx_4, Direction::Left).unwrap();

        tree.rotate(0, Direction::Left);

        // 2 is left child of 4
        assert_eq!(tree.classify(idx_2), Some(Direction::Left));

        // 3 is right child of 2
        assert_eq!(tree.classify(idx_3), Some(Direction::Right));

        // 4 is root
        assert_eq!(tree.root, Some(idx_4));
    }

    #[test]
    pub fn rotate_right() {
        let mut tree = create_tree(&[2, 0, 1, 4]);

        let idx_2 = 0;
        let idx_0 = tree.get_child_idx(idx_2, Direction::Left).unwrap();
        let idx_4 = tree.get_child_idx(idx_2, Direction::Right).unwrap();
        let idx_1 = tree.get_child_idx(idx_0, Direction::Right).unwrap();

        tree.rotate(idx_2, Direction::Right);

        // 2 is right child of 0
        assert_eq!(tree.classify(idx_2), Some(Direction::Right));

        // 1 is left child of 2
        assert_eq!(tree.classify(idx_1), Some(Direction::Left));

        // 0 is root
        assert_eq!(tree.root, Some(idx_0));
    }

    #[test]
    pub fn insert_with_rotations() {
        let mut tree = RbTree::new();

        tree.insert(6);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(5);
        tree.insert(8);

        dbg!(&tree.nodes);
    }
}