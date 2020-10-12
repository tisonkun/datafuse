// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;

pub trait IDataSourceProvider {
    // Return the schema of this datasource.
    fn schema(&self) -> Result<DataSchemaRef>;

    // Return the ReadDataSourcePlan that how to read the datasource.
    // Here we can push down some plans(Filter/Limit/Project) to datasource for optimizer.
    // ReadDataSourcePlan determines the number of parallel executors(transforms) on processor pipeline.
    fn read_plan(&self, plans: Vec<PlanNode>) -> Result<ReadDataSourcePlan>;
}
