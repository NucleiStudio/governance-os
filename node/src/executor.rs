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

use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;

native_executor_instance!(
    pub Executor,
    governance_os_runtime::api::dispatch,
    governance_os_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);
