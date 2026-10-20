#[cfg(test)]
mod tests {
    use std::{assert_eq};

use crate::tree::{
        RbTree,
        enums::{Direction, RbColor},
    };

    fn get_key(tree: &RbTree, idx: usize) -> u32 {
        tree.nodes.get(idx).expect("invalid index").key
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
        let five_lr = tree.nodes.get(five_l).unwrap().right.unwrap();

        assert_eq!(five.key, 5);
        assert_eq!(five.right, None);
        assert_eq!(get_key(&tree, five_l), 3);
        assert_eq!(get_key(&tree, five_lr), 4);
    }

    #[test]
    pub fn classify() {
        let tree = create_tree(&[1, 3, 2]);
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

        // dbg!(&tree.nodes);
    }

    #[test]
    pub fn min() {
        let mut tree = RbTree::new();
        tree.insert_all(&[1, 4, 3, 5, 6]);

        let min = tree.min().map(|m| m.key);
        assert_eq!(min, Some(1));
    }

    #[test]
    pub fn max() {
        let mut tree = RbTree::new();
        tree.insert_all(&[1, 4, 3, 5, 6]);

        let min = tree.max().map(|m| m.key);
        assert_eq!(min, Some(6));
    }

    #[test]
    pub fn successor() {
        let mut tree = RbTree::new();
        tree.insert_all(&[1, 2, 3, 4, 5]);

        assert!(tree.successor(2).is_some_and(|n| n.key == 3));
        assert!(tree.successor(5).is_none());
    }

    #[test]
    pub fn predecessor() {
        let mut tree = RbTree::new();
        tree.insert_all(&[1, 2, 3, 4, 5]);

        assert!(tree.predecessor(2).is_some_and(|n| n.key == 1));
        assert!(tree.predecessor(1).is_none());
    }

    #[test]
    pub fn find() {
        let mut tree = RbTree::new();
        tree.insert_all(&[1, 2, 3, 4, 5]);

        assert!(tree.find(4).is_some());
        assert!(tree.find(9).is_none());
    }

    #[test]
    pub fn height() {
        let mut tree = RbTree::new();
        tree.insert(18);

        assert_eq!(1, tree.height());

        tree.insert(16);
        tree.insert(17);

        assert_eq!(2, tree.height());

        tree.insert(1);
        assert_eq!(3, tree.height());
    }
}
