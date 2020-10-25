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

use crate::CallTags::*;
use governance_os_support::rules::SuperSetter;

macro_rules! test_master_superset_internal {
    ($master:ident, $child:ident) => {
        assert_eq!($master.is_superset(&$child), true);
        assert_eq!($child.is_superset(&$master), false);
    };

    ($master:ident, $child:ident, $($children:ident),+) => {
        test_master_superset_internal!($master, $child);
        test_master_superset_internal!($master, $($children),+);
    };
}

macro_rules! test_master_superset {
    ($test_name:ident, $master:ident, $($children:ident),*) => {
        #[test]
        fn $test_name() {
            test_master_superset_internal!($master, $($children),+);
        }
    };
}

macro_rules! test_superset_itself {
    ($test_name:ident, $master:ident) => {
        #[test]
        fn $test_name() {
            assert_eq!($master.is_superset(&$master), true);
        }
    };
}

test_master_superset!(any_supersets_all, Any, System, Economic, Bylaws);

test_superset_itself!(any_supersets_itself, Any);
test_superset_itself!(system_supersets_itself, System);
test_superset_itself!(economic_supersets_itself, Economic);
test_superset_itself!(bylaws_supersets_itself, Bylaws);
