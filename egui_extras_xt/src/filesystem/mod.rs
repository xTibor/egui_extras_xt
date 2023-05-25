mod breadcrumb_bar;
mod directory_cache;
mod directory_tree_view;
mod path_symbol;

pub use breadcrumb_bar::BreadcrumbBar;
pub use directory_tree_view::DirectoryTreeViewWidget;

// ----------------------------------------------------------------------------

use egui::Ui;
use std::path::Path;

pub type DirectoryFilter<'a> = Box<dyn Fn(&Path) -> bool + 'a>;

pub type DirectoryContextMenu<'a> = (
    Box<dyn Fn(&mut Ui, &Path) + 'a>,
    Box<dyn Fn(&Path) -> bool + 'a>,
);

pub type DirectoryHoverUi<'a> = (
    Box<dyn Fn(&mut Ui, &Path) + 'a>,
    Box<dyn Fn(&Path) -> bool + 'a>,
);
