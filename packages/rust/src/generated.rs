#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "`AccountType`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AccountType {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "business")]
    Business,
}
impl ::std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Individual => f.write_str("individual"),
            Self::Business => f.write_str("business"),
        }
    }
}
impl ::std::str::FromStr for AccountType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "individual" => Ok(Self::Individual),
            "business" => Ok(Self::Business),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccountType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountType {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The first frame on a new socket, within five seconds: the access token."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Auth {
    #[doc = "The app install's id, the one the device was registered under; lets this socket be signed out on its own when the device is removed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub installation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Access token (JWT)."]
    pub token: ::std::string::String,
}
#[doc = "Frame `auth` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AuthFrame {
    pub data: Auth,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A load appeared on a subscribed lane: newly posted or back after a cancelled booking. A signal; the app reads the list again over REST."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadAdded {
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub plan: bool,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `board_load_added` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadAddedFrame {
    pub data: BoardLoadAdded,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A load left a subscribed lane's board. A signal; the app drops it from the list."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadRemoved {
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub reason: BoardRemovalReason,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `board_load_removed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadRemovedFrame {
    pub data: BoardLoadRemoved,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A load on a subscribed lane was edited: price, pickup window or note. A signal; the app reads it again over REST."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadUpdated {
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub plan: bool,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `board_load_updated` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardLoadUpdatedFrame {
    pub data: BoardLoadUpdated,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`BoardRemovalReason`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoardRemovalReason {
    #[serde(rename = "booked")]
    Booked,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for BoardRemovalReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Booked => f.write_str("booked"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for BoardRemovalReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "booked" => Ok(Self::Booked),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BoardRemovalReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BoardRemovalReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`subscribe_board` was accepted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardSubscribed {
    pub from_region_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `board_subscribed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BoardSubscribedFrame {
    pub data: BoardSubscribed,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "One side of a delivered haul says it settled at another sum; nothing changes until an operator decides."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingAmountClaimed {
    #[doc = "The claimed sum, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: u64,
    pub booking_id: ::uuid::Uuid,
    pub claim_id: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: BookingAmountClaimedCurrency,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The sum on record now, whole so'm (a bare integer; `currency` is UZS today)."]
    pub recorded_amount: u64,
    pub side: BookingSide,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingAmountClaimedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingAmountClaimedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingAmountClaimedCurrency> for ::std::string::String {
    fn from(value: BookingAmountClaimedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingAmountClaimedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingAmountClaimedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingAmountClaimedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingAmountClaimedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "An operator accepted an amount claim; the haul now settles at `settled_amount` and billing adjusts its commission. `claim_side` says whose claim the operator accepted; a shipper's objection is also announced by `booking.objection_resolved`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingAmountSettled {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub claim_side: BookingSide,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: BookingAmountSettledCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The load's short code, for example `GW-7F3K2M`; wallets and receipts show it."]
    pub load_code: ::std::string::String,
    pub load_id: ::uuid::Uuid,
    #[doc = "The sum on record before, whole so'm (a bare integer; `currency` is UZS today)."]
    pub previous_amount: u64,
    #[doc = "The sum the operator settled on, whole so'm (a bare integer; `currency` is UZS today)."]
    pub settled_amount: u64,
    #[doc = "Null only when the load could not be read."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub shipper_org_id: ::std::option::Option<::uuid::Uuid>,
    pub status: BookingStatus,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingAmountSettledCurrency(::std::string::String);
impl ::std::ops::Deref for BookingAmountSettledCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingAmountSettledCurrency> for ::std::string::String {
    fn from(value: BookingAmountSettledCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingAmountSettledCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingAmountSettledCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingAmountSettledCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingAmountSettledCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A booking ended without being completed: either side cancelled before the cargo was loaded (`by` is `shipper` or `carrier`), or an operator cancelled it (`by` is `operator`), which after loading happens only through a reported issue. The load goes back to the board unless the operator withdrew it, which a `load.cancelled` then says."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCancelled {
    pub booking_id: ::uuid::Uuid,
    pub by: CancelledBy,
    pub carrier_org_id: ::uuid::Uuid,
    pub currency: BookingCancelledCurrency,
    #[doc = "Null when no driver was assigned."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub driver_user_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Cancellation fee the shipper owes the carrier, whole so'm (`cancel_fee_percent` once the driver set off); null when none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub fee_amount: ::std::option::Option<u64>,
    #[doc = "The load's short code, for example `GW-7F3K2M`; wallets and receipts show it."]
    pub load_code: ::std::string::String,
    pub load_id: ::uuid::Uuid,
    #[doc = "The cancellation counts against the driver's acceptance rate: the driver's own cancellation of an instant haul, or of a plan later than goway-loads' free-cancel window before pickup, or an operator's cancellation of a booking whose driver never showed up. False for every other shipper and operator cancellation."]
    pub penalized: bool,
    #[doc = "The cancelled booking was a plan."]
    pub plan: bool,
    #[doc = "What the canceller wrote; empty when nothing was written."]
    pub reason: ::std::string::String,
    #[doc = "Code from the cancellation reasons catalogue; null for older clients and system cancellations."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason_code: ::std::option::Option<::std::string::String>,
    pub shipper_org_id: ::uuid::Uuid,
}
#[doc = "`BookingCancelledCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingCancelledCurrency(::std::string::String);
impl ::std::ops::Deref for BookingCancelledCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingCancelledCurrency> for ::std::string::String {
    fn from(value: BookingCancelledCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingCancelledCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingCancelledCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingCancelledCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingCancelledCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The haul was cancelled; the app reads the booking for what it costs whom. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCancelledData {
    pub booking_id: ::uuid::Uuid,
    pub by: CancelledBy,
    #[doc = "The load's short code, for example `GW-7F3K2M`; wallets and receipts show it."]
    pub load_code: ::std::string::String,
    pub load_id: ::uuid::Uuid,
    #[doc = "The cancelled booking was a plan."]
    pub plan: bool,
    #[doc = "What the canceller wrote; empty when nothing was written."]
    pub reason: ::std::string::String,
    #[doc = "Code from the cancellation reasons catalogue; null for older clients and system cancellations."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason_code: ::std::option::Option<::std::string::String>,
}
#[doc = "Frame `booking_cancelled` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCancelledFrame {
    pub data: BookingCancelledData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`BookingChange`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BookingChange {
    #[serde(rename = "issue_opened")]
    IssueOpened,
    #[serde(rename = "issue_resolved")]
    IssueResolved,
    #[serde(rename = "waiting_started")]
    WaitingStarted,
}
impl ::std::fmt::Display for BookingChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::IssueOpened => f.write_str("issue_opened"),
            Self::IssueResolved => f.write_str("issue_resolved"),
            Self::WaitingStarted => f.write_str("waiting_started"),
        }
    }
}
impl ::std::str::FromStr for BookingChange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "issue_opened" => Ok(Self::IssueOpened),
            "issue_resolved" => Ok(Self::IssueResolved),
            "waiting_started" => Ok(Self::WaitingStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BookingChange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingChange {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A delivered haul was closed: by the shipper, by an operator, or by the auto-complete sweep. All three send the same fields. Billing charges its commission from this event."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCompleted {
    #[doc = "Price agreed at booking, whole so'm (a bare integer; `currency` is UZS today)."]
    pub agreed_amount: u64,
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "Price lines; the first is always the agreed tariff."]
    pub charges: ::std::vec::Vec<ChargeLine>,
    #[doc = "When the haul was closed; the code allows null, it is always set. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub completed_by: CompletedBy,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: BookingCompletedCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The load's short code, for example `GW-7F3K2M`; wallets and receipts show it."]
    pub load_code: ::std::string::String,
    pub load_id: ::uuid::Uuid,
    #[doc = "What the haul actually settled at: the agreed price unless another sum was confirmed, whole so'm (a bare integer; `currency` is UZS today)."]
    pub settled_amount: u64,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "Charges plus waiting, whole so'm (a bare integer; `currency` is UZS today); the code allows null."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub total_amount: ::std::option::Option<u64>,
    #[doc = "What the waiting costs, whole so'm (a bare integer; `currency` is UZS today)."]
    pub waiting_amount: u64,
    #[doc = "Whether the waiting is charged to the shipper."]
    pub waiting_billable: bool,
    #[doc = "Billable minutes the driver waited at pickup."]
    pub waiting_minutes: u64,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingCompletedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingCompletedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingCompletedCurrency> for ::std::string::String {
    fn from(value: BookingCompletedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingCompletedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingCompletedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingCompletedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingCompletedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The haul is closed; the app reads the booking for the bill and asks for a rating. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCompletedData {
    pub booking_id: ::uuid::Uuid,
    #[doc = "When the haul was closed; the code allows null, it is always set. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub completed_by: CompletedBy,
    #[doc = "The load's short code, for example `GW-7F3K2M`; wallets and receipts show it."]
    pub load_code: ::std::string::String,
    pub load_id: ::uuid::Uuid,
}
#[doc = "Frame `booking_completed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCompletedFrame {
    pub data: BookingCompletedData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A load was booked. Two paths: an accepted offer, or an instant claim from dispatch, which adds `source: instant`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCreated {
    #[doc = "Agreed price, whole so'm (a bare integer; `currency` is UZS today)."]
    pub agreed_amount: u64,
    pub booking_id: ::uuid::Uuid,
    #[doc = "Start of the span this booking keeps the driver busy, goway-loads' plan buffer included. Dispatch offers the driver nothing whose trip overlaps it."]
    pub busy_from: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "End of that span; null when the haul's duration is unknown, which keeps the driver busy until the booking ends."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub busy_until: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: BookingCreatedCurrency,
    #[doc = "The driver; the offer path allows null, a booking always has one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub driver_user_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Driving time of the haul in minutes; null when routing was unavailable."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub duration_min: ::std::option::Option<u64>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "Earliest pickup; null when not given. A pickup well ahead makes this booking a plan rather than a trip under way."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Latest pickup; null when not given."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The driver booked a plan: a load scheduled ahead, taken from the plan list."]
    pub plan: bool,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load; they hear `booking_created` on the socket too."]
    pub shipper_user_id: ::uuid::Uuid,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<BookingSource>,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingCreatedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingCreatedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingCreatedCurrency> for ::std::string::String {
    fn from(value: BookingCreatedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingCreatedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingCreatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingCreatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingCreatedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A haul was booked. Sent to the booked driver and to the shipper who posted the load; the app reads the booking for the rest. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCreatedData {
    #[doc = "Agreed price, whole so'm (a bare integer; `currency` is UZS today)."]
    pub agreed_amount: u64,
    pub booking_id: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: BookingCreatedDataCurrency,
    pub load_id: ::uuid::Uuid,
    #[doc = "Earliest pickup; null when not given. A pickup well ahead makes this booking a plan rather than a trip under way."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Latest pickup; null when not given."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The driver booked a plan: a load scheduled ahead, taken from the plan list."]
    pub plan: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<BookingSource>,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingCreatedDataCurrency(::std::string::String);
impl ::std::ops::Deref for BookingCreatedDataCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingCreatedDataCurrency> for ::std::string::String {
    fn from(value: BookingCreatedDataCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingCreatedDataCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingCreatedDataCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingCreatedDataCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingCreatedDataCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Frame `booking_created` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCreatedFrame {
    pub data: BookingCreatedData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The arrival window or the road position changed. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingEtaData {
    pub booking_id: ::uuid::Uuid,
    #[doc = "Earliest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Which stop the window is for; null when no arrival is expected any more."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_stop: ::std::option::Option<EtaStop>,
    #[doc = "Latest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The last region boundary the truck crossed, for «Angrendan o'tdi · 142 km qoldi»; null until it crosses one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_waypoint: ::std::option::Option<PassedWaypoint>,
    pub load_id: ::uuid::Uuid,
    #[doc = "Road kilometres left to the next stop; null before the haul sets off or while routing is unavailable."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub remaining_km: ::std::option::Option<f64>,
    pub status: BookingStatus,
}
#[doc = "Frame `booking_eta` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingEtaFrame {
    pub data: BookingEtaData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The arrival window or the time the truck is free again moved: when the haul is booked (`confirmed`), at every carrier step, and on fresh GPS while `en_route` or `in_transit`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingEtaUpdated {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Earliest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Which stop the window is for; null when no arrival is expected any more."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_stop: ::std::option::Option<EtaStop>,
    #[doc = "Latest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub eta_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "When the truck is expected to be empty again. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub free_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Where the truck will be free: the drop-off region; null when unknown."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub free_region_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The last region boundary the truck crossed, for «Angrendan o'tdi · 142 km qoldi»; null until it crosses one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_waypoint: ::std::option::Option<PassedWaypoint>,
    pub load_id: ::uuid::Uuid,
    #[doc = "Road kilometres left to the next stop; null before the haul sets off or while routing is unavailable."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub remaining_km: ::std::option::Option<f64>,
    pub shipper_org_id: ::uuid::Uuid,
    pub status: BookingStatus,
    #[doc = "The booked vehicle's truck class code."]
    pub truck_class: ::std::string::String,
}
#[doc = "Someone on the haul reported a problem («Muammo bor»). Until an operator resolves it the haul is under review: its step does not move, the waiting clock is paused and the driver's cancellation does not count against them."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingIssueOpened {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub issue_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub opened_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub opened_by: IssueSide,
    #[doc = "Code from the issue reasons catalogue."]
    pub reason_code: ::std::string::String,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load."]
    pub shipper_user_id: ::uuid::Uuid,
    #[doc = "The booking's step when the issue was opened."]
    pub status: ::std::string::String,
}
#[doc = "An operator resolved a reported problem; the haul is no longer under review."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingIssueResolved {
    #[doc = "The new price for `reprice`, the added charge for `waiting_fee`, whole so'm; null otherwise."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub amount: ::std::option::Option<u64>,
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub currency: BookingIssueResolvedCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub issue_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub resolution: IssueResolution,
    pub resolved_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load."]
    pub shipper_user_id: ::uuid::Uuid,
}
#[doc = "`BookingIssueResolvedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingIssueResolvedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingIssueResolvedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingIssueResolvedCurrency> for ::std::string::String {
    fn from(value: BookingIssueResolvedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingIssueResolvedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingIssueResolvedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingIssueResolvedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingIssueResolvedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The shipper objected to a completed haul within the objection window: the sum, damage or shortage. An operator decides it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingObjectionOpened {
    #[doc = "The sum the shipper says is right, whole so'm; null when they named none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub amount: ::std::option::Option<u64>,
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub claim_id: ::uuid::Uuid,
    pub currency: BookingObjectionOpenedCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: ObjectionKind,
    pub load_id: ::uuid::Uuid,
    pub opened_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load."]
    pub shipper_user_id: ::uuid::Uuid,
}
#[doc = "`BookingObjectionOpenedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingObjectionOpenedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingObjectionOpenedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingObjectionOpenedCurrency> for ::std::string::String {
    fn from(value: BookingObjectionOpenedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingObjectionOpenedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingObjectionOpenedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingObjectionOpenedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingObjectionOpenedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "An operator decided a shipper's objection. An accepted one that names a sum also sends `booking.amount_settled`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingObjectionResolved {
    pub accepted: bool,
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub claim_id: ::uuid::Uuid,
    pub currency: BookingObjectionResolvedCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: ObjectionKind,
    pub load_id: ::uuid::Uuid,
    #[doc = "The operator's reason, shown to both sides."]
    pub note: ::std::string::String,
    pub resolved_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "What the haul now settles at when the decision changed it, whole so'm; null otherwise."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub settled_amount: ::std::option::Option<u64>,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load."]
    pub shipper_user_id: ::uuid::Uuid,
}
#[doc = "`BookingObjectionResolvedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingObjectionResolvedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingObjectionResolvedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingObjectionResolvedCurrency> for ::std::string::String {
    fn from(value: BookingObjectionResolvedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingObjectionResolvedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingObjectionResolvedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingObjectionResolvedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingObjectionResolvedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A booked plan's pickup is near: time for the reminder, time to set off, or the pickup time came and the driver has not set off. One event per stage per booking."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingPlanDue {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "First pickup stop's address; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_address: ::std::option::Option<::std::string::String>,
    pub pickup_from: ::chrono::DateTime<::chrono::offset::Utc>,
    pub shipper_org_id: ::uuid::Uuid,
    pub stage: PlanStage,
}
#[doc = "Hours before a truck is empty: loads are waiting in the region where it unloads. Sent once per haul."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingReturnLoads {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "Matching loads waiting there."]
    pub count: ::std::num::NonZeroU64,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "When the truck is expected to be empty. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub free_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "How many of them go back to the haul's pickup region."]
    pub heads_home: u64,
    #[doc = "The drop-off region, where the truck will be free."]
    pub region_id: ::uuid::Uuid,
}
#[doc = "Which side claims."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BookingSide {
    #[serde(rename = "shipper")]
    Shipper,
    #[serde(rename = "carrier")]
    Carrier,
}
impl ::std::fmt::Display for BookingSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shipper => f.write_str("shipper"),
            Self::Carrier => f.write_str("carrier"),
        }
    }
}
impl ::std::str::FromStr for BookingSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shipper" => Ok(Self::Shipper),
            "carrier" => Ok(Self::Carrier),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BookingSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Present only for an instant claim from dispatch; an accepted offer leaves it out."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BookingSource {
    #[serde(rename = "instant")]
    Instant,
}
impl ::std::fmt::Display for BookingSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Instant => f.write_str("instant"),
        }
    }
}
impl ::std::str::FromStr for BookingSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "instant" => Ok(Self::Instant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BookingSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingSource {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BookingStatus`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BookingStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "en_route")]
    EnRoute,
    #[serde(rename = "at_pickup")]
    AtPickup,
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "in_transit")]
    InTransit,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Confirmed => f.write_str("confirmed"),
            Self::EnRoute => f.write_str("en_route"),
            Self::AtPickup => f.write_str("at_pickup"),
            Self::Loading => f.write_str("loading"),
            Self::InTransit => f.write_str("in_transit"),
            Self::Delivered => f.write_str("delivered"),
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for BookingStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "confirmed" => Ok(Self::Confirmed),
            "en_route" => Ok(Self::EnRoute),
            "at_pickup" => Ok(Self::AtPickup),
            "loading" => Ok(Self::Loading),
            "in_transit" => Ok(Self::InTransit),
            "delivered" => Ok(Self::Delivered),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BookingStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The carrier moved the haul one step on: `en_route`, `at_pickup`, `loading`, `in_transit` or `delivered`. Completion and cancellation have their own subjects."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingStatusChanged {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "When the step happened: the driver's tap, which an app offline for a while sends late. Steps arrive in order; this is the time to show."]
    pub occurred_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub shipper_org_id: ::uuid::Uuid,
    pub status: BookingStatus,
}
#[doc = "The haul moved to another step. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingStatusData {
    pub booking_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "When the step happened: the driver's tap, which an app offline for a while sends late. Steps arrive in order; this is the time to show."]
    pub occurred_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: BookingStatus,
}
#[doc = "Frame `booking_status` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingStatusFrame {
    pub data: BookingStatusData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Something on a haul changed besides its step; sent to the driver and the shipper without a subscription. A signal: the app reads the booking again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingUpdated {
    pub booking_id: ::uuid::Uuid,
    pub change: BookingChange,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub issue_id: ::std::option::Option<::uuid::Uuid>,
}
#[doc = "Frame `booking_updated` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingUpdatedFrame {
    pub data: BookingUpdated,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The free waiting at the pickup ran out and paid waiting began. Once per booking."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingWaitingStarted {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    pub currency: BookingWaitingStartedCurrency,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "Whole so'm per minute."]
    pub rate_per_minute: u64,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load."]
    pub shipper_user_id: ::uuid::Uuid,
    pub started_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "`BookingWaitingStartedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BookingWaitingStartedCurrency(::std::string::String);
impl ::std::ops::Deref for BookingWaitingStartedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BookingWaitingStartedCurrency> for ::std::string::String {
    fn from(value: BookingWaitingStartedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BookingWaitingStartedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BookingWaitingStartedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BookingWaitingStartedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BookingWaitingStartedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Who cancelled."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CancelledBy {
    #[serde(rename = "shipper")]
    Shipper,
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "operator")]
    Operator,
}
impl ::std::fmt::Display for CancelledBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shipper => f.write_str("shipper"),
            Self::Carrier => f.write_str("carrier"),
            Self::Operator => f.write_str("operator"),
        }
    }
}
impl ::std::str::FromStr for CancelledBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shipper" => Ok(Self::Shipper),
            "carrier" => Ok(Self::Carrier),
            "operator" => Ok(Self::Operator),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelledBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelledBy {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A carrier's document passed its expiry date; the hourly sweep marked it expired."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierDocumentExpired {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The document's expiry date. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub expired_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub kind: DocumentKind,
    pub org_id: ::uuid::Uuid,
}
#[doc = "A carrier's document expires within the warning window; sent once per document."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierDocumentExpiring {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The document's expiry date. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub expires_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub kind: DocumentKind,
    pub org_id: ::uuid::Uuid,
}
#[doc = "An operator approved or rejected a carrier's document."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierDocumentReviewed {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: DocumentKind,
    #[doc = "What the operator wrote; a rejection always has one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub note: ::std::option::Option<::std::string::String>,
    pub org_id: ::uuid::Uuid,
    pub status: ReviewDecision,
}
#[doc = "A carrier submitted a new version of a document; it waits for review."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierDocumentUpdated {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: DocumentKind,
    pub org_id: ::uuid::Uuid,
    pub status: SubmittedDocumentStatus,
    #[doc = "Version of this document kind, from 1."]
    pub version: ::std::num::NonZeroU64,
}
#[doc = "A driver opened or changed their line, its return direction or its truck."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierLineChanged {
    #[doc = "The way back is taken too."]
    pub bidirectional: bool,
    pub changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Where the line starts."]
    pub from_region_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    #[doc = "Where the line ends."]
    pub to_region_id: ::uuid::Uuid,
    pub user_id: ::uuid::Uuid,
    #[doc = "The truck for the line; null after it was removed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub vehicle_id: ::std::option::Option<::uuid::Uuid>,
}
#[doc = "A carrier organization's verification moved: submitted or reopened by a replaced document (`pending`), or decided by an operator."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CarrierVerificationChanged {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    pub status: VerificationStatus,
    #[doc = "The organization's members, so a consumer can act on each without its own member list."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_ids: ::std::option::Option<::std::vec::Vec<::uuid::Uuid>>,
}
#[doc = "One priced line of the haul."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChargeLine {
    #[doc = "Line total, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: u64,
    #[doc = "`tariff`, `loaders` or `req:<requirement code>`."]
    pub code: ::std::string::String,
    pub qty: ::std::num::NonZeroU64,
}
#[doc = "A message was written in a haul's chat between the driver and the shipper. Realtime shows it at once; notify pushes it to whoever is not looking."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatMessageCreated {
    pub booking_id: ::uuid::Uuid,
    #[doc = "The sending app's id for the message, so a resend after a lost reply shows once."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub client_message_id: ::std::option::Option<::uuid::Uuid>,
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: ChatMessageKind,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub lat: ::std::option::Option<f64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub lng: ::std::option::Option<f64>,
    pub load_id: ::uuid::Uuid,
    pub message_id: ::uuid::Uuid,
    #[doc = "The `chat_photo` file; the app reads its link over REST."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub photo_file_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Code from the quick replies catalogue when `kind` is `quick_reply`."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub quick_reply_code: ::std::option::Option<::std::string::String>,
    #[doc = "People on the other side who should hear it: today the driver or the person who posted the load."]
    pub recipient_user_ids: ::std::vec::Vec<::uuid::Uuid>,
    pub sender_side: ChatSide,
    pub sender_user_id: ::uuid::Uuid,
    #[doc = "The words; for a quick reply, its text in the sender's language. Null for a bare photo or location."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub text: ::std::option::Option<ChatMessageCreatedText>,
}
#[doc = "The words; for a quick reply, its text in the sender's language. Null for a bare photo or location."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChatMessageCreatedText(::std::string::String);
impl ::std::ops::Deref for ChatMessageCreatedText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ChatMessageCreatedText> for ::std::string::String {
    fn from(value: ChatMessageCreatedText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ChatMessageCreatedText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ChatMessageCreatedText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatMessageCreatedText {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ChatMessageCreatedText {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A message in a haul's chat, to both sides and to the writer's other devices. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatMessageData {
    pub booking_id: ::uuid::Uuid,
    #[doc = "The sending app's id for the message, so a resend after a lost reply shows once."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub client_message_id: ::std::option::Option<::uuid::Uuid>,
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub kind: ChatMessageKind,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub lat: ::std::option::Option<f64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub lng: ::std::option::Option<f64>,
    pub load_id: ::uuid::Uuid,
    pub message_id: ::uuid::Uuid,
    #[doc = "The `chat_photo` file; the app reads its link over REST."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub photo_file_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Code from the quick replies catalogue when `kind` is `quick_reply`."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub quick_reply_code: ::std::option::Option<::std::string::String>,
    pub sender_side: ChatSide,
    pub sender_user_id: ::uuid::Uuid,
    #[doc = "The words; for a quick reply, its text in the sender's language. Null for a bare photo or location."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub text: ::std::option::Option<ChatMessageDataText>,
}
#[doc = "The words; for a quick reply, its text in the sender's language. Null for a bare photo or location."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChatMessageDataText(::std::string::String);
impl ::std::ops::Deref for ChatMessageDataText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ChatMessageDataText> for ::std::string::String {
    fn from(value: ChatMessageDataText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ChatMessageDataText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ChatMessageDataText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatMessageDataText {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ChatMessageDataText {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Frame `chat_message` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatMessageFrame {
    pub data: ChatMessageData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`ChatMessageKind`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChatMessageKind {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "quick_reply")]
    QuickReply,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "location")]
    Location,
}
impl ::std::fmt::Display for ChatMessageKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::QuickReply => f.write_str("quick_reply"),
            Self::Photo => f.write_str("photo"),
            Self::Location => f.write_str("location"),
        }
    }
}
impl ::std::str::FromStr for ChatMessageKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "quick_reply" => Ok(Self::QuickReply),
            "photo" => Ok(Self::Photo),
            "location" => Ok(Self::Location),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatMessageKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatMessageKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Someone read a haul's chat up to a message; the other side sees «o'qildi»."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatRead {
    pub booking_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub last_read_message_id: ::uuid::Uuid,
    pub read_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub reader_side: ChatSide,
    pub reader_user_id: ::uuid::Uuid,
    #[doc = "People on the other side who should hear it: today the driver or the person who posted the load."]
    pub recipient_user_ids: ::std::vec::Vec<::uuid::Uuid>,
}
#[doc = "One side read a haul's chat up to a message. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatReadData {
    pub booking_id: ::uuid::Uuid,
    pub last_read_message_id: ::uuid::Uuid,
    pub read_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub reader_side: ChatSide,
    pub reader_user_id: ::uuid::Uuid,
}
#[doc = "Frame `chat_read` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatReadFrame {
    pub data: ChatReadData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The side of the haul the person writes for."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChatSide {
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "shipper")]
    Shipper,
}
impl ::std::fmt::Display for ChatSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Carrier => f.write_str("carrier"),
            Self::Shipper => f.write_str("shipper"),
        }
    }
}
impl ::std::str::FromStr for ChatSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "carrier" => Ok(Self::Carrier),
            "shipper" => Ok(Self::Shipper),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The other side is typing in a haul's chat. Not stored; show it for a few seconds."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatTyping {
    pub booking_id: ::uuid::Uuid,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Frame `chat_typing` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ChatTypingFrame {
    pub data: ChatTyping,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Any frame an app may send; `type` tells which."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ClientFrame {
    #[doc = "Frame `auth` an app sends to goway-realtime."]
    #[serde(rename = "auth")]
    Auth { data: Auth },
    #[doc = "Frame `ping` an app sends to goway-realtime."]
    #[serde(rename = "ping")]
    Ping {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        data: ::std::option::Option<Ping>,
    },
    #[doc = "Frame `go_online` an app sends to goway-realtime."]
    #[serde(rename = "go_online")]
    GoOnline { data: GoOnline },
    #[doc = "Frame `go_offline` an app sends to goway-realtime."]
    #[serde(rename = "go_offline")]
    GoOffline {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        data: ::std::option::Option<GoOffline>,
    },
    #[doc = "Frame `location` an app sends to goway-realtime."]
    #[serde(rename = "location")]
    Location { data: Location },
    #[doc = "Frame `offer_response` an app sends to goway-realtime."]
    #[serde(rename = "offer_response")]
    OfferResponse { data: OfferResponse },
    #[doc = "Frame `subscribe_booking` an app sends to goway-realtime."]
    #[serde(rename = "subscribe_booking")]
    SubscribeBooking { data: SubscribeBooking },
    #[doc = "Frame `unsubscribe_booking` an app sends to goway-realtime."]
    #[serde(rename = "unsubscribe_booking")]
    UnsubscribeBooking { data: UnsubscribeBooking },
    #[doc = "Frame `subscribe_board` an app sends to goway-realtime."]
    #[serde(rename = "subscribe_board")]
    SubscribeBoard { data: SubscribeBoard },
    #[doc = "Frame `unsubscribe_board` an app sends to goway-realtime."]
    #[serde(rename = "unsubscribe_board")]
    UnsubscribeBoard { data: UnsubscribeBoard },
    #[doc = "Frame `typing` an app sends to goway-realtime."]
    #[serde(rename = "typing")]
    Typing { data: Typing },
}
#[doc = "Who closed the haul: `carrier` when the driver finishes it at the drop-off, `system` for the auto-complete sweep."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompletedBy {
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "shipper")]
    Shipper,
    #[serde(rename = "operator")]
    Operator,
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for CompletedBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Carrier => f.write_str("carrier"),
            Self::Shipper => f.write_str("shipper"),
            Self::Operator => f.write_str("operator"),
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for CompletedBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "carrier" => Ok(Self::Carrier),
            "shipper" => Ok(Self::Shipper),
            "operator" => Ok(Self::Operator),
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompletedBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompletedBy {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The `auth` frame was accepted; the socket is live."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Connected {
    pub config: ConnectionConfig,
    #[doc = "This socket's id, 32 hex characters."]
    pub conn_id: ::std::string::String,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Frame `connected` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConnectedFrame {
    pub data: Connected,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Current realtime settings the app plays by."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConnectionConfig {
    #[doc = "Seconds between durable GPS samples."]
    pub location_sample_seconds: u64,
    #[doc = "Extra seconds after the deadline a late answer is still honoured."]
    pub offer_grace_seconds: u64,
    #[doc = "Seconds a driver has to answer an offer."]
    pub offer_ttl_seconds: u64,
}
#[doc = "`DevicePlatform`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DevicePlatform {
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "web")]
    Web,
}
impl ::std::fmt::Display for DevicePlatform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Android => f.write_str("android"),
            Self::Ios => f.write_str("ios"),
            Self::Web => f.write_str("web"),
        }
    }
}
impl ::std::str::FromStr for DevicePlatform {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::Ios),
            "web" => Ok(Self::Web),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DevicePlatform {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DevicePlatform {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The push provider rejected a device's token as dead; the device should be shown as unreachable."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DevicePushFailed {
    pub device_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub user_id: ::uuid::Uuid,
}
#[doc = "The owner removed a device; its sessions are closed and it is pushed no more."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeviceRemoved {
    pub device_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The app install's own id."]
    pub installation_id: ::std::string::String,
    pub user_id: ::uuid::Uuid,
}
#[doc = "A device was registered or came back after removal, or its push registration changed or was cleared (signing out clears it). `side` is the side its app signed in to: a push for one side goes only to that side's devices; a device without it, not signed in since sides were kept, hears every side."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeviceUpserted {
    pub device_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Whether the phone lets the app show notifications, as the app last said; null when it has not said. FCM accepts a push the phone will not show, so `false` counts the device as unreachable."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notifications_enabled: ::std::option::Option<bool>,
    pub platform: DevicePlatform,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub push_kind: ::std::option::Option<PushKind>,
    #[doc = "Where FCM delivers this device's pushes: a registration token or a Firebase Installation ID, as `push_kind` says; null when the device can no longer be pushed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub push_token: ::std::option::Option<::std::string::String>,
    #[doc = "When the app last uploaded its push registration; a registration not renewed for long is stale."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub push_updated_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<MembershipSide>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "An instant search ran for the whole search window and no driver took the load. The search goes on; the shipper is told so they can raise the price, move the pickup or cancel."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DispatchExhausted {
    #[doc = "Current price, whole so'm."]
    pub amount: u64,
    #[doc = "The person who posted the load."]
    pub created_by: ::uuid::Uuid,
    pub currency: DispatchExhaustedCurrency,
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "The shipper's organization."]
    pub org_id: ::uuid::Uuid,
    pub searched_seconds: u64,
    #[doc = "Drivers the load was offered to so far."]
    pub seen: u64,
    #[doc = "The price GoWay suggests trying next: the current one raised by the configured step."]
    pub suggested_amount: u64,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "`DispatchExhaustedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DispatchExhaustedCurrency(::std::string::String);
