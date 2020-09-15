use std::{fs::File, io::Read as _};

use property::Property;
use serde::Deserialize;
use serde_json::{Result, Value};

type TestCases = Vec<TestCase>;

#[derive(Property, Deserialize, Debug)]
#[property(get(crate), set(disable), mut(disable))]
#[serde(deny_unknown_fields)]
struct TestCase {
    input: String,
    expected: String,
}

fn load_testcases(path: &str) -> TestCases {
    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("failed to open testcases from {}: {}", path, err));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|err| panic!("failed to load testcases from {}: {}", path, err));
    serde_yaml::from_str(&contents)
        .unwrap_or_else(|err| panic!("failed to parse testcases: {}", err))
}

fn test(case: &TestCase) {
    let expected: Result<Value> = serde_json::from_str(case.expected());
    assert!(
        expected.is_ok(),
        "failed to parse the provided JSON: <{}> since {}",
        case.expected(),
        expected.unwrap_err(),
    );
    let expected = expected.unwrap();

    let normalized = handwritten_json::normalize(case.input());
    assert!(
        normalized.is_ok(),
        "failed to normalize the handwritten JSON: <{}> since {}",
        case.input(),
        normalized.unwrap_err(),
    );
    let normalized = normalized.unwrap();

    let actual: Result<Value> = serde_json::from_str(&normalized);
    assert!(
        actual.is_ok(),
        "failed to parse the normalized JSON: <{}> since {}",
        normalized,
        actual.unwrap_err(),
    );
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn run_testcases() {
    let testcases = load_testcases("tests/testcases.yaml");
    for case in &testcases {
        test(case);
    }
}
