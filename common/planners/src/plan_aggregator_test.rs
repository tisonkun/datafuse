// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_aggregator_plan() -> anyhow::Result<()> {
    use std::sync::Arc;

    use pretty_assertions::assert_eq;

    use crate::*;

    let source = Test::create().generate_source_plan_for_test(10000)?;
    let plan = PlanBuilder::from(&source)
        .aggregate_partial(vec![sum(col("number")).alias("sumx")], vec![])?
        .aggregate_final(vec![sum(col("number")).alias("sumx")], vec![])?
        .project(vec![col("sumx")])?
        .build()?;
    let explain = PlanNode::Explain(ExplainPlan {
        typ: ExplainType::Syntax,
        input: Arc::new(plan),
    });
    let expect = "Projection: sumx:UInt64\
    \n  AggregatorFinal: groupBy=[[]], aggr=[[sum([number]) as sumx]]\
    \n    AggregatorPartial: groupBy=[[]], aggr=[[sum([number]) as sumx]]\
    \n      ReadDataSource: scan partitions: [8], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]";
    let actual = format!("{:?}", explain);
    assert_eq!(expect, actual);
    Ok(())
}
