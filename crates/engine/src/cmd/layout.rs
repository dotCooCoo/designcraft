//! Layout menu and Pages/Layers panels: pages, spreads, parents, margins & columns, layers.

use std::collections::HashMap;
use std::sync::Arc;

use designcraft_doc::{
    Content, Guide, Hyperlink, HyperlinkSource, Item, ItemId, LAYER_COLORS, Layer, LayerId, Margins, SpreadId, SpreadRef, Story, StoryId,
};
use serde_json::{Value, json};

use super::{CommandSpec, bad, cmd, has_doc, ok, str_param};
use crate::{Result, Session};

mod layers;
mod margins;
mod pages;
mod parents;
mod registry;
mod sections;

pub(crate) use pages::resize_pages;
pub use registry::specs;
pub use sections::chapter_numbering;

#[cfg(test)]
#[path = "parent_delete_tests.rs"]
mod parent_delete_tests;
