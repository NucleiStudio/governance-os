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

//! A collection of helpers and well know values used accross the Governance OS
//! codebase when writing benchmarks.

use frame_system::{Module as System, Trait as SystemTrait};

pub const SEED: u32 = 0;

#[macro_export]
/// This macro can be used to generate tests for your benchmarks. This avoid having
/// to type the same code everytime. Usage should be similar to
/// `create_benchmarking_test! { new_test_ext, Test, dispatchable, test_benchmark_dispatchable }`.
macro_rules! create_benchmarking_test {
    ($new_test_ext:ident, $runtime:ident, $function:ident, $bench:ident) => {
        #[test]
        fn $function() {
            $new_test_ext().execute_with(|| {
                assert_ok!($bench::<$runtime>());
            })
        }
    };
}

/// Advance blocks by the `x`. Useful when benchmarks depends on block numbers changing.
pub fn advance_blocks<T: SystemTrait>(x: T::BlockNumber) {
    System::<T>::set_block_number(System::<T>::block_number() + x);
}
