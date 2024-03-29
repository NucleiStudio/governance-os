/*
 * Copyright 2020 Nuclei Studio OÜ
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! A compilation of traits and helpers for implementing the Governance OS

#![cfg_attr(not(feature = "std"), no_std)]

mod acl;
mod currencies;
pub mod errors;
pub mod testing;
pub mod traits;
mod voting;

#[macro_export]
/// Use this macro to easily implement `Default` for a given enum. This avoids
/// having to type the same code everytime.
macro_rules! impl_enum_default {
    ($target:ident, $default:ident) => {
        impl Default for $target {
            fn default() -> Self {
                Self::$default
            }
        }
    };
}
