#[cfg(feature = "strings")]
mod case;
#[cfg(feature = "strings")]
mod concat;
#[cfg(feature = "strings")]
mod extract;
#[cfg(feature = "find_many")]
mod find_many;
#[cfg(feature = "extract_jsonpath")]
mod json_path;
#[cfg(feature = "strings")]
mod namespace;
#[cfg(feature = "string_pad")]
mod pad;
#[cfg(feature = "strings")]
mod replace;
#[cfg(feature = "string_reverse")]
mod reverse;
#[cfg(feature = "strings")]
mod split;
#[cfg(feature = "strings")]
mod strip;
#[cfg(feature = "strings")]
mod substring;

#[cfg(all(not(feature = "nightly"), feature = "strings"))]
mod unicode_internals;

#[cfg(feature = "strings")]
pub use concat::*;
#[cfg(feature = "find_many")]
pub use find_many::*;
#[cfg(feature = "extract_jsonpath")]
pub use json_path::*;
#[cfg(feature = "strings")]
pub use namespace::*;
use polars_core::prelude::*;
#[cfg(feature = "strings")]
pub use split::*;
#[cfg(feature = "strings")]
pub use strip::*;

pub trait AsUtf8 {
    fn as_utf8(&self) -> &Utf8Chunked;
}

impl AsUtf8 for Utf8Chunked {
    fn as_utf8(&self) -> &Utf8Chunked {
        self
    }
}
