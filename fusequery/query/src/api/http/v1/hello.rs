// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use warp::{Filter, Reply, Rejection};

use crate::configs::Config;
use crate::sessions::SessionManagerRef;
use common_exception::Result;

pub struct HelloRouter {
    sessions: SessionManagerRef,
}

impl HelloRouter {
    pub fn create(sessions: SessionManagerRef) -> Self {
        HelloRouter { sessions }
    }

    pub fn build(&self) -> Result<impl Filter<Extract=impl Reply, Error=Rejection> + Clone> {
        let cfg = self.sessions.get_conf();
        Ok(warp::path!("v1" / "hello").map(move || format!("{:?}", cfg)))
    }
}
