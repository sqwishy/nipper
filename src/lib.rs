//! HTML manipulation with CSS selectors.
//!
//! # Features
//!
//! * Iteration
//! * Manipulation
//! * Property
//! * Query
//! * Traversal
//!
//! # Get started
//!
//! ```
//! use nipper::Document;
//!
//! let html = r#"<div>
//!     <a href="/1">One</a>
//!     <a href="/2">Two</a>
//!     <a href="/3">Three</a>
//! </div>"#;
//!
//! let document = Document::from(html);
//! let a = document.select("a:nth-child(3)");
//! let text: &str = &a.text();
//! assert!(text == "Three");
//! ```
//!

// #![deny(missing_docs)] // TODO: add this back in.
mod document;
mod dom_tree;
mod element;
mod manipulation;
mod matcher;
mod property;
mod query;
mod selection;
mod traversal;

pub use document::Document;
pub use document::IntoAtomic;
pub use dom_tree::Node;
#[doc(hidden)]
pub use dom_tree::NodeId;
pub use dom_tree::NodeRef;
#[doc(hidden)]
pub use dom_tree::SerializableNodeRef;
pub use dom_tree::StrTendril;
pub use matcher::MatchScope;
pub use matcher::Matcher;
pub use matcher::Matches;
pub use selection::Selection;
pub use traversal::Selections;
