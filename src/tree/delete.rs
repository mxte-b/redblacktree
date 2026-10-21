use crate::tree::{
    RbElem, RbTree,
    enums::{Direction, RbColor},
};

impl RbTree {
    fn update_references(&mut self, before_idx: usize, after_idx: usize) {
        // Update parent reference if needed
        if let Some(parent_idx) = self.get_parent_idx(before_idx) {
            let direction = self.classify(before_idx).unwrap();
            self.set_child_index(parent_idx, Some(after_idx), direction);
        }

        // Update child references if needed
        if let Some(left_idx) = self.get_child_idx(before_idx, Direction::Left) {
            self.set_parent_index(left_idx, Some(after_idx));
        }

        if let Some(right_idx) = self.get_child_idx(before_idx, Direction::Right) {
            self.set_parent_index(right_idx, Some(after_idx));
        }

        // Update root if needed
        if self.root == Some(before_idx) {
            self.root = Some(after_idx);
        }
    }

    /// Deletes a node from the tree and returns it.
    pub(crate) fn delete_idx(&mut self, idx: usize) -> RbElem {
        let mut to_delete_idx = idx;

        // If the node to delete has two children, we delete
        // its successor instead, which is well-defined, so
        // we can safely unwrap
        if self.get_child_idx(idx, Direction::Left).is_some()
            && self.get_child_idx(idx, Direction::Right).is_some()
        {
            to_delete_idx = self.inorder_neighbor_idx(idx, Direction::Right).unwrap();
        }

        let to_delete_parent = self.get_parent_idx(to_delete_idx);

        // Get the replacement node
        let replacement_idx = self
            .get_child_idx(to_delete_idx, Direction::Left)
            .or(self.get_child_idx(to_delete_idx, Direction::Right));

        // Set the parent of the replacement node
        // to the parent of the node to delete
        let replacement_parent_idx = to_delete_parent;
        if let Some(i) = replacement_idx {
            // If the replacement is not NIL, update the node
            self.set_parent_index(i, to_delete_parent);
        }

        // If the node we delete is the root, the
        // replacement will become the root
        if to_delete_parent.is_none() {
            self.root = replacement_idx;
        } else {
            // Else, we set the correct child index of the parent
            // to the replacement
            let to_delete_dir = self.classify(to_delete_idx).unwrap();
            self.set_child_index(to_delete_parent.unwrap(), replacement_idx, to_delete_dir);
        }

        // If the node to delete changed,
        // we have to copy its data into idx's place
        let to_delete = self.get(to_delete_idx);
        let original_key = self.get(idx).key;
        let new_key = to_delete.key;
        let deleted_color = to_delete.color;

        if to_delete_idx != idx {
            self.get_mut(to_delete_idx).key = original_key;
            self.get_mut(idx).key = new_key;
        }

        // If the deleted node was black, we have
        // to fix the red-black properties
        if deleted_color == RbColor::Black {
            self.rb_delete_fix(replacement_idx, replacement_parent_idx);
        }

        // Because swap_remove replaces the deleted node
        // with the last element in the vector, we have to
        // update all references to the last element
        let last_idx = self.nodes.len() - 1;
        if to_delete_idx != last_idx {
            self.update_references(last_idx, to_delete_idx);
        }

        self.nodes.swap_remove(to_delete_idx)
    }

    fn rb_delete_fix(
        &mut self,
        replacement_idx: Option<usize>,
        replacement_parent_idx: Option<usize>,
    ) {
        let mut current_idx = replacement_idx;
        let mut current_parent_idx = replacement_parent_idx;

        while let Some(parent_idx) = current_parent_idx
            && self.get_color(current_idx) == RbColor::Black
        {
            // Can unwrap because it cannot be the root
            let dir = if self.get_child_idx(parent_idx, Direction::Left) == current_idx {
                Direction::Left
            } else {
                Direction::Right
            };
            let other_dir = dir.invert();

            let mut sibling_idx = self
                .get_child_idx(parent_idx, other_dir)
                .expect("sibling of a doubly-black node must exist");

            if self.get_color(Some(sibling_idx)) == RbColor::Red {
                self.set_color(sibling_idx, RbColor::Black);
                self.set_color(parent_idx, RbColor::Red);
                self.rotate(parent_idx, dir);

                sibling_idx = self.get_child_idx(parent_idx, other_dir).unwrap();
            }

            let near_idx = self.get_child_idx(sibling_idx, dir);
            let mut far_idx = self.get_child_idx(sibling_idx, other_dir);

            // If both children of the sibling are black,
            // we can remove a black from each subtree of the parent,
            // and continue fixing from the parent
            if self.get_color(near_idx) == RbColor::Black
                && self.get_color(far_idx) == RbColor::Black
            {
                self.set_color(sibling_idx, RbColor::Red);

                current_idx = Some(parent_idx);
                current_parent_idx = self.get_parent_idx(parent_idx);
            }
            else {
                // If the near child of w is red, we can transform
                // it into case 4
                if self.get_color(far_idx) == RbColor::Black {
                    self.set_color(near_idx.unwrap(), RbColor::Black);
                    self.set_color(sibling_idx, RbColor::Red);
                    self.rotate(sibling_idx, other_dir);

                    sibling_idx = self.get_child_idx(parent_idx, other_dir).unwrap();
                    far_idx = self.get_child_idx(sibling_idx, other_dir);
                }

                // If the far child of w is red, we can fix the tree
                // via recoloring and rotating.
                self.set_color(sibling_idx, self.get_color(Some(parent_idx)));
                self.set_color(parent_idx, RbColor::Black);
                self.set_color(far_idx.unwrap(), RbColor::Black);
                self.rotate(parent_idx, dir);
 
                current_idx = self.root;
                current_parent_idx = None;
            }

        }

        if let Some(i) = current_idx {
            self.set_color(i, RbColor::Black);
        }
    }

    pub fn delete(&mut self, key: u32) {
        if let Some(i) = self.find_idx(key) {
            self.delete_idx(i);
        }
    }
}
