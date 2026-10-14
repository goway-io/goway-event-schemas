#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "A booking ended before the cargo was loaded. The load goes back to the board."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BookingCancelled {
    pub booking_id: ::uuid::Uuid,
    pub by: CancelledBy,
    pub carrier_org_id: ::uuid::Uuid,
    #[doc = "Null when no driver was assigned."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub driver_user_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it."]
    pub event_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    #[doc = "What the canceller wrote; empty when nothing was written."]
    pub reason: ::std::string::String,
    pub shipper_org_id: ::uuid::Uuid,
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
#[doc = "Any frame an app may send; `type` tells which."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientFrame {
    #[doc = "Frame `go_online` an app sends to goway-realtime."]
    #[serde(rename = "go_online")]
    GoOnline(GoOnline),
    #[doc = "Frame `offer_response` an app sends to goway-realtime."]
    #[serde(rename = "offer_response")]
    OfferResponse(OfferResponse),
}
impl ::std::convert::From<GoOnline> for ClientFrame {
    fn from(value: GoOnline) -> Self {
        Self::GoOnline(value)
    }
}
impl ::std::convert::From<OfferResponse> for ClientFrame {
    fn from(value: OfferResponse) -> Self {
        Self::OfferResponse(value)
    }
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
#[doc = "A frame was refused; the same problem shape as REST errors."]
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
}
#[doc = "Start receiving instant offers on a line with one vehicle."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GoOnline {
    #[doc = "Also take loads on the way back."]
    #[serde(default = "defaults::default_bool::<true>")]
    pub bidirectional: bool,
    #[doc = "How long to stay online; must be one of the allowed durations."]
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 12>")]
    pub duration_hours: ::std::num::NonZeroU64,
    pub from_region_id: ::uuid::Uuid,
    pub to_region_id: ::uuid::Uuid,
    pub vehicle_id: ::uuid::Uuid,
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
    pub currency: OfferProposedCurrency,
    #[doc = "Answer deadline, unix milliseconds in server time."]
    pub deadline_at: i64,
    pub from_region_id: ::uuid::Uuid,
    pub load_id: ::uuid::Uuid,
    pub server_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub to_region_id: ::uuid::Uuid,
    pub truck_class: ::std::string::String,
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
    #[doc = "Set when the outcome is `won`."]
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
#[doc = "Any frame goway-realtime may send; `type` tells which."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ServerFrame {
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
    #[doc = "Frame `error` goway-realtime sends to an app."]
    #[serde(rename = "error")]
    Error {
        data: FrameError,
        #[doc = "When the server sent the frame, RFC 3339 UTC."]
        ts: ::chrono::DateTime<::chrono::offset::Utc>,
    },
}
#[doc = "An organization's wallet started or stopped being allowed to receive offers. One event per user of the organization."]
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
    pub user_id: ::uuid::Uuid,
}
#[doc = " Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
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
