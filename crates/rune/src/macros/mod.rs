//! The macro system of Rune.

mod format_args;
mod functions;
mod macro_compiler;
mod macro_context;
mod quote_fn;
mod storage;
mod token_stream;

pub use self::format_args::FormatArgs;
pub use self::functions::{eval, resolve, stringify, to_tokens};
pub use self::macro_context::{with_context, IntoLit, MacroContext};
pub use self::quote_fn::{quote_fn, Quote};
pub use self::storage::Storage;
pub use self::token_stream::{ToTokens, TokenStream, TokenStreamIter};

pub(crate) use self::macro_compiler::MacroCompiler;
pub(crate) use self::macro_context::{current_context, current_stream_span};
