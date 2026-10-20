use crate::tree::enums::RbColor;

pub mod core;
pub mod delete;
pub mod display;
pub mod enums;
pub mod insert;

#[derive(Debug)]
pub struct RbElem {
    pub(crate) key: u32,
    pub(crate) parent: Option<usize>,
    pub(crate) left: Option<usize>,
    pub(crate) right: Option<usize>,
    pub(crate) color: RbColor,
}

#[derive(Debug)]
pub struct RbTree {
    pub(crate) root: Option<usize>,
    pub(crate) nodes: Vec<RbElem>,
}
