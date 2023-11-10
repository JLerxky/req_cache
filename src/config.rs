// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cloud_util::tracer::LogConfig;
use common_rs::consul::ConsulConfig;
use serde::{Deserialize, Serialize};
use storage_hal::StorageConfig;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub port: u16,
    pub limit_uri_regex_vec: Vec<String>,
    pub consul_config: Option<ConsulConfig>,
    pub log_config: LogConfig,
    pub storage_config: StorageConfig,
}
