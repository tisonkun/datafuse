// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under Apache License, Version 2.0.

mod datasource;
mod memory;

pub use self::datasource::IDataSourceProvider;
pub use self::memory::MemorySource;

use crate::datatypes::DataSchemaRef;
use crate::error::Result;
use crate::planners::{PlanNode, ReadDataSourcePlan};
