// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A handy ECS
//!
//! hecs provides a high-performance, minimalist entity-component-system (ECS) world. It is a
//! library, not a framework. In place of an explicit "System" abstraction, a `World`'s entities are
//! easily queried from regular code. Organize your application however you like!
//!
//! In order of importance, hecs pursues:
//! - fast traversals
//! - a simple interface
//! - a small dependency closure
//! - exclusion of externally-implementable functionality
//!
//! ```
//! # use hecs::*;
//! let mut world = World::new();
//! // Nearly any type can be used as a component with zero boilerplate
//! let a = world.spawn((123, true, "abc"));
//! let b = world.spawn((42, false));
//! // Systems can be simple for loops
//! for (id, (number, &flag)) in world.query::<(&mut i32, &bool)>().iter() {
//!   if flag { *number *= 2; }
//! }
//! // Random access is simple and safe
//! assert_eq!(*world.get::<i32>(a).unwrap(), 246);
//! assert_eq!(*world.get::<i32>(b).unwrap(), 42);
//! ```

#![warn(missing_docs)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

/// Imagine macro parameters, but more like those Russian dolls.
///
/// Calls m!(A, B, C), m!(A, B), m!(B), and m!() for i.e. (m, A, B, C)
/// where m is any macro, for any number of parameters.
macro_rules! smaller_tuples_too {
    ($m: ident, $ty: ident) => {
        $m!{$ty}
        $m!{}
    };
    ($m: ident, $ty: ident, $($tt: ident),*) => {
        $m!{$ty, $($tt),*}
        smaller_tuples_too!{$m, $($tt),*}
    };
}

mod archetype;
mod borrow;
mod bundle;
mod entities;
mod entity_builder;
mod query;
mod query_one;
mod world;

pub use archetype::Archetype;
pub use borrow::{EntityRef, Ref, RefMut};
pub use bundle::{Bundle, DynamicBundle, MissingComponent};
pub use entities::{Entity, NoSuchEntity};
pub use entity_builder::{BuiltEntity, EntityBuilder};
pub use query::{Access, BatchedIter, Query, QueryBorrow, QueryIter, With, Without};
pub use query_one::QueryOne;
pub use world::{ArchetypesGeneration, Component, ComponentError, Iter, SpawnBatchIter, World};

// Unstable implementation details needed by the macros
#[doc(hidden)]
pub use archetype::TypeInfo;
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use lazy_static;
#[doc(hidden)]
pub use query::Fetch;

#[cfg(feature = "macros")]
pub use hecs_macros::Bundle;
