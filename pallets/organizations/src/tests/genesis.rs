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

use super::mock::{ExtBuilder, Organizations, Test};
use crate::Parameters;
use frame_support::StorageMap;

#[test]
fn create_organizations_and_increment_counter() {
    ExtBuilder::default()
        .with_default_org()
        .with_default_org()
        .with_default_org()
        .build()
        .execute_with(|| {
            assert_eq!(Organizations::counter(), 3);
            assert!(Parameters::<Test>::contains_key(Organizations::org_id_for(
                0
            )));
            assert!(Parameters::<Test>::contains_key(Organizations::org_id_for(
                1
            )));
            assert!(Parameters::<Test>::contains_key(Organizations::org_id_for(
                2
            )));
            assert!(!Parameters::<Test>::contains_key(
                Organizations::org_id_for(3)
            ));
        })
}
