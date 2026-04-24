// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::message2::MessageFormatter;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestSuite {
    scenario: String,
    tests: Vec<TestCase>,
}

#[derive(Deserialize, Debug)]
struct TestCase {
    description: Option<String>,
    src: String,
    exp: Option<String>,
    params: Option<Vec<TestParam>>,
    #[serde(rename = "expErrors")]
    exp_errors: Option<Vec<TestError>>,
}

#[derive(Deserialize, Debug)]
struct TestParam {
    name: String,
    value: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct TestError {
    #[serde(rename = "type")]
    error_type: String,
}

fn run_tests_in_file(file_path: &str, expected_failures: usize) {
    let file_content = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));
    let suite: TestSuite = serde_json::from_str(&file_content)
        .unwrap_or_else(|e| panic!("Failed to parse JSON in {}: {}", file_path, e));

    let mut failures = Vec::new();
    let mut _total = 0;

    for test in suite.tests {
        _total += 1;
        let formatter = MessageFormatter::try_new(&test.src);

        if let Ok(fmt) = formatter {
            let result = fmt.format();
            if let Some(expected) = test.exp {
                if result != expected {
                    failures.push(format!(
                        "Suite: '{}', Case: '{}' (Expected '{}', got '{}')",
                        suite.scenario,
                        test.description.clone().unwrap_or_else(|| test.src.clone()),
                        expected,
                        result
                    ));
                }
            }
        } else {
            failures.push(format!(
                "Suite: '{}', Case: '{}' (Failed to construct formatter)",
                suite.scenario,
                test.description.clone().unwrap_or_else(|| test.src.clone())
            ));
        }
    }

    let failed = failures.len();
    if failed != expected_failures {
        for failure in &failures {
            println!("{}", failure);
        }
    }

    assert_eq!(
        failed, expected_failures,
        "Expected {} tests to fail in {}, but {} failed",
        expected_failures, file_path, failed
    );
}

#[test]
fn run_syntax_conformance() {
    // TODO: Get all these tests to pass, then add more!
    run_tests_in_file(
        "tests/message2/data/message-format-wg-tests/syntax.json",
        114,
    );
}

#[test]
fn run_non_spec_tests() {
    // TODO: Get all these tests to pass, then add more!
    run_tests_in_file("tests/message2/data/matches-whitespace.json", 4);
}
