use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn examples(kind: &str) -> Vec<(String, PathBuf)> {
    let mut found = Vec::new();
    for folder in fs::read_dir(repo().join("examples").join(kind))
        .expect("examples folder")
        .flatten()
    {
        let title = folder.file_name().to_string_lossy().into_owned();
        for file in fs::read_dir(folder.path()).expect("example files").flatten() {
            found.push((title.clone(), file.path()));
        }
    }
    found.sort();
    found
}

fn round_trip<T: DeserializeOwned + Serialize>(value: &Value) -> Result<Value, serde_json::Error> {
    let parsed: T = serde_json::from_value(value.clone())?;
    serde_json::to_value(parsed)
}

fn through(title: &str, value: &Value) -> Result<Value, serde_json::Error> {
    match title {
        "BookingCancelled" => round_trip::<goway_events::BookingCancelled>(value),
        "WalletEligibilityChanged" => round_trip::<goway_events::WalletEligibilityChanged>(value),
        "DriverStatsChanged" => round_trip::<goway_events::DriverStatsChanged>(value),
        "ClientFrame" => round_trip::<goway_events::ClientFrame>(value),
        "ServerFrame" => round_trip::<goway_events::ServerFrame>(value),
        other => panic!("no Rust type mapped for examples of {other}; add it to `through`"),
    }
}

fn schema_check(schemas: &Value, title: &str, value: &Value) -> Vec<String> {
    let schema = json!({
        "definitions": schemas["definitions"],
        "allOf": [{ "$ref": format!("#/definitions/{title}") }],
    });
    let validator = jsonschema::draft7::options()
        .should_validate_formats(true)
        .build(&schema)
        .expect("schema compiles");
    validator.iter_errors(value).map(|error| error.to_string()).collect()
}

#[test]
fn every_valid_example_reads_and_what_rust_writes_still_matches_the_schema() {
    let schemas = read(&repo().join("build/schemas.json"));
    for (title, path) in examples("valid") {
        let original = read(&path);
        let written = through(&title, &original).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let problems = schema_check(&schemas, &title, &written);
        assert!(
            problems.is_empty(),
            "{} written back as {written}: {problems:?}",
            path.display()
        );
        let again = through(&title, &written).expect("reads its own output");
        assert_eq!(again, written, "{} does not read back stable", path.display());
    }
}

#[test]
fn every_invalid_example_is_refused() {
    for (title, path) in examples("invalid") {
        assert!(
            through(&title, &read(&path)).is_err(),
            "{} was accepted",
            path.display()
        );
    }
}

#[test]
fn a_consumer_reads_an_event_with_a_field_it_does_not_know_yet() {
    let mut event = read(&repo().join("examples/valid/BookingCancelled/basic.json"));
    event["refund"] = json!({ "amount": 1000, "currency": "UZS" });
    let parsed: goway_events::BookingCancelled = serde_json::from_value(event).expect("unknown fields are ignored");
    assert_eq!(parsed.by, goway_events::CancelledBy::Carrier);
}
