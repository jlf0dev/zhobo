use crate::tree::{item::DatabaseTreeItem, treeitems_iter::TreeItemsIterator};

pub struct TreeIterator<'a> {
    item_iter: TreeItemsIterator<'a>,
    selection: Option<usize>,
}

impl<'a> TreeIterator<'a> {
    pub const fn new(item_iter: TreeItemsIterator<'a>, selection: Option<usize>) -> Self {
        Self {
            item_iter,
            selection,
        }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = (&'a DatabaseTreeItem, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.item_iter
            .next()
            .map(|(index, item)| (item, self.selection.map(|i| i == index).unwrap_or_default()))
    }
}
