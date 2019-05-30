extern crate build_plan;

use build_plan::{BuildPlan, TargetKind};

use std::process::Command;

/// Basic test that retrieves the build plan of this integration test.
#[test]
fn reflect() {
    let plan = Command::new(env!("CARGO"))
        .arg("-Zunstable-options")
        .arg("build")
        .args(&["--target-dir", "target_test"])
        .arg("--build-plan")
        .arg("--test")
        .arg("integration")
        .output().unwrap().stdout;

    let plan = BuildPlan::from_cargo_output(plan).unwrap();
    println!("{:#?}", plan);

    // We should have exactly one invocation where the target is a test
    let test_invocations = plan.invocations.iter().filter(|inv| inv.target_kind == TargetKind::Test).collect::<Vec<_>>();
    assert_eq!(test_invocations.len(), 1);

    let test_invocation = test_invocations[0];
    assert_eq!(test_invocation.package_name, "build-plan");
    assert_eq!(test_invocation.program, "rustc");
}