impl ::std::ops::Deref for DispatchExhaustedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DispatchExhaustedCurrency> for ::std::string::String {
    fn from(value: DispatchExhaustedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DispatchExhaustedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DispatchExhaustedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DispatchExhaustedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DispatchExhaustedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "An instant load was offered to a wave of drivers on its line, so a driver whose app is in the background still sees it. Loss-tolerant: the socket frame is the offer itself."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DispatchOffered {
    #[doc = "The load's fixed price, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: u64,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: DispatchOfferedCurrency,
    #[doc = "Answer deadline, unix milliseconds in server time."]
    pub deadline_at: i64,
    #[doc = "Drivers the wave reached."]
    pub drivers: ::std::vec::Vec<::uuid::Uuid>,
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
    #[doc = "Truck class code from the reference list."]
    pub truck_class: ::std::string::String,
    pub weight_kg: u64,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DispatchOfferedCurrency(::std::string::String);
impl ::std::ops::Deref for DispatchOfferedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DispatchOfferedCurrency> for ::std::string::String {
    fn from(value: DispatchOfferedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DispatchOfferedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DispatchOfferedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DispatchOfferedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DispatchOfferedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Display currency; charges stay in UZS."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DisplayCurrency {
    #[serde(rename = "UZS")]
    Uzs,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "KZT")]
    Kzt,
}
impl ::std::fmt::Display for DisplayCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uzs => f.write_str("UZS"),
            Self::Usd => f.write_str("USD"),
            Self::Rub => f.write_str("RUB"),
            Self::Kzt => f.write_str("KZT"),
        }
    }
}
impl ::std::str::FromStr for DisplayCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UZS" => Ok(Self::Uzs),
            "USD" => Ok(Self::Usd),
            "RUB" => Ok(Self::Rub),
            "KZT" => Ok(Self::Kzt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DisplayCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DisplayCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DocumentKind`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DocumentKind {
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "tech_passport")]
    TechPassport,
    #[serde(rename = "vehicle_photo")]
    VehiclePhoto,
    #[serde(rename = "power_of_attorney")]
    PowerOfAttorney,
}
impl ::std::fmt::Display for DocumentKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Passport => f.write_str("passport"),
            Self::DriverLicense => f.write_str("driver_license"),
            Self::TechPassport => f.write_str("tech_passport"),
            Self::VehiclePhoto => f.write_str("vehicle_photo"),
            Self::PowerOfAttorney => f.write_str("power_of_attorney"),
        }
    }
}
impl ::std::str::FromStr for DocumentKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "passport" => Ok(Self::Passport),
            "driver_license" => Ok(Self::DriverLicense),
            "tech_passport" => Ok(Self::TechPassport),
            "vehicle_photo" => Ok(Self::VehiclePhoto),
            "power_of_attorney" => Ok(Self::PowerOfAttorney),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DocumentKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DocumentKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The booked driver's live position, relayed to the booking's viewers as it arrives; dropped when a socket is slow."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverLocation {
    pub booking_id: ::uuid::Uuid,
    #[doc = "Degrees from north; null when the driver's device did not send it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub heading: ::std::option::Option<f64>,
    pub lat: f64,
    pub lng: f64,
    #[doc = "Null when the driver's device did not send it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub speed: ::std::option::Option<f64>,
}
#[doc = "Frame `driver_location` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverLocationFrame {
    pub data: DriverLocation,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A driver changed which instant offers they want: how far to the pickup at most and the lowest price."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverOfferFilterChanged {
    #[doc = "When it was saved; consumers ignore a change older than the one they hold."]
    pub changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Farthest pickup from the driver's last position; null for no limit. Not applied while the position is unknown."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_pickup_km: ::std::option::Option<f64>,
    #[doc = "Lowest price the driver wants offered, whole so'm; null for no limit."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub min_amount: ::std::option::Option<u64>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "A driver left the line: went offline, the line expired, the grace after a disconnect ran out, or the sessions were revoked."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverOffline {
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "From the presence record, which always has it."]
    pub org_id: ::uuid::Uuid,
    pub user_id: ::uuid::Uuid,
}
#[doc = "A driver opened or changed a line and waits for instant offers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverOnline {
    #[doc = "Also takes loads on the way back."]
    pub bidirectional: bool,
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub from_region_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
    pub user_id: ::uuid::Uuid,
    pub vehicle_id: ::uuid::Uuid,
}
#[doc = "A driver's completed-haul count or rating changed. Counters only grow, so a late older event is recognisable."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DriverStatsChanged {
    pub completed_count: u64,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Null until the first rating."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub rating_avg: ::std::option::Option<f64>,
    pub rating_count: u64,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Frame `error` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ErrorFrame {
    pub data: FrameError,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`EtaStop`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EtaStop {
    #[serde(rename = "pickup")]
    Pickup,
    #[serde(rename = "dropoff")]
    Dropoff,
}
impl ::std::fmt::Display for EtaStop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pickup => f.write_str("pickup"),
            Self::Dropoff => f.write_str("dropoff"),
        }
    }
}
impl ::std::str::FromStr for EtaStop {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pickup" => Ok(Self::Pickup),
            "dropoff" => Ok(Self::Dropoff),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EtaStop {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EtaStop {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A frame was refused; the same problem shape as REST errors. Two forms are sent: a first frame that is not `auth`, an unreadable frame and a refused booking subscription send only `code` and `key`; every other refusal sends the full problem body with `type`, `title`, `status` and `code`, plus `detail`, `key` and `params` when set."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FrameError {
    #[doc = "Stable code the app branches on."]
    pub code: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[doc = "Translation key for the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<::std::string::String>,
    #[doc = "Values for the translated message."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<i64>,
    #[doc = "HTTP status phrase, for example `Conflict`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "Problem type URI, `https://api.goway.uz/problems/<code>`."]
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
}
#[doc = "Leave the line. Nothing to send: an empty object, or no `data` at all."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GoOffline(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for GoOffline {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<GoOffline> for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: GoOffline) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>> for GoOffline {
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Frame `go_offline` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GoOfflineFrame {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<GoOffline>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Start receiving instant offers on the line saved with `PUT /v1/carrier/line`, with its vehicle."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GoOnline(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for GoOnline {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<GoOnline> for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: GoOnline) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>> for GoOnline {
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Frame `go_online` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GoOnlineFrame {
    pub data: GoOnline,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`GowayEventSchemas`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GowayEventSchemas(pub ::serde_json::Value);
impl ::std::ops::Deref for GowayEventSchemas {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<GowayEventSchemas> for ::serde_json::Value {
    fn from(value: GowayEventSchemas) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for GowayEventSchemas {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "`IneligibleReason`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IneligibleReason {
    #[serde(rename = "debt")]
    Debt,
    #[serde(rename = "subscription")]
    Subscription,
}
impl ::std::fmt::Display for IneligibleReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debt => f.write_str("debt"),
            Self::Subscription => f.write_str("subscription"),
        }
    }
}
impl ::std::str::FromStr for IneligibleReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "debt" => Ok(Self::Debt),
            "subscription" => Ok(Self::Subscription),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IneligibleReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IneligibleReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`continue`: the haul goes on as it was. `cancel`: cancelled without penalty to either side (a `booking.cancelled` follows). `reprice`: the price was recalculated. `waiting_fee`: a waiting charge was added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IssueResolution {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "reprice")]
    Reprice,
    #[serde(rename = "waiting_fee")]
    WaitingFee,
}
impl ::std::fmt::Display for IssueResolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Continue => f.write_str("continue"),
            Self::Cancel => f.write_str("cancel"),
            Self::Reprice => f.write_str("reprice"),
            Self::WaitingFee => f.write_str("waiting_fee"),
        }
    }
}
impl ::std::str::FromStr for IssueResolution {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "continue" => Ok(Self::Continue),
            "cancel" => Ok(Self::Cancel),
            "reprice" => Ok(Self::Reprice),
            "waiting_fee" => Ok(Self::WaitingFee),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IssueResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IssueResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`IssueSide`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IssueSide {
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "shipper")]
    Shipper,
}
impl ::std::fmt::Display for IssueSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Carrier => f.write_str("carrier"),
            Self::Shipper => f.write_str("shipper"),
        }
    }
}
impl ::std::str::FromStr for IssueSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "carrier" => Ok(Self::Carrier),
            "shipper" => Ok(Self::Shipper),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IssueSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IssueSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "This socket is being closed by the server; sign in again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Kicked {
    #[doc = "`device_removed`, `logout` (closed from another device), the `user.session_revoked` reason, or what a service passed to `/internal/kick`; null when none was given."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[doc = "Frame `kicked` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct KickedFrame {
    pub data: Kicked,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The shipper withdrew an open load."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoadCancelled {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "The shipper organization."]
    pub org_id: ::uuid::Uuid,
}
#[doc = "`LoadChangedField`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LoadChangedField {
    #[serde(rename = "asking_amount")]
    AskingAmount,
    #[serde(rename = "pickup_from")]
    PickupFrom,
    #[serde(rename = "pickup_to")]
    PickupTo,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "plan")]
    Plan,
}
impl ::std::fmt::Display for LoadChangedField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AskingAmount => f.write_str("asking_amount"),
            Self::PickupFrom => f.write_str("pickup_from"),
            Self::PickupTo => f.write_str("pickup_to"),
            Self::Note => f.write_str("note"),
            Self::Plan => f.write_str("plan"),
        }
    }
}
impl ::std::str::FromStr for LoadChangedField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asking_amount" => Ok(Self::AskingAmount),
            "pickup_from" => Ok(Self::PickupFrom),
            "pickup_to" => Ok(Self::PickupTo),
            "note" => Ok(Self::Note),
            "plan" => Ok(Self::Plan),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LoadChangedField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoadChangedField {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An open load passed its expiry unbooked; the sweep closed it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoadExpired {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "The shipper organization."]
    pub org_id: ::uuid::Uuid,
}
#[doc = "A plan is close to its pickup and no driver has booked it yet; the shipper is told they may raise the price. Once per load."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoadPlanUnclaimed {
    #[doc = "Current price, whole so'm; null for a load without an asking price."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub amount: ::std::option::Option<u64>,
    #[doc = "The person who posted the load."]
    pub created_by: ::uuid::Uuid,
    pub currency: LoadPlanUnclaimedCurrency,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "The shipper's organization."]
    pub org_id: ::uuid::Uuid,
    pub pickup_from: ::chrono::DateTime<::chrono::offset::Utc>,
    pub to_region_id: ::uuid::Uuid,
    #[doc = "Drivers who saw the plan so far."]
    pub views: u64,
}
#[doc = "`LoadPlanUnclaimedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LoadPlanUnclaimedCurrency(::std::string::String);
impl ::std::ops::Deref for LoadPlanUnclaimedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LoadPlanUnclaimedCurrency> for ::std::string::String {
    fn from(value: LoadPlanUnclaimedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LoadPlanUnclaimedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LoadPlanUnclaimedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoadPlanUnclaimedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LoadPlanUnclaimedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A load is open on the board: just posted (one event per truck of a multi-truck post), or back on the board after its booking was cancelled, which adds `reposted: true`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoadPosted {
    #[doc = "Whether instant dispatch may offer the load."]
    pub allow_instant: bool,
    #[doc = "The shipper's price, whole so'm (a bare integer; `currency` is UZS today); null when it asks for offers."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub asking_amount: ::std::option::Option<u64>,
    #[doc = "Shipper bonus set aside for this truck, whole so'm (a bare integer; `currency` is UZS today); null when none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub bonus_amount: ::std::option::Option<u64>,
    #[doc = "Cargo type code from the reference catalogue."]
    pub cargo_category: ::std::string::String,
    #[doc = "The person who posted the load; dispatch tells them how the search goes."]
    pub created_by: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: LoadPostedCurrency,
    #[doc = "Driving distance through the stops; null when routing was unavailable at posting."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub distance_km: ::std::option::Option<f64>,
    #[doc = "Last drop-off stop's address; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub dropoff_address: ::std::option::Option<::std::string::String>,
    #[doc = "Driving time for the same route in minutes; null together with the distance."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub duration_min: ::std::option::Option<u64>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Pickup region; null when the address resolved to none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub from_region_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Shared by the sibling loads of one multi-truck post; null for a single truck."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub group_id: ::std::option::Option<::uuid::Uuid>,
    pub is_urgent: bool,
    pub load_id: ::uuid::Uuid,
    #[doc = "Loaders the shipper ordered with the truck."]
    pub loaders_count: u64,
    pub loading_method: LoadingMethod,
    pub needs_refrigeration: bool,
    #[doc = "The shipper organization."]
    pub org_id: ::uuid::Uuid,
    pub payment_method: PaymentMethod,
    #[doc = "First pickup stop's address as the shipper wrote it; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_address: ::std::option::Option<::std::string::String>,
    #[doc = "Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Pickup point latitude; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_lat: ::std::option::Option<f64>,
    #[doc = "Pickup point longitude; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_lng: ::std::option::Option<f64>,
    #[doc = "Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The pickup is scheduled more than goway-loads' instant horizon ahead: drivers book it from the plan list, and dispatch makes no instant offers for it."]
    pub plan: bool,
    pub price_mode: PriceMode,
    #[doc = "Present, and true, only when a cancelled booking put the load back."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reposted: ::std::option::Option<bool>,
    #[doc = "Pickup and drop-off stops together."]
    pub stops_count: u64,
    #[doc = "Market price GoWay suggested, whole so'm (a bare integer; `currency` is UZS today); null when there was none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub suggested_amount: ::std::option::Option<u64>,
    #[doc = "Drop-off region; null when the address resolved to none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub to_region_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Truck class code from the reference list."]
    pub truck_class: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub volume_m3: ::std::option::Option<f64>,
    pub weight_kg: u64,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LoadPostedCurrency(::std::string::String);
impl ::std::ops::Deref for LoadPostedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LoadPostedCurrency> for ::std::string::String {
    fn from(value: LoadPostedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LoadPostedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LoadPostedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoadPostedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LoadPostedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A load still open on the board was edited by its shipper or an operator: the price, the pickup window or the note. Carries the whole load as `load.posted` does, plus what changed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoadUpdated {
    #[doc = "Whether instant dispatch may offer the load."]
    pub allow_instant: bool,
    #[doc = "The shipper's price, whole so'm (a bare integer; `currency` is UZS today); null when it asks for offers."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub asking_amount: ::std::option::Option<u64>,
    #[doc = "Shipper bonus set aside for this truck, whole so'm (a bare integer; `currency` is UZS today); null when none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub bonus_amount: ::std::option::Option<u64>,
    #[doc = "Cargo type code from the reference catalogue."]
    pub cargo_category: ::std::string::String,
    #[doc = "Fields this edit changed. A changed price or pickup window restarts the instant search, so every fitting driver is offered it again; `plan` changes when the new pickup moves the load across the instant horizon."]
    pub changed: ::std::vec::Vec<LoadChangedField>,
    #[doc = "The person who posted the load; dispatch tells them how the search goes."]
    pub created_by: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: LoadUpdatedCurrency,
    #[doc = "Driving distance through the stops; null when routing was unavailable at posting."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub distance_km: ::std::option::Option<f64>,
    #[doc = "Last drop-off stop's address; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub dropoff_address: ::std::option::Option<::std::string::String>,
    #[doc = "Driving time for the same route in minutes; null together with the distance."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub duration_min: ::std::option::Option<u64>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Pickup region; null when the address resolved to none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub from_region_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Shared by the sibling loads of one multi-truck post; null for a single truck."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub group_id: ::std::option::Option<::uuid::Uuid>,
    pub is_urgent: bool,
    pub load_id: ::uuid::Uuid,
    #[doc = "Loaders the shipper ordered with the truck."]
    pub loaders_count: u64,
    pub loading_method: LoadingMethod,
    pub needs_refrigeration: bool,
    #[doc = "The shipper organization."]
    pub org_id: ::uuid::Uuid,
    pub payment_method: PaymentMethod,
    #[doc = "First pickup stop's address as the shipper wrote it; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_address: ::std::option::Option<::std::string::String>,
    #[doc = "Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Pickup point latitude; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_lat: ::std::option::Option<f64>,
    #[doc = "Pickup point longitude; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_lng: ::std::option::Option<f64>,
    #[doc = "Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The pickup is scheduled more than goway-loads' instant horizon ahead: drivers book it from the plan list, and dispatch makes no instant offers for it."]
    pub plan: bool,
    pub price_mode: PriceMode,
    #[doc = "Pickup and drop-off stops together."]
    pub stops_count: u64,
    #[doc = "Market price GoWay suggested, whole so'm (a bare integer; `currency` is UZS today); null when there was none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub suggested_amount: ::std::option::Option<u64>,
    #[doc = "Drop-off region; null when the address resolved to none."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub to_region_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Truck class code from the reference list."]
    pub truck_class: ::std::string::String,
    #[doc = "When the edit was saved; consumers ignore an edit older than the one they hold."]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub volume_m3: ::std::option::Option<f64>,
    pub weight_kg: u64,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LoadUpdatedCurrency(::std::string::String);
impl ::std::ops::Deref for LoadUpdatedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LoadUpdatedCurrency> for ::std::string::String {
    fn from(value: LoadUpdatedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LoadUpdatedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LoadUpdatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoadUpdatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LoadUpdatedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "How the cargo goes in."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LoadingMethod {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "rear")]
    Rear,
}
impl ::std::fmt::Display for LoadingMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Top => f.write_str("top"),
            Self::Rear => f.write_str("rear"),
        }
    }
}
impl ::std::str::FromStr for LoadingMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "top" => Ok(Self::Top),
            "rear" => Ok(Self::Rear),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LoadingMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoadingMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The driver's position. Relayed to booking viewers and sampled as evidence only while a booking is active."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Location {
    #[doc = "Degrees from north, when the device knows it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub heading: ::std::option::Option<f64>,
    pub lat: f64,
    pub lng: f64,
    #[doc = "Speed the device reports, when it knows it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
}
#[doc = "Frame `location` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LocationFrame {
    pub data: Location,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A driver's GPS position during an active booking, sampled every `location_sample_seconds` as evidence of the haul."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LocationSampled {
    pub booking_id: ::uuid::Uuid,
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub lat: f64,
    pub lng: f64,
    #[doc = "When the position was sampled, RFC 3339 UTC with milliseconds."]
    pub recorded_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "A person joined an organization on one side of the market, or a removed membership came back. A person holds at most one live membership per side. The full state is sent, so a consumer upserts it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MembershipCreated {
    #[doc = "When it was created or came back. ISO 8601 with offset."]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Stable id of the membership; every later event about it carries the same one."]
    pub membership_id: ::uuid::Uuid,
    #[doc = "The organization the person works in on this side."]
    pub org_id: ::uuid::Uuid,
    pub org_role: OrgRole,
    #[doc = "The role the side's tokens carry, for example `carrier`, `shipper` or `superadmin`."]
    pub role: ::std::string::String,
    pub side: MembershipSide,
    #[doc = "The person."]
    pub user_id: ::uuid::Uuid,
}
#[doc = "A membership ended: the person closed that side or the account was deleted. Keep it as a tombstone; a later `membership.created` with the same id brings it back."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MembershipRemoved {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Stable id of the membership; every later event about it carries the same one."]
    pub membership_id: ::uuid::Uuid,
    #[doc = "The organization the person works in on this side."]
    pub org_id: ::uuid::Uuid,
    #[doc = "When it ended. ISO 8601 with offset."]
    pub removed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub side: MembershipSide,
    #[doc = "The person."]
    pub user_id: ::uuid::Uuid,
}
#[doc = "A side of the market: `shipper`, `carrier`, or `platform` for GoWay's own staff."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MembershipSide {
    #[serde(rename = "shipper")]
    Shipper,
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "platform")]
    Platform,
}
impl ::std::fmt::Display for MembershipSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shipper => f.write_str("shipper"),
            Self::Carrier => f.write_str("carrier"),
            Self::Platform => f.write_str("platform"),
        }
    }
}
impl ::std::str::FromStr for MembershipSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shipper" => Ok(Self::Shipper),
            "carrier" => Ok(Self::Carrier),
            "platform" => Ok(Self::Platform),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MembershipSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MembershipSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A live membership changed its organization's side, its role or the person's role inside the organization. The full state is sent; keep the newest by `updated_at`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MembershipUpdated {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Stable id of the membership; every later event about it carries the same one."]
    pub membership_id: ::uuid::Uuid,
    #[doc = "The organization the person works in on this side."]
    pub org_id: ::uuid::Uuid,
    pub org_role: OrgRole,
    #[doc = "The role the side's tokens carry, for example `carrier`, `shipper` or `superadmin`."]
    pub role: ::std::string::String,
    pub side: MembershipSide,
    #[doc = "When it changed. ISO 8601 with offset."]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The person."]
    pub user_id: ::uuid::Uuid,
}
#[doc = "Something landed in the user's notification feed. `data` opens the right screen on tap and `unread` is the new badge. Copied from `notification.created`; a field that event lacked or sent empty arrives as null."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Notification {
    #[doc = "Text in the user's language."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub body: ::std::option::Option<::std::string::String>,
    #[doc = "The keys that open the screen, as the push carries them (`booking_id`, `load_id`, `ticket_id`...)."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub data: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Template kind, for example `driver_found` or `announcement`."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub notification_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Title in the user's language."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "Unread feed items, this one included."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub unread: ::std::option::Option<u64>,
}
#[doc = "A message landed in a user's notification feed: from an event, or from an admin announcement (`kind: announcement`). `side` is the side the notice is for, shown only in that side's app; a notice about the person has none and every app shows it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NotificationCreated {
    #[doc = "Text in the user's language."]
    pub body: ::std::string::String,
    #[doc = "What the app needs to open the right screen when the item is tapped: the same keys the push carries, such as `booking_id`, `load_id` or `ticket_id`. Empty for an announcement."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Template kind, for example `driver_found`, `booking_status` or `announcement`."]
    pub kind: ::std::string::String,
    pub notification_id: ::uuid::Uuid,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<MembershipSide>,
    #[doc = "Title in the user's language."]
    pub title: ::std::string::String,
    #[doc = "The user's unread feed items, this one included, for the badge."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub unread: ::std::option::Option<u64>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Frame `notification` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NotificationFrame {
    pub data: Notification,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Every switch, sent whole whenever one of them changed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NotificationPrefs {
    #[doc = "Push for chat messages about a haul and from support."]
    pub chat: bool,
    #[doc = "Promotions and news."]
    pub marketing: bool,
    #[doc = "Push for new offers and dispatch results."]
    pub offers: bool,
    #[doc = "Push for booking status changes."]
    pub orders: bool,
    #[doc = "SMS fallback for critical events."]
    pub sms: bool,
}
#[doc = "`amount`: the sum is disputed. `damage`: the cargo arrived damaged. `shortage`: part of it is missing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObjectionKind {
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "damage")]
    Damage,
    #[serde(rename = "shortage")]
    Shortage,
}
impl ::std::fmt::Display for ObjectionKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Amount => f.write_str("amount"),
            Self::Damage => f.write_str("damage"),
            Self::Shortage => f.write_str("shortage"),
        }
    }
}
impl ::std::str::FromStr for ObjectionKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "amount" => Ok(Self::Amount),
            "damage" => Ok(Self::Damage),
            "shortage" => Ok(Self::Shortage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ObjectionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ObjectionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An offer was accepted and became a booking; published together with `booking.created`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferAccepted {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "The driver who made the offer; the code allows null, an offer always has one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub driver_user_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub offer_id: ::uuid::Uuid,
    pub shipper_org_id: ::uuid::Uuid,
}
#[doc = "`OfferAction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OfferAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "decline")]
    Decline,
}
impl ::std::fmt::Display for OfferAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accept => f.write_str("accept"),
            Self::Decline => f.write_str("decline"),
        }
    }
}
impl ::std::str::FromStr for OfferAction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accept" => Ok(Self::Accept),
            "decline" => Ok(Self::Decline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OfferAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferAction {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A carrier asked for a load, or either side countered the other's price."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferCreated {
    #[doc = "Offered price, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: u64,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: OfferCreatedCurrency,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: OfferKind,
    pub load_id: ::uuid::Uuid,
    pub offer_id: ::uuid::Uuid,
    pub shipper_org_id: ::uuid::Uuid,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct OfferCreatedCurrency(::std::string::String);
impl ::std::ops::Deref for OfferCreatedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OfferCreatedCurrency> for ::std::string::String {
    fn from(value: OfferCreatedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OfferCreatedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OfferCreatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferCreatedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OfferCreatedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Who made the offer and how."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OfferKind {
    #[serde(rename = "carrier_request")]
    CarrierRequest,
    #[serde(rename = "carrier_counter")]
    CarrierCounter,
    #[serde(rename = "shipper_counter")]
    ShipperCounter,
}
impl ::std::fmt::Display for OfferKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CarrierRequest => f.write_str("carrier_request"),
            Self::CarrierCounter => f.write_str("carrier_counter"),
            Self::ShipperCounter => f.write_str("shipper_counter"),
        }
    }
}
impl ::std::str::FromStr for OfferKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "carrier_request" => Ok(Self::CarrierRequest),
            "carrier_counter" => Ok(Self::CarrierCounter),
            "shipper_counter" => Ok(Self::ShipperCounter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OfferKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OfferOutcome`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OfferOutcome {
    #[serde(rename = "won")]
    Won,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "withdrawn")]
    Withdrawn,
}
impl ::std::fmt::Display for OfferOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Won => f.write_str("won"),
            Self::Lost => f.write_str("lost"),
            Self::Declined => f.write_str("declined"),
            Self::Expired => f.write_str("expired"),
            Self::Withdrawn => f.write_str("withdrawn"),
        }
    }
}
impl ::std::str::FromStr for OfferOutcome {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "won" => Ok(Self::Won),
            "lost" => Ok(Self::Lost),
            "declined" => Ok(Self::Declined),
            "expired" => Ok(Self::Expired),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OfferOutcome {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferOutcome {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An instant load offered to this driver; answer before `deadline_at`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferProposed {
    #[doc = "Price in minor units of `currency`."]
    pub amount: u64,
    #[doc = "Cargo type code from the reference catalogue."]
    pub cargo_category: ::std::string::String,
    pub currency: OfferProposedCurrency,
    #[doc = "Answer deadline, unix milliseconds in server time."]
    pub deadline_at: i64,
    #[doc = "Driving distance through the stops; null when routing was unavailable at posting."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub distance_km: ::std::option::Option<f64>,
    #[doc = "Last drop-off stop's address; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub dropoff_address: ::std::option::Option<::std::string::String>,
    #[doc = "Driving time for the same route in minutes; null together with the distance."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub duration_min: ::std::option::Option<u64>,
    pub from_region_id: ::uuid::Uuid,
    pub is_urgent: bool,
    pub load_id: ::uuid::Uuid,
    #[doc = "Loaders the shipper ordered with the truck."]
    pub loaders_count: u64,
    pub loading_method: LoadingMethod,
    pub needs_refrigeration: bool,
    pub payment_method: PaymentMethod,
    #[doc = "First pickup stop's address as the shipper wrote it; null only when the load has no stops."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_address: ::std::option::Option<::std::string::String>,
    #[doc = "Straight-line distance from the driver's last position to the pickup; null while the position is unknown."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_distance_km: ::std::option::Option<f64>,
    #[doc = "Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_from: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub pickup_to: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub server_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Pickup and drop-off stops together."]
    pub stops_count: u64,
    pub to_region_id: ::uuid::Uuid,
    pub truck_class: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub volume_m3: ::std::option::Option<f64>,
    pub weight_kg: u64,
}
#[doc = "`OfferProposedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct OfferProposedCurrency(::std::string::String);
impl ::std::ops::Deref for OfferProposedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OfferProposedCurrency> for ::std::string::String {
    fn from(value: OfferProposedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OfferProposedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OfferProposedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferProposedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OfferProposedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Frame `offer_proposed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferProposedFrame {
    pub data: OfferProposed,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`accepted`: the driver took it. `lost`: they accepted but another driver was first. `declined`: they skipped it. `expired`: no answer in time. `withdrawn`: the load was taken by someone else, cancelled or edited before they answered."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OfferResolution {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "withdrawn")]
    Withdrawn,
}
impl ::std::fmt::Display for OfferResolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accepted => f.write_str("accepted"),
            Self::Lost => f.write_str("lost"),
            Self::Declined => f.write_str("declined"),
            Self::Expired => f.write_str("expired"),
            Self::Withdrawn => f.write_str("withdrawn"),
        }
    }
}
impl ::std::str::FromStr for OfferResolution {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "lost" => Ok(Self::Lost),
            "declined" => Ok(Self::Declined),
            "expired" => Ok(Self::Expired),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OfferResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfferResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "One driver's instant offer ended. One event per driver per offer; it feeds the driver's acceptance rate."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferResolved {
    pub driver_user_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub offered_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The driver's organization."]
    pub org_id: ::uuid::Uuid,
    pub outcome: OfferResolution,
    pub resolved_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "The driver's answer to an instant offer."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferResponse {
    pub action: OfferAction,
    pub load_id: ::uuid::Uuid,
}
#[doc = "Frame `offer_response` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferResponseFrame {
    pub data: OfferResponse,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "How an offer ended for this driver."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferResult {
    #[doc = "Sent only when the outcome is `won`; null when goway-loads' answer could not be read."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub booking_id: ::std::option::Option<::uuid::Uuid>,
    pub load_id: ::uuid::Uuid,
    pub outcome: OfferOutcome,
}
#[doc = "Frame `offer_result` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfferResultFrame {
    pub data: OfferResult,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The wallet stopped being eligible; no offers until it is settled."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OffersBlocked {
    pub reason: OffersBlockedReason,
}
#[doc = "Frame `offers_blocked` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OffersBlockedFrame {
    pub data: OffersBlocked,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`OffersBlockedReason`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OffersBlockedReason {
    #[serde(rename = "BALANCE_INSUFFICIENT")]
    BalanceInsufficient,
    #[serde(rename = "SUBSCRIPTION_REQUIRED")]
    SubscriptionRequired,
}
impl ::std::fmt::Display for OffersBlockedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BalanceInsufficient => f.write_str("BALANCE_INSUFFICIENT"),
            Self::SubscriptionRequired => f.write_str("SUBSCRIPTION_REQUIRED"),
        }
    }
}
impl ::std::str::FromStr for OffersBlockedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "BALANCE_INSUFFICIENT" => Ok(Self::BalanceInsufficient),
            "SUBSCRIPTION_REQUIRED" => Ok(Self::SubscriptionRequired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OffersBlockedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OffersBlockedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The wallet is eligible again after being blocked; nothing in it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OffersUnblocked(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for OffersUnblocked {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<OffersUnblocked> for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: OffersUnblocked) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>> for OffersUnblocked {
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Frame `offers_unblocked` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OffersUnblockedFrame {
    pub data: OffersUnblocked,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The driver left the line: after `go_offline`, or taken off it with a `reason`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct Offline {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<OfflineReason>,
}
#[doc = "Frame `offline` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OfflineFrame {
    pub data: Offline,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Why the server took the driver off the line; absent when the driver left by themselves. `vehicle_unavailable`: the line's truck was removed, left the fleet, or the line has none."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OfflineReason {
    #[serde(rename = "documents_expired")]
    DocumentsExpired,
    #[serde(rename = "verification_rejected")]
    VerificationRejected,
    #[serde(rename = "vehicle_unavailable")]
    VehicleUnavailable,
}
impl ::std::fmt::Display for OfflineReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DocumentsExpired => f.write_str("documents_expired"),
            Self::VerificationRejected => f.write_str("verification_rejected"),
            Self::VehicleUnavailable => f.write_str("vehicle_unavailable"),
        }
    }
}
impl ::std::str::FromStr for OfflineReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "documents_expired" => Ok(Self::DocumentsExpired),
            "verification_rejected" => Ok(Self::VerificationRejected),
            "vehicle_unavailable" => Ok(Self::VehicleUnavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OfflineReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OfflineReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The driver is on their line: after `go_online`, and again whenever the line is changed while online."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Online {
    #[doc = "The way back is taken too."]
    pub bidirectional: bool,
    #[doc = "Where the line starts."]
    pub from_region_id: ::uuid::Uuid,
    #[doc = "Set when the wallet blocks offers, with the reason; null otherwise."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub offers_blocked: ::std::option::Option<OffersBlockedReason>,
    #[doc = "Place in the outbound line's queue, from 1."]
    pub queue_position: ::std::num::NonZeroU64,
    #[doc = "Where the line ends."]
    pub to_region_id: ::uuid::Uuid,
    #[doc = "The truck the offers are matched against."]
    pub vehicle_id: ::uuid::Uuid,
}
#[doc = "Frame `online` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OnlineFrame {
    pub data: Online,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "What the person is inside the organization: `owner` for the account that opened it, `member` otherwise."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrgRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
}
impl ::std::fmt::Display for OrgRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Owner => f.write_str("owner"),
            Self::Member => f.write_str("member"),
        }
    }
}
impl ::std::str::FromStr for OrgRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "owner" => Ok(Self::Owner),
            "member" => Ok(Self::Member),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrgRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrgRole {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An organization's details changed. Only what changed is sent, besides `org_id` and `updated_at`: a role switch sends `kind` and `account_type`; the organization screen any of `name`, `legal_name`, `inn`, `contact_phone`, `address`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrgUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub account_type: ::std::option::Option<AccountType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<::std::string::String>,
    #[doc = "E.164 number, `+998…`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub contact_phone: ::std::option::Option<::std::string::String>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Tax id (STIR/INN), 9 digits."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub inn: ::std::option::Option<::std::string::String>,
    #[doc = "Market side: `shipper` or `carrier`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::std::string::String>,
    #[doc = "Registered company name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legal_name: ::std::option::Option<::std::string::String>,
    #[doc = "Display name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub org_id: ::uuid::Uuid,
    #[doc = "When the change was made. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "`PassedWaypoint`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PassedWaypoint {
    pub passed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub region_id: ::uuid::Uuid,
}
#[doc = "`PaymentMethod`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaymentMethod {
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "mixed")]
    Mixed,
}
impl ::std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cash => f.write_str("cash"),
            Self::Transfer => f.write_str("transfer"),
            Self::Card => f.write_str("card"),
            Self::Mixed => f.write_str("mixed"),
        }
    }
}
impl ::std::str::FromStr for PaymentMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cash" => Ok(Self::Cash),
            "transfer" => Ok(Self::Transfer),
            "card" => Ok(Self::Card),
            "mixed" => Ok(Self::Mixed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaymentMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaymentMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Nothing: an empty object, or no `data` at all."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Ping(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for Ping {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Ping> for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: Ping) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>> for Ping {
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Frame `ping` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingFrame {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<Ping>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`reminder`: the pickup is `plan_reminder_minutes` away. `departure`: `plan_departure_minutes` away, the app opens navigation. `overdue`: the pickup time came and the haul has not set off; operators are told too."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlanStage {
    #[serde(rename = "reminder")]
    Reminder,
    #[serde(rename = "departure")]
    Departure,
    #[serde(rename = "overdue")]
    Overdue,
}
impl ::std::fmt::Display for PlanStage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reminder => f.write_str("reminder"),
            Self::Departure => f.write_str("departure"),
            Self::Overdue => f.write_str("overdue"),
        }
    }
}
impl ::std::str::FromStr for PlanStage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reminder" => Ok(Self::Reminder),
            "departure" => Ok(Self::Departure),
            "overdue" => Ok(Self::Overdue),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlanStage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlanStage {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The answer to `ping`; nothing in it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Pong(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for Pong {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Pong> for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: Pong) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>> for Pong {
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Frame `pong` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PongFrame {
    pub data: Pong,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`PriceMode`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PriceMode {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "negotiable")]
    Negotiable,
}
impl ::std::fmt::Display for PriceMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fixed => f.write_str("fixed"),
            Self::Negotiable => f.write_str("negotiable"),
        }
    }
}
impl ::std::str::FromStr for PriceMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fixed" => Ok(Self::Fixed),
            "negotiable" => Ok(Self::Negotiable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PriceMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PriceMode {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "What `push_token` holds: `token`, an FCM registration token (sent as `message.token`), or `fid`, a Firebase Installation ID (sent as `message.fid`). Absent on events from before it was kept: `token`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PushKind {
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "fid")]
    Fid,
}
impl ::std::fmt::Display for PushKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Token => f.write_str("token"),
            Self::Fid => f.write_str("fid"),
        }
    }
}
impl ::std::str::FromStr for PushKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "token" => Ok(Self::Token),
            "fid" => Ok(Self::Fid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PushKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PushKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The shipper rated a completed haul; sent once per haul."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RatingCreated {
    pub booking_id: ::uuid::Uuid,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "The rated driver; the code allows null, a booking always has one."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub driver_user_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub shipper_org_id: ::uuid::Uuid,
    #[doc = "The person who posted the load: the one rated when `target` is `shipper`."]
    pub shipper_user_id: ::uuid::Uuid,
    #[doc = "1 to 5."]
    pub stars: ::std::num::NonZeroU64,
    #[doc = "Codes from the rating tags catalogue, for example `on_time`, `cargo_ready`."]
    pub tags: ::std::vec::Vec<::std::string::String>,
    pub target: RatingTarget,
}
#[doc = "Who was rated: the shipper rates the driver, the driver rates the shipper."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RatingTarget {
    #[serde(rename = "driver")]
    Driver,
    #[serde(rename = "shipper")]
    Shipper,
}
impl ::std::fmt::Display for RatingTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Driver => f.write_str("driver"),
            Self::Shipper => f.write_str("shipper"),
        }
    }
}
impl ::std::str::FromStr for RatingTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "driver" => Ok(Self::Driver),
            "shipper" => Ok(Self::Shipper),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RatingTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RatingTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An admin opened or closed a region for the marketplace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RegionChanged {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub is_active: bool,
    pub region_id: ::uuid::Uuid,
}
#[doc = "The decision."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReviewDecision {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}
impl ::std::fmt::Display for ReviewDecision {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Approved => f.write_str("approved"),
            Self::Rejected => f.write_str("rejected"),
        }
    }
}
impl ::std::str::FromStr for ReviewDecision {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "approved" => Ok(Self::Approved),
            "rejected" => Ok(Self::Rejected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReviewDecision {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReviewDecision {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "To the shipper: no driver took the load within the search window. The search goes on; the app offers to raise the price, move the pickup to a plan or cancel."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchExhausted {
    #[doc = "Current price, whole so'm."]
    pub amount: u64,
    pub currency: SearchExhaustedCurrency,
    pub load_id: ::uuid::Uuid,
    pub seen: u64,
    #[doc = "The raised price GoWay suggests."]
    pub suggested_amount: u64,
}
#[doc = "`SearchExhaustedCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchExhaustedCurrency(::std::string::String);
impl ::std::ops::Deref for SearchExhaustedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchExhaustedCurrency> for ::std::string::String {
    fn from(value: SearchExhaustedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchExhaustedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchExhaustedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchExhaustedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchExhaustedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Frame `search_exhausted` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchExhaustedFrame {
    pub data: SearchExhausted,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "To the shipper while their instant load is being offered: how many drivers have seen it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchProgress {
    pub load_id: ::uuid::Uuid,
    #[doc = "Drivers the load was offered to so far."]
    pub seen: u64,
    #[doc = "When the search began; the app counts the waiting time from it."]
    pub started_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "Frame `search_progress` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SearchProgressFrame {
    pub data: SearchProgress,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Any frame goway-realtime may send; `type` tells which."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ServerFrame {
    #[doc = "Frame `connected` goway-realtime sends to an app."]
    #[serde(rename = "connected")]
    Connected {
        data: Connected,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `pong` goway-realtime sends to an app."]
    #[serde(rename = "pong")]
    Pong {
        data: Pong,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `online` goway-realtime sends to an app."]
    #[serde(rename = "online")]
    Online {
        data: Online,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `offline` goway-realtime sends to an app."]
    #[serde(rename = "offline")]
    Offline {
        data: Offline,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `offer_proposed` goway-realtime sends to an app."]
    #[serde(rename = "offer_proposed")]
    OfferProposed {
        data: OfferProposed,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `offer_result` goway-realtime sends to an app."]
    #[serde(rename = "offer_result")]
    OfferResult {
        data: OfferResult,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `subscribed` goway-realtime sends to an app."]
    #[serde(rename = "subscribed")]
    Subscribed {
        data: Subscribed,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_created` goway-realtime sends to an app."]
    #[serde(rename = "booking_created")]
    BookingCreated {
        data: BookingCreatedData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_status` goway-realtime sends to an app."]
    #[serde(rename = "booking_status")]
    BookingStatus {
        data: BookingStatusData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_eta` goway-realtime sends to an app."]
    #[serde(rename = "booking_eta")]
    BookingEta {
        data: BookingEtaData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_completed` goway-realtime sends to an app."]
    #[serde(rename = "booking_completed")]
    BookingCompleted {
        data: BookingCompletedData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_cancelled` goway-realtime sends to an app."]
    #[serde(rename = "booking_cancelled")]
    BookingCancelled {
        data: BookingCancelledData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `driver_location` goway-realtime sends to an app."]
    #[serde(rename = "driver_location")]
    DriverLocation {
        data: DriverLocation,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `offers_blocked` goway-realtime sends to an app."]
    #[serde(rename = "offers_blocked")]
    OffersBlocked {
        data: OffersBlocked,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `offers_unblocked` goway-realtime sends to an app."]
    #[serde(rename = "offers_unblocked")]
    OffersUnblocked {
        data: OffersUnblocked,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `notification` goway-realtime sends to an app."]
    #[serde(rename = "notification")]
    Notification {
        data: Notification,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `session_started` goway-realtime sends to an app."]
    #[serde(rename = "session_started")]
    SessionStarted {
        data: SessionStarted,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `kicked` goway-realtime sends to an app."]
    #[serde(rename = "kicked")]
    Kicked {
        data: Kicked,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `error` goway-realtime sends to an app."]
    #[serde(rename = "error")]
    Error {
        data: FrameError,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `verification_changed` goway-realtime sends to an app."]
    #[serde(rename = "verification_changed")]
    VerificationChanged {
        data: VerificationChanged,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `search_progress` goway-realtime sends to an app."]
    #[serde(rename = "search_progress")]
    SearchProgress {
        data: SearchProgress,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `search_exhausted` goway-realtime sends to an app."]
    #[serde(rename = "search_exhausted")]
    SearchExhausted {
        data: SearchExhausted,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `board_subscribed` goway-realtime sends to an app."]
    #[serde(rename = "board_subscribed")]
    BoardSubscribed {
        data: BoardSubscribed,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `board_load_added` goway-realtime sends to an app."]
    #[serde(rename = "board_load_added")]
    BoardLoadAdded {
        data: BoardLoadAdded,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `board_load_updated` goway-realtime sends to an app."]
    #[serde(rename = "board_load_updated")]
    BoardLoadUpdated {
        data: BoardLoadUpdated,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `board_load_removed` goway-realtime sends to an app."]
    #[serde(rename = "board_load_removed")]
    BoardLoadRemoved {
        data: BoardLoadRemoved,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `booking_updated` goway-realtime sends to an app."]
    #[serde(rename = "booking_updated")]
    BookingUpdated {
        data: BookingUpdated,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `chat_message` goway-realtime sends to an app."]
    #[serde(rename = "chat_message")]
    ChatMessage {
        data: ChatMessageData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `chat_read` goway-realtime sends to an app."]
    #[serde(rename = "chat_read")]
    ChatRead {
        data: ChatReadData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `chat_typing` goway-realtime sends to an app."]
    #[serde(rename = "chat_typing")]
    ChatTyping {
        data: ChatTyping,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
    #[doc = "Frame `support_message` goway-realtime sends to an app."]
    #[serde(rename = "support_message")]
    SupportMessage {
        data: SupportMessageData,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
}
#[doc = "Why every session of the user was closed. An operator's own note on a block stays with staff."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SessionRevokedReason {
    #[serde(rename = "password_change")]
    PasswordChange,
    #[serde(rename = "password_reset")]
    PasswordReset,
    #[serde(rename = "phone_change")]
    PhoneChange,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "blocked")]
    Blocked,
}
impl ::std::fmt::Display for SessionRevokedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PasswordChange => f.write_str("password_change"),
            Self::PasswordReset => f.write_str("password_reset"),
            Self::PhoneChange => f.write_str("phone_change"),
            Self::Deleted => f.write_str("deleted"),
            Self::Blocked => f.write_str("blocked"),
        }
    }
}
impl ::std::str::FromStr for SessionRevokedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "password_change" => Ok(Self::PasswordChange),
            "password_reset" => Ok(Self::PasswordReset),
            "phone_change" => Ok(Self::PhoneChange),
            "deleted" => Ok(Self::Deleted),
            "blocked" => Ok(Self::Blocked),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionRevokedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionRevokedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Somebody signed in to this account; ignore it when `installation_id` is this app's own. Copied from `user.login`; a field that event lacked or sent empty arrives as null."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SessionStarted {
    #[doc = "When the sign-in happened, as `user.login` wrote it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub device_id: ::std::option::Option<::uuid::Uuid>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub installation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Device model."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub model: ::std::option::Option<::std::string::String>,
    pub new_device: bool,
    #[doc = "Device platform."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<DevicePlatform>,
}
#[doc = "Frame `session_started` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SessionStartedFrame {
    pub data: SessionStarted,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Why the sessions were closed; only a sign-out from another device today."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SessionsClosedReason {
    #[serde(rename = "logout")]
    Logout,
}
impl ::std::fmt::Display for SessionsClosedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logout => f.write_str("logout"),
        }
    }
}
impl ::std::str::FromStr for SessionsClosedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logout" => Ok(Self::Logout),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionsClosedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionsClosedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An admin changed or reset runtime settings of one service. Published by goway-lib for goway-auth, goway-loads, goway-billing and goway-notify, and by goway-realtime itself, with the same fields."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SettingsChanged {
    #[doc = "The admin who changed them; null when not known."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub by: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event: the outbox row id from a Python service, a random UUID from goway-realtime. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Setting keys that changed."]
    pub keys: ::std::vec::Vec<::std::string::String>,
    #[doc = "The service whose settings changed, for example `goway-loads`."]
    pub service: ::std::string::String,
}
#[doc = "A shipper's standing moved: a driver rated them. Replicas show it on the shipper card."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ShipperStatsChanged {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Average over the rating window; null before the first rating."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub rating_avg: ::std::option::Option<f64>,
    #[doc = "All ratings ever received; only grows."]
    pub rating_count: u64,
    pub user_id: ::uuid::Uuid,
}
#[doc = "A new version always waits for review."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubmittedDocumentStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl ::std::fmt::Display for SubmittedDocumentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
        }
    }
}
impl ::std::str::FromStr for SubmittedDocumentStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubmittedDocumentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubmittedDocumentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Hear when loads on a lane appear, change or leave the board, for a live plan list. One direction per frame; a driver with a way back subscribes to both."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeBoard {
    pub from_region_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `subscribe_board` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeBoardFrame {
    pub data: SubscribeBoard,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Watch a booking of the caller's organization: status, arrival window and the driver's position. The driver and the shipper who posted the load get every frame but the position without subscribing; subscribe for the position, or as another member of either organization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeBooking {
    pub booking_id: ::uuid::Uuid,
}
#[doc = "Frame `subscribe_booking` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeBookingFrame {
    pub data: SubscribeBooking,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`subscribe_booking` was accepted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Subscribed {
    pub booking_id: ::uuid::Uuid,
}
#[doc = "Frame `subscribed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribedFrame {
    pub data: Subscribed,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`SupportAuthor`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportAuthor {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "operator")]
    Operator,
}
impl ::std::fmt::Display for SupportAuthor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Operator => f.write_str("operator"),
        }
    }
}
impl ::std::str::FromStr for SupportAuthor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "operator" => Ok(Self::Operator),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SupportAuthor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupportAuthor {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A message was added to a support conversation, by the person who opened it or by an operator."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupportMessageCreated {
    pub author: SupportAuthor,
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub message_id: ::uuid::Uuid,
    #[doc = "The ticket's short number."]
    pub number: ::std::string::String,
    pub text: SupportMessageCreatedText,
    pub ticket_id: ::uuid::Uuid,
    #[doc = "The person the conversation belongs to."]
    pub user_id: ::uuid::Uuid,
}
#[doc = "`SupportMessageCreatedText`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SupportMessageCreatedText(::std::string::String);
impl ::std::ops::Deref for SupportMessageCreatedText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SupportMessageCreatedText> for ::std::string::String {
    fn from(value: SupportMessageCreatedText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SupportMessageCreatedText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SupportMessageCreatedText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupportMessageCreatedText {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SupportMessageCreatedText {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A message in a support conversation, to the person who opened it. The internal event's ids for organizations, recipients and the event itself never reach an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupportMessageData {
    pub author: SupportAuthor,
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub message_id: ::uuid::Uuid,
    #[doc = "The ticket's short number."]
    pub number: ::std::string::String,
    pub text: SupportMessageDataText,
    pub ticket_id: ::uuid::Uuid,
}
#[doc = "`SupportMessageDataText`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SupportMessageDataText(::std::string::String);
impl ::std::ops::Deref for SupportMessageDataText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SupportMessageDataText> for ::std::string::String {
    fn from(value: SupportMessageDataText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SupportMessageDataText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SupportMessageDataText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupportMessageDataText {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SupportMessageDataText {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Frame `support_message` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupportMessageFrame {
    pub data: SupportMessageData,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "A user opened a support ticket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupportTicketCreated {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Reference the user sees: `T-` and six characters, for example `T-7KQ2MZ`."]
    pub number: ::std::string::String,
    #[doc = "The haul the ticket is about; null when none was named."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub order_id: ::std::option::Option<::uuid::Uuid>,
    pub ticket_id: ::uuid::Uuid,
    pub topic: SupportTopic,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Support answered or closed a ticket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupportTicketReplied {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Reference the user sees: `T-` and six characters, for example `T-7KQ2MZ`."]
    pub number: ::std::string::String,
    pub status: TicketReplyStatus,
    pub ticket_id: ::uuid::Uuid,
    #[doc = "Who opened the ticket."]
    pub user_id: ::uuid::Uuid,
}
#[doc = "What the ticket is about."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportTopic {
    #[serde(rename = "suggestion")]
    Suggestion,
    #[serde(rename = "complaint")]
    Complaint,
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "driver")]
    Driver,
    #[serde(rename = "app")]
    App,
    #[serde(rename = "documents")]
    Documents,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for SupportTopic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Suggestion => f.write_str("suggestion"),
            Self::Complaint => f.write_str("complaint"),
            Self::Bug => f.write_str("bug"),
            Self::Payment => f.write_str("payment"),
            Self::Order => f.write_str("order"),
            Self::Driver => f.write_str("driver"),
            Self::App => f.write_str("app"),
            Self::Documents => f.write_str("documents"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for SupportTopic {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "suggestion" => Ok(Self::Suggestion),
            "complaint" => Ok(Self::Complaint),
            "bug" => Ok(Self::Bug),
            "payment" => Ok(Self::Payment),
            "order" => Ok(Self::Order),
            "driver" => Ok(Self::Driver),
            "app" => Ok(Self::App),
            "documents" => Ok(Self::Documents),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SupportTopic {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupportTopic {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "App theme."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for Theme {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Light => f.write_str("light"),
            Self::Dark => f.write_str("dark"),
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for Theme {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Theme {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Theme {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The ticket's status after the reply."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TicketReplyStatus {
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "closed")]
    Closed,
}
impl ::std::fmt::Display for TicketReplyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Answered => f.write_str("answered"),
            Self::Closed => f.write_str("closed"),
        }
    }
}
impl ::std::str::FromStr for TicketReplyStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "answered" => Ok(Self::Answered),
            "closed" => Ok(Self::Closed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TicketReplyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TicketReplyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The caller is typing in a haul's chat; sent at most every few seconds while typing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Typing {
    pub booking_id: ::uuid::Uuid,
}
#[doc = "Frame `typing` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TypingFrame {
    pub data: Typing,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Stop hearing about a lane."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeBoard {
    pub from_region_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
}
#[doc = "Frame `unsubscribe_board` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeBoardFrame {
    pub data: UnsubscribeBoard,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "Stop watching a booking."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeBooking {
    pub booking_id: ::uuid::Uuid,
}
#[doc = "Frame `unsubscribe_booking` an app sends to goway-realtime."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeBookingFrame {
    pub data: UnsubscribeBooking,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "An operator blocked the account; the app shows the reason on its blocked screen."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserBlocked {
    pub blocked_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "What the operator told the user, in their words."]
    pub reason: UserBlockedReason,
    pub user_id: ::uuid::Uuid,
}
#[doc = "What the operator told the user, in their words."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UserBlockedReason(::std::string::String);
impl ::std::ops::Deref for UserBlockedReason {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UserBlockedReason> for ::std::string::String {
    fn from(value: UserBlockedReason) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UserBlockedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UserBlockedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserBlockedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UserBlockedReason {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A person's account was opened (self-signup, admin create) or restored (with `updated_at`). The account is the person; the organization and side they work in come in `membership.created`, sent with it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserCreated {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Admin create only; null when the admin gave none."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[doc = "App language code: uz, uzc, ru, en, kaa or zh."]
    pub language: ::std::string::String,
    #[doc = "Admin create only; null when the admin gave none."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[doc = "E.164 number, `+998…`. A restore sends the stored value, which is `del:…` for an account its owner deleted."]
    pub phone: ::std::string::String,
    #[doc = "Admin restore only: when the account came back. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "An account was deleted: by its owner, without `updated_at`, or by an admin, soft or hard, with `updated_at`. Each side the person held ends with its own `membership.removed`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserDeleted {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Admin deletes only: when the account was deleted. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Somebody signed in to the account: by code, password, password reset or phone change. The device fields are null when the app sent no device."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserLogin {
    #[doc = "When the sign-in happened. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Device row the session belongs to."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub device_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "The app install's own id."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub installation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Client address."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub ip: ::std::option::Option<::std::string::String>,
    #[doc = "Device model as the app reported it."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "True the first time this install signs in, or when a removed device comes back."]
    pub new_device: bool,
    #[doc = "Device platform."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<DevicePlatform>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Every session of the account was closed; each app must sign in again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserSessionRevoked {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub reason: SessionRevokedReason,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Some of the account's sessions were closed from another device; those installs must sign in again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserSessionsClosed {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Installs whose sessions were closed; never empty."]
    pub installation_ids: ::std::vec::Vec<::std::string::String>,
    pub reason: SessionsClosedReason,
    pub user_id: ::uuid::Uuid,
}
#[doc = "An operator lifted the block; the user may sign in again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserUnblocked {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub unblocked_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "Something about an account changed. Only what changed is sent, besides `user_id` and `updated_at`, and four paths send four field sets: a confirmed phone change sends `phone`; the account settings screen any of `language`, `currency`, `theme`, `notification_prefs`; an admin edit any of `phone`, `language`, `first_name`, `last_name`; a profile edit `first_name`, `last_name`, `email` and `avatar_url` together, each of them possibly null. A change of side or role is a `membership.*` event."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserUpdated {
    #[doc = "Profile edit only. Public avatar address; null when there is none or it would expire."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<DisplayCurrency>,
    #[doc = "Profile edit only; null when the profile has no email."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "Null only from a profile edit, when the profile has no first name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[doc = "App language code: uz, uzc, ru, en, kaa or zh."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "Null only from a profile edit, when the profile has no last name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notification_prefs: ::std::option::Option<NotificationPrefs>,
    #[doc = "New E.164 number, `+998…`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phone: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub theme: ::std::option::Option<Theme>,
    #[doc = "When the change was made. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub user_id: ::uuid::Uuid,
}
#[doc = "`VehicleColor`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VehicleColor {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "brown")]
    Brown,
    #[serde(rename = "beige")]
    Beige,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for VehicleColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::White => f.write_str("white"),
            Self::Black => f.write_str("black"),
            Self::Silver => f.write_str("silver"),
            Self::Gray => f.write_str("gray"),
            Self::Red => f.write_str("red"),
            Self::Blue => f.write_str("blue"),
            Self::Green => f.write_str("green"),
            Self::Yellow => f.write_str("yellow"),
            Self::Orange => f.write_str("orange"),
            Self::Brown => f.write_str("brown"),
            Self::Beige => f.write_str("beige"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for VehicleColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            "silver" => Ok(Self::Silver),
            "gray" => Ok(Self::Gray),
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "orange" => Ok(Self::Orange),
            "brown" => Ok(Self::Brown),
            "beige" => Ok(Self::Beige),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VehicleColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VehicleColor {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the vehicle is loaded."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VehicleLoadingMethod {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "rear")]
    Rear,
    #[serde(rename = "both")]
    Both,
}
impl ::std::fmt::Display for VehicleLoadingMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Top => f.write_str("top"),
            Self::Rear => f.write_str("rear"),
            Self::Both => f.write_str("both"),
        }
    }
}
impl ::std::str::FromStr for VehicleLoadingMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "top" => Ok(Self::Top),
            "rear" => Ok(Self::Rear),
            "both" => Ok(Self::Both),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VehicleLoadingMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VehicleLoadingMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A fleet vehicle was removed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VehicleRemoved {
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    pub vehicle_id: ::uuid::Uuid,
}
#[doc = "A fleet vehicle was added or changed; the whole vehicle is sent every time."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VehicleUpserted {
    #[doc = "Make, as the driver entered it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub brand: ::std::option::Option<VehicleUpsertedBrand>,
    #[doc = "Capability codes: rear_ramp, open_top, side_curtain, tail_lift, refrigerated."]
    pub capabilities: ::std::vec::Vec<::std::string::String>,
    #[doc = "Payload in kilograms; null when not given."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub capacity_kg: ::std::option::Option<u64>,
    #[doc = "Body colour code; apps name it in their language."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<VehicleColor>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub loading_method: VehicleLoadingMethod,
    #[doc = "Model, as the driver entered it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<VehicleUpsertedModel>,
    pub org_id: ::uuid::Uuid,
    #[doc = "Normalized plate: no spaces, upper case."]
    pub plate_number: ::std::string::String,
    #[doc = "Truck class code from the reference list."]
    pub truck_class: ::std::string::String,
    #[doc = "When the vehicle was saved. ISO 8601 with offset, as Python's `isoformat()` writes it."]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub vehicle_id: ::uuid::Uuid,
    #[doc = "Body volume in m³."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volume_m3: ::std::option::Option<f64>,
    #[doc = "Year of manufacture."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub year: ::std::option::Option<i64>,
}
#[doc = "Make, as the driver entered it."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VehicleUpsertedBrand(::std::string::String);
impl ::std::ops::Deref for VehicleUpsertedBrand {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VehicleUpsertedBrand> for ::std::string::String {
    fn from(value: VehicleUpsertedBrand) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VehicleUpsertedBrand {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 40usize {
            return Err("longer than 40 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VehicleUpsertedBrand {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VehicleUpsertedBrand {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VehicleUpsertedBrand {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Model, as the driver entered it."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VehicleUpsertedModel(::std::string::String);
impl ::std::ops::Deref for VehicleUpsertedModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VehicleUpsertedModel> for ::std::string::String {
    fn from(value: VehicleUpsertedModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VehicleUpsertedModel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 60usize {
            return Err("longer than 60 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VehicleUpsertedModel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VehicleUpsertedModel {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VehicleUpsertedModel {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The carrier's verification moved; the app re-reads `GET /v1/carrier/verification` for the note."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VerificationChanged {
    pub status: VerificationStatus,
}
#[doc = "Frame `verification_changed` goway-realtime sends to an app."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VerificationChangedFrame {
    pub data: VerificationChanged,
    #[doc = "When the server sent the frame, RFC 3339 UTC."]
    pub ts: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The new verification status; `expired` when an approved document ran out."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VerificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Approved => f.write_str("approved"),
            Self::Rejected => f.write_str("rejected"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for VerificationStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VerificationStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerificationStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An organization's wallet started or stopped being allowed to receive offers. One event per user of the organization, or a single event with a null `user_id` when billing knows no user of it yet. `side` says which: realtime acts only on carriers, goway-loads refuses posting for shippers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WalletEligibilityChanged {
    #[doc = "Wallet balance in so'm; negative means debt."]
    pub balance: i64,
    #[doc = "How far below zero the balance may go, so'm."]
    pub debt_limit: u64,
    pub eligible: bool,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    #[doc = "Why offers stopped; null when the wallet is eligible."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<IneligibleReason>,
    pub side: WalletSide,
    #[doc = "Null when billing's replica holds no user of the organization."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub user_id: ::std::option::Option<::uuid::Uuid>,
}
#[doc = "Ledger line kind. `cancel_fee`: a shipper owes the fee for cancelling after the driver set off; `cancel_fee_share`: the carrier's share of it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WalletEntryKind {
    #[serde(rename = "commission")]
    Commission,
    #[serde(rename = "commission_adjustment")]
    CommissionAdjustment,
    #[serde(rename = "cash_topup")]
    CashTopup,
    #[serde(rename = "cash_void")]
    CashVoid,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "subscription_fee")]
    SubscriptionFee,
    #[serde(rename = "promo")]
    Promo,
    #[serde(rename = "referral")]
    Referral,
    #[serde(rename = "bonus_hold")]
    BonusHold,
    #[serde(rename = "bonus_release")]
    BonusRelease,
    #[serde(rename = "bonus_discount")]
    BonusDiscount,
    #[serde(rename = "cancel_fee")]
    CancelFee,
    #[serde(rename = "cancel_fee_share")]
    CancelFeeShare,
}
impl ::std::fmt::Display for WalletEntryKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Commission => f.write_str("commission"),
            Self::CommissionAdjustment => f.write_str("commission_adjustment"),
            Self::CashTopup => f.write_str("cash_topup"),
            Self::CashVoid => f.write_str("cash_void"),
            Self::Adjustment => f.write_str("adjustment"),
            Self::SubscriptionFee => f.write_str("subscription_fee"),
            Self::Promo => f.write_str("promo"),
            Self::Referral => f.write_str("referral"),
            Self::BonusHold => f.write_str("bonus_hold"),
            Self::BonusRelease => f.write_str("bonus_release"),
            Self::BonusDiscount => f.write_str("bonus_discount"),
            Self::CancelFee => f.write_str("cancel_fee"),
            Self::CancelFeeShare => f.write_str("cancel_fee_share"),
        }
    }
}
impl ::std::str::FromStr for WalletEntryKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "commission" => Ok(Self::Commission),
            "commission_adjustment" => Ok(Self::CommissionAdjustment),
            "cash_topup" => Ok(Self::CashTopup),
            "cash_void" => Ok(Self::CashVoid),
            "adjustment" => Ok(Self::Adjustment),
            "subscription_fee" => Ok(Self::SubscriptionFee),
            "promo" => Ok(Self::Promo),
            "referral" => Ok(Self::Referral),
            "bonus_hold" => Ok(Self::BonusHold),
            "bonus_release" => Ok(Self::BonusRelease),
            "bonus_discount" => Ok(Self::BonusDiscount),
            "cancel_fee" => Ok(Self::CancelFee),
            "cancel_fee_share" => Ok(Self::CancelFeeShare),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WalletEntryKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletEntryKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A line was written to an organization's wallet ledger."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WalletEntryPosted {
    #[doc = "Signed change of the balance, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: i64,
    #[doc = "Balance after the line, whole so'm (a bare integer; `currency` is UZS today); negative means debt."]
    pub balance: i64,
    #[doc = "The haul the line is for; null for lines not tied to a haul."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub booking_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: WalletEntryPostedCurrency,
    pub entry_id: ::uuid::Uuid,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub kind: WalletEntryKind,
    pub org_id: ::uuid::Uuid,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WalletEntryPostedCurrency(::std::string::String);
impl ::std::ops::Deref for WalletEntryPostedCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WalletEntryPostedCurrency> for ::std::string::String {
    fn from(value: WalletEntryPostedCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WalletEntryPostedCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WalletEntryPostedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletEntryPostedCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WalletEntryPostedCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A carrier's wallet came within `low_balance_warning` of the debt limit, before offers stop. Once until the headroom grows past the warning again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WalletLowBalance {
    #[doc = "Whole so'm; negative while the carrier owes."]
    pub balance: i64,
    pub currency: WalletLowBalanceCurrency,
    #[doc = "How far below zero the wallet may go before offers stop."]
    pub debt_limit: u64,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    #[doc = "What is left before offers stop: `balance + debt_limit`."]
    pub headroom: i64,
    pub org_id: ::uuid::Uuid,
}
#[doc = "`WalletLowBalanceCurrency`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WalletLowBalanceCurrency(::std::string::String);
impl ::std::ops::Deref for WalletLowBalanceCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WalletLowBalanceCurrency> for ::std::string::String {
    fn from(value: WalletLowBalanceCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WalletLowBalanceCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WalletLowBalanceCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletLowBalanceCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WalletLowBalanceCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Whose wallet: a carrier's stops instant offers past the debt limit; a shipper's stops posting new loads at any debt."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WalletSide {
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "shipper")]
    Shipper,
}
impl ::std::fmt::Display for WalletSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Carrier => f.write_str("carrier"),
            Self::Shipper => f.write_str("shipper"),
        }
    }
}
impl ::std::str::FromStr for WalletSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "carrier" => Ok(Self::Carrier),
            "shipper" => Ok(Self::Shipper),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WalletSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An operator took cash for an organization's wallet and issued a receipt."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WalletToppedUp {
    #[doc = "Cash received, whole so'm (a bare integer; `currency` is UZS today)."]
    pub amount: u64,
    #[doc = "Balance after the top-up, whole so'm (a bare integer; `currency` is UZS today); negative means debt."]
    pub balance: i64,
    #[doc = "ISO 4217 code; `UZS` today."]
    pub currency: WalletToppedUpCurrency,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub org_id: ::uuid::Uuid,
    pub payment_id: ::uuid::Uuid,
    #[doc = "Receipt number, `GW-<year>-<6 digits>`, for example `GW-2026-000123`."]
    pub receipt_no: ::std::string::String,
}
#[doc = "ISO 4217 code; `UZS` today."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WalletToppedUpCurrency(::std::string::String);
impl ::std::ops::Deref for WalletToppedUpCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WalletToppedUpCurrency> for ::std::string::String {
    fn from(value: WalletToppedUpCurrency) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WalletToppedUpCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WalletToppedUpCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletToppedUpCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WalletToppedUpCurrency {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
