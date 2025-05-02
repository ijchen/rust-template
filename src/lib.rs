// These markdown links are used to override the docs.rs web links present at
// the bottom of README.md. These two lists must be kept in sync, or the links
// included here will be hard-coded links to docs.rs, not relative doc links.
// ON_RELEASE: the below link(s) should be verified to match the readme, and
// this "on release" comment removed (the above one should stay).
//! [`ExampleItem`]: ExampleItem
// TODO: update ExampleItem, or remove the item entirely if unused
#![cfg_attr(any(doc, test), doc = include_str!("../README.md"))]
