//! <div align="center">
//!     <img alt="Rune Logo" src="https://raw.githubusercontent.com/rune-rs/rune/master/assets/icon.png" />
//! </div>
//!
//! <br>
//!
//! <div align="center">
//! <a href="https://rune-rs.github.io">
//!     <b>Visit the site 🌐</b>
//! </a>
//! -
//! <a href="https://rune-rs.github.io/book/">
//!     <b>Read the book 📖</b>
//! </a>
//! </div>
//!
//! <br>
//!
//! <div align="center">
//! <a href="https://github.com/rune-rs/rune/actions">
//!     <img alt="Build Status" src="https://github.com/rune-rs/rune/workflows/Build/badge.svg">
//! </a>
//!
//! <a href="https://github.com/rune-rs/rune/actions">
//!     <img alt="Site Status" src="https://github.com/rune-rs/rune/workflows/Site/badge.svg">
//! </a>
//!
//! <a href="https://crates.io/crates/rune">
//!     <img alt="crates.io" src="https://img.shields.io/crates/v/rune.svg">
//! </a>
//!
//! <a href="https://docs.rs/rune">
//!     <img alt="docs.rs" src="https://docs.rs/rune/badge.svg">
//! </a>
//!
//! <a href="https://discord.gg/v5AeNkT">
//!     <img alt="Chat on Discord" src="https://img.shields.io/discord/558644981137670144.svg?logo=discord&style=flat-square">
//! </a>
//! </div>
//!
//! <br>
//!
//! A stack-based virtual machine for the Rust programming language.
//!
//! This is the driver for the [Rune language].
//! [Rune Language]: https://rune-rs.github.io

#![deny(missing_docs)]
#![allow(clippy::enum_variant_names, clippy::too_many_arguments)]

mod any;
mod context;
mod value;
mod vm;
#[macro_use]
mod macros;
mod access;
mod any_obj;
mod args;
mod awaited;
pub mod budget;
mod bytes;
mod call;
mod compile_meta;
mod const_value;
pub mod debug;
pub mod format;
mod from_value;
mod function;
mod future;
mod generator;
mod generator_state;
mod guarded_args;
mod hash;
mod id;
mod inst;
mod item;
mod label;
pub mod module;
pub mod modules;
mod named;
mod names;
mod object;
mod panic;
mod protocol;
mod raw_str;
mod select;
mod serde;
mod shared;
mod source;
mod span;
mod spanned_error;
mod stack;
mod static_string;
mod static_type;
mod stream;
mod to_value;
mod tuple;
mod type_;
mod type_info;
mod type_of;
mod unit;
mod vec;
mod vec_tuple;
mod vm_call;
mod vm_error;
mod vm_execution;
mod vm_halt;

/// Construct a span that can be used during pattern matching.
///
/// # Examples
///
/// ```rust
/// use runestick::{Span, span};
///
/// let s = Span::new(0, 10);
///
/// assert!(match s {
///     span!(0, 10) => true,
///     _ => false,
/// });
/// ```
#[macro_export]
macro_rules! span {
    ($start:expr, $end:expr) => {
        $crate::Span {
            start: $crate::ByteIndex($start),
            end: $crate::ByteIndex($end),
        }
    };
}

/// The identifier of a source file.
pub type SourceId = usize;

/// Exported result type for convenience.
pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Exported boxed error type for convenience.
pub type Error = anyhow::Error;

pub use self::any_obj::{AnyObj, AnyObjVtable};
pub use self::args::Args;
pub use self::compile_meta::{
    CompileMeta, CompileMetaCapture, CompileMetaEmpty, CompileMetaKind, CompileMetaStruct,
    CompileMetaTuple, CompileSource,
};
pub use self::const_value::ConstValue;
pub use self::format::{Format, FormatSpec};
pub use self::from_value::{FromValue, UnsafeFromValue};
pub use self::generator::Generator;
pub use self::generator_state::GeneratorState;
pub use self::guarded_args::GuardedArgs;
pub use self::id::Id;
pub use self::label::{DebugLabel, Label};
pub use self::module::{InstFnNameHash, Module};
pub use self::named::Named;
pub use self::raw_str::RawStr;
pub use self::select::Select;
pub use self::source::Source;
pub use self::span::{ByteIndex, IntoByteIndex, Span};
pub use self::spanned_error::SpannedError;
pub use self::static_string::StaticString;
pub use self::static_type::{
    StaticType, BOOL_TYPE, BYTES_TYPE, BYTE_TYPE, CHAR_TYPE, FLOAT_TYPE, FORMAT_TYPE,
    FUNCTION_TYPE, FUTURE_TYPE, GENERATOR_STATE_TYPE, GENERATOR_TYPE, INTEGER_TYPE, OBJECT_TYPE,
    OPTION_TYPE, RESULT_TYPE, STREAM_TYPE, STRING_TYPE, TUPLE_TYPE, UNIT_TYPE, VEC_TYPE,
};
pub use self::stream::Stream;
pub use self::to_value::{ToValue, UnsafeToValue};
pub use self::tuple::Tuple;
pub use self::type_::Type;
pub use self::type_info::TypeInfo;
pub use self::vec::Vec;
pub use crate::access::{
    AccessError, BorrowMut, BorrowRef, NotAccessibleMut, NotAccessibleRef, RawExclusiveGuard,
    RawSharedGuard,
};
pub use crate::any::Any;
pub use crate::awaited::Awaited;
pub use crate::bytes::Bytes;
pub use crate::call::Call;
pub use crate::context::{Context, ContextError};
pub use crate::debug::{DebugInfo, DebugInst};
pub use crate::function::Function;
pub use crate::future::Future;
pub use crate::hash::{Hash, IntoTypeHash};
pub use crate::inst::{Inst, InstAssignOp, InstOp, InstTarget, InstValue, PanicReason, TypeCheck};
pub use crate::item::{Component, ComponentRef, IntoComponent, Item};
pub use crate::names::Names;
pub use crate::object::Object;
pub use crate::panic::Panic;
pub use crate::protocol::{
    Protocol, ADD, ADD_ASSIGN, BIT_AND, BIT_AND_ASSIGN, BIT_OR, BIT_OR_ASSIGN, BIT_XOR,
    BIT_XOR_ASSIGN, DIV, DIV_ASSIGN, INDEX_GET, INDEX_SET, INTO_FUTURE, INTO_ITER, MUL, MUL_ASSIGN,
    NEXT, REM, REM_ASSIGN, SHL, SHL_ASSIGN, SHR, SHR_ASSIGN, STRING_DISPLAY, SUB, SUB_ASSIGN,
};
pub use crate::shared::{Mut, RawMut, RawRef, Ref, Shared, SharedPointerGuard};
pub use crate::stack::{Stack, StackError};
pub use crate::type_of::TypeOf;
pub use crate::unit::{Unit, UnitFn, UnitTypeInfo};
pub use crate::value::{
    Rtti, Struct, StructVariant, TupleStruct, TupleVariant, UnitStruct, UnitVariant, Value,
    VariantRtti,
};
pub use crate::vec_tuple::VecTuple;
pub use crate::vm::{CallFrame, Vm};
pub use crate::vm_call::VmCall;
pub use crate::vm_error::{VmError, VmErrorKind, VmIntegerRepr};
pub use crate::vm_execution::VmExecution;
pub use crate::vm_halt::{VmHalt, VmHaltInfo};
pub(crate) use runestick_macros::__internal_impl_any;
pub use runestick_macros::{Any, FromValue};

mod collections {
    pub use hashbrown::{hash_map, HashMap};
    pub use hashbrown::{hash_set, HashSet};
}
