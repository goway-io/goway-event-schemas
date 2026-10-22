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
        "UserCreated" => round_trip::<goway_events::UserCreated>(value),
        "UserUpdated" => round_trip::<goway_events::UserUpdated>(value),
        "UserDeleted" => round_trip::<goway_events::UserDeleted>(value),
        "MembershipCreated" => round_trip::<goway_events::MembershipCreated>(value),
        "MembershipUpdated" => round_trip::<goway_events::MembershipUpdated>(value),
        "MembershipRemoved" => round_trip::<goway_events::MembershipRemoved>(value),
        "UserLogin" => round_trip::<goway_events::UserLogin>(value),
        "UserSessionRevoked" => round_trip::<goway_events::UserSessionRevoked>(value),
        "UserSessionsClosed" => round_trip::<goway_events::UserSessionsClosed>(value),
        "UserBlocked" => round_trip::<goway_events::UserBlocked>(value),
        "UserUnblocked" => round_trip::<goway_events::UserUnblocked>(value),
        "DeviceUpserted" => round_trip::<goway_events::DeviceUpserted>(value),
        "DeviceRemoved" => round_trip::<goway_events::DeviceRemoved>(value),
        "OrgUpdated" => round_trip::<goway_events::OrgUpdated>(value),
        "VehicleUpserted" => round_trip::<goway_events::VehicleUpserted>(value),
        "VehicleRemoved" => round_trip::<goway_events::VehicleRemoved>(value),
        "CarrierDocumentUpdated" => round_trip::<goway_events::CarrierDocumentUpdated>(value),
        "CarrierDocumentReviewed" => round_trip::<goway_events::CarrierDocumentReviewed>(value),
        "CarrierDocumentExpired" => round_trip::<goway_events::CarrierDocumentExpired>(value),
        "CarrierDocumentExpiring" => round_trip::<goway_events::CarrierDocumentExpiring>(value),
        "CarrierLineChanged" => round_trip::<goway_events::CarrierLineChanged>(value),
        "CarrierVerificationChanged" => round_trip::<goway_events::CarrierVerificationChanged>(value),
        "DriverOfferFilterChanged" => round_trip::<goway_events::DriverOfferFilterChanged>(value),
        "SupportTicketCreated" => round_trip::<goway_events::SupportTicketCreated>(value),
        "SupportTicketReplied" => round_trip::<goway_events::SupportTicketReplied>(value),
        "DriverStatsChanged" => round_trip::<goway_events::DriverStatsChanged>(value),
        "LoadPosted" => round_trip::<goway_events::LoadPosted>(value),
        "LoadUpdated" => round_trip::<goway_events::LoadUpdated>(value),
        "LoadCancelled" => round_trip::<goway_events::LoadCancelled>(value),
        "LoadExpired" => round_trip::<goway_events::LoadExpired>(value),
        "OfferCreated" => round_trip::<goway_events::OfferCreated>(value),
        "OfferAccepted" => round_trip::<goway_events::OfferAccepted>(value),
        "BookingCreated" => round_trip::<goway_events::BookingCreated>(value),
        "BookingStatusChanged" => round_trip::<goway_events::BookingStatusChanged>(value),
        "BookingEtaUpdated" => round_trip::<goway_events::BookingEtaUpdated>(value),
        "BookingCompleted" => round_trip::<goway_events::BookingCompleted>(value),
        "BookingCancelled" => round_trip::<goway_events::BookingCancelled>(value),
        "BookingAmountClaimed" => round_trip::<goway_events::BookingAmountClaimed>(value),
        "BookingAmountSettled" => round_trip::<goway_events::BookingAmountSettled>(value),
        "BookingReturnLoads" => round_trip::<goway_events::BookingReturnLoads>(value),
        "BookingPlanDue" => round_trip::<goway_events::BookingPlanDue>(value),
        "WalletLowBalance" => round_trip::<goway_events::WalletLowBalance>(value),
        "ChatMessageCreated" => round_trip::<goway_events::ChatMessageCreated>(value),
        "ChatRead" => round_trip::<goway_events::ChatRead>(value),
        "SupportMessageCreated" => round_trip::<goway_events::SupportMessageCreated>(value),
        "BookingObjectionOpened" => round_trip::<goway_events::BookingObjectionOpened>(value),
        "BookingObjectionResolved" => round_trip::<goway_events::BookingObjectionResolved>(value),
        "BookingWaitingStarted" => round_trip::<goway_events::BookingWaitingStarted>(value),
        "BookingIssueOpened" => round_trip::<goway_events::BookingIssueOpened>(value),
        "BookingIssueResolved" => round_trip::<goway_events::BookingIssueResolved>(value),
        "ShipperStatsChanged" => round_trip::<goway_events::ShipperStatsChanged>(value),
        "LoadPlanUnclaimed" => round_trip::<goway_events::LoadPlanUnclaimed>(value),
        "RatingCreated" => round_trip::<goway_events::RatingCreated>(value),
        "WalletEntryPosted" => round_trip::<goway_events::WalletEntryPosted>(value),
        "WalletEligibilityChanged" => round_trip::<goway_events::WalletEligibilityChanged>(value),
        "WalletToppedUp" => round_trip::<goway_events::WalletToppedUp>(value),
        "NotificationCreated" => round_trip::<goway_events::NotificationCreated>(value),
        "DevicePushFailed" => round_trip::<goway_events::DevicePushFailed>(value),
        "RegionChanged" => round_trip::<goway_events::RegionChanged>(value),
        "SettingsChanged" => round_trip::<goway_events::SettingsChanged>(value),
        "DriverOnline" => round_trip::<goway_events::DriverOnline>(value),
        "DriverOffline" => round_trip::<goway_events::DriverOffline>(value),
        "LocationSampled" => round_trip::<goway_events::LocationSampled>(value),
        "DispatchOffered" => round_trip::<goway_events::DispatchOffered>(value),
        "DispatchExhausted" => round_trip::<goway_events::DispatchExhausted>(value),
        "OfferResolved" => round_trip::<goway_events::OfferResolved>(value),
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
