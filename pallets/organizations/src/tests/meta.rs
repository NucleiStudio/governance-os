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

use super::mock::Organizations;
use sp_std::collections::btree_set::BTreeSet;

#[test]
fn org_ids_are_different() {
    let mut all_ids = BTreeSet::new();

    assert!(Organizations::org_id_for(1) != Organizations::org_id_for(2));
    for i in 0..100 {
        let id = Organizations::org_id_for(i);
        assert!(!all_ids.contains(&id));
        all_ids.insert(id);
    }
}
