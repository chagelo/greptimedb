// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;

use common_procedure::{Context as ProcedureContext, Status};
use common_telemetry::warn;
use serde::{Deserialize, Serialize};

use crate::error::{self, Result};
use crate::procedure::region_migration::{Context, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionMigrationAbort {
    reason: String,
}

impl RegionMigrationAbort {
    /// Returns the [RegionMigrationAbort] with `reason`.
    pub fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

#[async_trait::async_trait]
#[typetag::serde]
impl State for RegionMigrationAbort {
    async fn next(
        &mut self,
        ctx: &mut Context,
        _procedure_ctx: &ProcedureContext,
    ) -> Result<(Box<dyn State>, Status)> {
        warn!(
            "Region migration is aborted: {}, region_id: {}, from_peer: {}, to_peer: {}, trigger_reason: {}, {}",
            self.reason,
            ctx.region_id(),
            ctx.persistent_ctx.from_peer,
            ctx.persistent_ctx.to_peer,
            ctx.persistent_ctx.trigger_reason,
            ctx.volatile_ctx.metrics,
        );
        error::MigrationAbortSnafu {
            reason: &self.reason,
        }
        .fail()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
