// Copyright 2022 Datafuse Labs.
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

use std::sync::Arc;

use chrono::Utc;
use common_exception::Result;
use common_meta_api::ShareApi;
use common_meta_app::share::RevokeShareObjectReq;
use common_meta_app::share::ShareNameIdent;
use common_storages_share::save_share_spec;
use common_users::UserApiProvider;

use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;
use crate::sessions::TableContext;
use crate::sql::plans::share::RevokeShareObjectPlan;

pub struct RevokeShareObjectInterpreter {
    ctx: Arc<QueryContext>,
    plan: RevokeShareObjectPlan,
}

impl RevokeShareObjectInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: RevokeShareObjectPlan) -> Result<Self> {
        Ok(RevokeShareObjectInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for RevokeShareObjectInterpreter {
    fn name(&self) -> &str {
        "RevokeShareObjectInterpreter"
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let tenant = self.ctx.get_tenant();
        let meta_api = UserApiProvider::instance().get_meta_store_client();
        let req = RevokeShareObjectReq {
            share_name: ShareNameIdent {
                tenant,
                share_name: self.plan.share.clone(),
            },
            object: self.plan.object.clone(),
            privilege: self.plan.privilege,
            update_on: Utc::now(),
        };
        let resp = meta_api.revoke_share_object(req).await?;

        save_share_spec(
            &self.ctx.get_tenant(),
            self.ctx.get_data_operator()?.operator(),
            resp.spec_vec,
            Some(vec![resp.share_table_info]),
        )
        .await?;

        Ok(PipelineBuildResult::create())
    }
}
