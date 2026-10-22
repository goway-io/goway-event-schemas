/* Generated from goway-event-schemas by make generate; change the schemas, never this file. */

/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "AccountType".
 */
export type AccountType = "individual" | "business";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardRemovalReason".
 */
export type BoardRemovalReason = "booked" | "cancelled" | "expired";
/**
 * Which side claims.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingSide".
 */
export type BookingSide = "shipper" | "carrier";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingStatus".
 */
export type BookingStatus =
  "confirmed" | "en_route" | "at_pickup" | "loading" | "in_transit" | "delivered" | "completed" | "cancelled";
/**
 * Who cancelled.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CancelledBy".
 */
export type CancelledBy = "shipper" | "carrier" | "operator";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingChange".
 */
export type BookingChange = "issue_opened" | "issue_resolved" | "waiting_started";
/**
 * Who closed the haul: `carrier` when the driver finishes it at the drop-off, `system` for the auto-complete sweep.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CompletedBy".
 */
export type CompletedBy = "carrier" | "shipper" | "operator" | "system";
/**
 * Present only for an instant claim from dispatch; an accepted offer leaves it out.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingSource".
 */
export type BookingSource = "instant";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "EtaStop".
 */
export type EtaStop = "pickup" | "dropoff";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "IssueSide".
 */
export type IssueSide = "carrier" | "shipper";
/**
 * `continue`: the haul goes on as it was. `cancel`: cancelled without penalty to either side (a `booking.cancelled` follows). `reprice`: the price was recalculated. `waiting_fee`: a waiting charge was added.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "IssueResolution".
 */
export type IssueResolution = "continue" | "cancel" | "reprice" | "waiting_fee";
/**
 * `amount`: the sum is disputed. `damage`: the cargo arrived damaged. `shortage`: part of it is missing.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ObjectionKind".
 */
export type ObjectionKind = "amount" | "damage" | "shortage";
/**
 * `reminder`: the pickup is `plan_reminder_minutes` away. `departure`: `plan_departure_minutes` away, the app opens navigation. `overdue`: the pickup time came and the haul has not set off; operators are told too.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PlanStage".
 */
export type PlanStage = "reminder" | "departure" | "overdue";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DocumentKind".
 */
export type DocumentKind = "passport" | "driver_license" | "tech_passport" | "vehicle_photo" | "power_of_attorney";
/**
 * The decision.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ReviewDecision".
 */
export type ReviewDecision = "approved" | "rejected";
/**
 * A new version always waits for review.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubmittedDocumentStatus".
 */
export type SubmittedDocumentStatus = "pending";
/**
 * The new verification status; `expired` when an approved document ran out.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VerificationStatus".
 */
export type VerificationStatus = "pending" | "approved" | "rejected" | "expired";
/**
 * The side of the haul the person writes for.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatSide".
 */
export type ChatSide = "carrier" | "shipper";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatMessageKind".
 */
export type ChatMessageKind = "text" | "quick_reply" | "photo" | "location";
/**
 * Any frame an app may send; `type` tells which.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ClientFrame".
 */
export type ClientFrame =
  | AuthFrame
  | PingFrame
  | GoOnlineFrame
  | GoOfflineFrame
  | LocationFrame
  | OfferResponseFrame
  | SubscribeBookingFrame
  | UnsubscribeBookingFrame
  | SubscribeBoardFrame
  | UnsubscribeBoardFrame
  | TypingFrame;
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferAction".
 */
export type OfferAction = "accept" | "decline";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DevicePlatform".
 */
export type DevicePlatform = "android" | "ios" | "web";
/**
 * A side of the market: `shipper`, `carrier`, or `platform` for GoWay's own staff.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "MembershipSide".
 */
export type MembershipSide = "shipper" | "carrier" | "platform";
/**
 * Display currency; charges stay in UZS.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DisplayCurrency".
 */
export type DisplayCurrency = "UZS" | "USD" | "RUB" | "KZT";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "IneligibleReason".
 */
export type IneligibleReason = "debt" | "subscription";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadChangedField".
 */
export type LoadChangedField = "asking_amount" | "pickup_from" | "pickup_to" | "note" | "plan";
/**
 * How the cargo goes in.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadingMethod".
 */
export type LoadingMethod = "top" | "rear";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PriceMode".
 */
export type PriceMode = "fixed" | "negotiable";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PaymentMethod".
 */
export type PaymentMethod = "cash" | "transfer" | "card" | "mixed";
/**
 * What the person is inside the organization: `owner` for the account that opened it, `member` otherwise.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OrgRole".
 */
export type OrgRole = "owner" | "member";
/**
 * Who made the offer and how.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferKind".
 */
export type OfferKind = "carrier_request" | "carrier_counter" | "shipper_counter";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferOutcome".
 */
export type OfferOutcome = "won" | "lost" | "declined" | "expired" | "withdrawn";
/**
 * `accepted`: the driver took it. `lost`: they accepted but another driver was first. `declined`: they skipped it. `expired`: no answer in time. `withdrawn`: the load was taken by someone else, cancelled or edited before they answered.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResolution".
 */
export type OfferResolution = "accepted" | "lost" | "declined" | "expired" | "withdrawn";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OffersBlockedReason".
 */
export type OffersBlockedReason = "BALANCE_INSUFFICIENT" | "SUBSCRIPTION_REQUIRED";
/**
 * Why the server took the driver off the line; absent when the driver left by themselves. `vehicle_unavailable`: the line's truck was removed, left the fleet, or the line has none.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfflineReason".
 */
export type OfflineReason = "documents_expired" | "verification_rejected" | "vehicle_unavailable";
/**
 * Who was rated: the shipper rates the driver, the driver rates the shipper.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "RatingTarget".
 */
export type RatingTarget = "driver" | "shipper";
/**
 * Any frame goway-realtime may send; `type` tells which.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ServerFrame".
 */
export type ServerFrame =
  | ConnectedFrame
  | PongFrame
  | OnlineFrame
  | OfflineFrame
  | OfferProposedFrame
  | OfferResultFrame
  | SubscribedFrame
  | BookingCreatedFrame
  | BookingStatusFrame
  | BookingEtaFrame
  | BookingCompletedFrame
  | BookingCancelledFrame
  | DriverLocationFrame
  | OffersBlockedFrame
  | OffersUnblockedFrame
  | NotificationFrame
  | SessionStartedFrame
  | KickedFrame
  | ErrorFrame
  | VerificationChangedFrame
  | SearchProgressFrame
  | SearchExhaustedFrame
  | BoardSubscribedFrame
  | BoardLoadAddedFrame
  | BoardLoadUpdatedFrame
  | BoardLoadRemovedFrame
  | BookingUpdatedFrame
  | ChatMessageFrame
  | ChatReadFrame
  | ChatTypingFrame
  | SupportMessageFrame;
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportAuthor".
 */
export type SupportAuthor = "user" | "operator";
/**
 * Why every session of the user was closed. An operator's own note on a block stays with staff.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SessionRevokedReason".
 */
export type SessionRevokedReason = "password_change" | "password_reset" | "phone_change" | "deleted" | "blocked";
/**
 * Why the sessions were closed; only a sign-out from another device today.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SessionsClosedReason".
 */
export type SessionsClosedReason = "logout";
/**
 * What the ticket is about.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportTopic".
 */
export type SupportTopic =
  "suggestion" | "complaint" | "bug" | "payment" | "order" | "driver" | "app" | "documents" | "other";
/**
 * The ticket's status after the reply.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "TicketReplyStatus".
 */
export type TicketReplyStatus = "answered" | "closed";
/**
 * App theme.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Theme".
 */
export type Theme = "light" | "dark" | "system";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VehicleColor".
 */
export type VehicleColor =
  "white" | "black" | "silver" | "gray" | "red" | "blue" | "green" | "yellow" | "orange" | "brown" | "beige" | "other";
/**
 * How the vehicle is loaded.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VehicleLoadingMethod".
 */
export type VehicleLoadingMethod = "top" | "rear" | "both";
/**
 * Whose wallet: a carrier's stops instant offers past the debt limit; a shipper's stops posting new loads at any debt.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletSide".
 */
export type WalletSide = "carrier" | "shipper";
/**
 * Ledger line kind. `cancel_fee`: a shipper owes the fee for cancelling after the driver set off; `cancel_fee_share`: the carrier's share of it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletEntryKind".
 */
export type WalletEntryKind =
  | "commission"
  | "commission_adjustment"
  | "cash_topup"
  | "cash_void"
  | "adjustment"
  | "subscription_fee"
  | "promo"
  | "referral"
  | "bonus_hold"
  | "bonus_release"
  | "bonus_discount"
  | "cancel_fee"
  | "cancel_fee_share";

export interface GowayEventSchemas {
  [k: string]: unknown;
}
/**
 * The first frame on a new socket, within five seconds: the access token.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Auth".
 */
export interface Auth {
  /**
   * Access token (JWT).
   */
  token: string;
  /**
   * The app install's id, the one the device was registered under; lets this socket be signed out on its own when the device is removed.
   */
  installation_id?: string | null;
}
/**
 * Frame `auth` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "AuthFrame".
 */
export interface AuthFrame {
  type: "auth";
  data: Auth;
}
/**
 * A load appeared on a subscribed lane: newly posted or back after a cancelled booking. A signal; the app reads the list again over REST.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadAdded".
 */
export interface BoardLoadAdded {
  load_id: string;
  from_region_id: string;
  to_region_id: string;
  plan: boolean;
}
/**
 * Frame `board_load_added` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadAddedFrame".
 */
export interface BoardLoadAddedFrame {
  type: "board_load_added";
  data: BoardLoadAdded;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A load left a subscribed lane's board. A signal; the app drops it from the list.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadRemoved".
 */
export interface BoardLoadRemoved {
  load_id: string;
  from_region_id: string;
  to_region_id: string;
  reason: BoardRemovalReason;
}
/**
 * Frame `board_load_removed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadRemovedFrame".
 */
export interface BoardLoadRemovedFrame {
  type: "board_load_removed";
  data: BoardLoadRemoved;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A load on a subscribed lane was edited: price, pickup window or note. A signal; the app reads it again over REST.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadUpdated".
 */
export interface BoardLoadUpdated {
  load_id: string;
  from_region_id: string;
  to_region_id: string;
  plan: boolean;
}
/**
 * Frame `board_load_updated` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardLoadUpdatedFrame".
 */
export interface BoardLoadUpdatedFrame {
  type: "board_load_updated";
  data: BoardLoadUpdated;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * `subscribe_board` was accepted.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardSubscribed".
 */
export interface BoardSubscribed {
  from_region_id: string;
  to_region_id: string;
}
/**
 * Frame `board_subscribed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BoardSubscribedFrame".
 */
export interface BoardSubscribedFrame {
  type: "board_subscribed";
  data: BoardSubscribed;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * One side of a delivered haul says it settled at another sum; nothing changes until an operator decides.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingAmountClaimed".
 */
export interface BookingAmountClaimed {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  claim_id: string;
  booking_id: string;
  side: BookingSide;
  /**
   * The claimed sum, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  /**
   * The sum on record now, whole so'm (a bare integer; `currency` is UZS today).
   */
  recorded_amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
}
/**
 * An operator accepted an amount claim; the haul now settles at `settled_amount` and billing adjusts its commission. `claim_side` says whose claim the operator accepted; a shipper's objection is also announced by `booking.objection_resolved`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingAmountSettled".
 */
export interface BookingAmountSettled {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  carrier_org_id: string;
  /**
   * Null only when the load could not be read.
   */
  shipper_org_id: string | null;
  driver_user_id: string;
  status: BookingStatus;
  /**
   * The sum on record before, whole so'm (a bare integer; `currency` is UZS today).
   */
  previous_amount: number;
  /**
   * The sum the operator settled on, whole so'm (a bare integer; `currency` is UZS today).
   */
  settled_amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  claim_side: BookingSide;
  /**
   * The load's short code, for example `GW-7F3K2M`; wallets and receipts show it.
   */
  load_code: string;
}
/**
 * A booking ended without being completed: either side cancelled before the cargo was loaded (`by` is `shipper` or `carrier`), or an operator cancelled it (`by` is `operator`), which after loading happens only through a reported issue. The load goes back to the board unless the operator withdrew it, which a `load.cancelled` then says.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCancelled".
 */
export interface BookingCancelled {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  by: CancelledBy;
  /**
   * What the canceller wrote; empty when nothing was written.
   */
  reason: string;
  shipper_org_id: string;
  carrier_org_id: string;
  /**
   * Null when no driver was assigned.
   */
  driver_user_id: string | null;
  /**
   * The cancelled booking was a plan.
   */
  plan: boolean;
  /**
   * The cancellation counts against the driver's acceptance rate: the driver's own cancellation of an instant haul, or of a plan later than goway-loads' free-cancel window before pickup, or an operator's cancellation of a booking whose driver never showed up. False for every other shipper and operator cancellation.
   */
  penalized: boolean;
  /**
   * Code from the cancellation reasons catalogue; null for older clients and system cancellations.
   */
  reason_code: string | null;
  /**
   * Cancellation fee the shipper owes the carrier, whole so'm (`cancel_fee_percent` once the driver set off); null when none.
   */
  fee_amount: number | null;
  currency: string;
  /**
   * The load's short code, for example `GW-7F3K2M`; wallets and receipts show it.
   */
  load_code: string;
}
/**
 * The haul was cancelled; the app reads the booking for what it costs whom. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCancelledData".
 */
export interface BookingCancelledData {
  booking_id: string;
  load_id: string;
  /**
   * The load's short code, for example `GW-7F3K2M`; wallets and receipts show it.
   */
  load_code: string;
  by: CancelledBy;
  /**
   * Code from the cancellation reasons catalogue; null for older clients and system cancellations.
   */
  reason_code: string | null;
  /**
   * What the canceller wrote; empty when nothing was written.
   */
  reason: string;
  /**
   * The cancelled booking was a plan.
   */
  plan: boolean;
}
/**
 * Frame `booking_cancelled` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCancelledFrame".
 */
export interface BookingCancelledFrame {
  type: "booking_cancelled";
  data: BookingCancelledData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A delivered haul was closed: by the shipper, by an operator, or by the auto-complete sweep. All three send the same fields. Billing charges its commission from this event.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCompleted".
 */
export interface BookingCompleted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  carrier_org_id: string;
  shipper_org_id: string;
  driver_user_id: string;
  /**
   * Price agreed at booking, whole so'm (a bare integer; `currency` is UZS today).
   */
  agreed_amount: number;
  /**
   * What the haul actually settled at: the agreed price unless another sum was confirmed, whole so'm (a bare integer; `currency` is UZS today).
   */
  settled_amount: number;
  /**
   * When the haul was closed; the code allows null, it is always set. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  completed_at: string | null;
  completed_by: CompletedBy;
  /**
   * Price lines; the first is always the agreed tariff.
   */
  charges: ChargeLine[];
  /**
   * Billable minutes the driver waited at pickup.
   */
  waiting_minutes: number;
  /**
   * What the waiting costs, whole so'm (a bare integer; `currency` is UZS today).
   */
  waiting_amount: number;
  /**
   * Whether the waiting is charged to the shipper.
   */
  waiting_billable: boolean;
  /**
   * Charges plus waiting, whole so'm (a bare integer; `currency` is UZS today); the code allows null.
   */
  total_amount: number | null;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  /**
   * The load's short code, for example `GW-7F3K2M`; wallets and receipts show it.
   */
  load_code: string;
}
/**
 * One priced line of the haul.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChargeLine".
 */
export interface ChargeLine {
  /**
   * `tariff`, `loaders` or `req:<requirement code>`.
   */
  code: string;
  /**
   * Line total, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  qty: number;
}
/**
 * The haul is closed; the app reads the booking for the bill and asks for a rating. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCompletedData".
 */
export interface BookingCompletedData {
  booking_id: string;
  load_id: string;
  /**
   * The load's short code, for example `GW-7F3K2M`; wallets and receipts show it.
   */
  load_code: string;
  /**
   * When the haul was closed; the code allows null, it is always set. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  completed_at: string | null;
  completed_by: CompletedBy;
}
/**
 * Frame `booking_completed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCompletedFrame".
 */
export interface BookingCompletedFrame {
  type: "booking_completed";
  data: BookingCompletedData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A load was booked. Two paths: an accepted offer, or an instant claim from dispatch, which adds `source: instant`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCreated".
 */
export interface BookingCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  carrier_org_id: string;
  /**
   * The driver; the offer path allows null, a booking always has one.
   */
  driver_user_id: string | null;
  /**
   * Agreed price, whole so'm (a bare integer; `currency` is UZS today).
   */
  agreed_amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  source?: BookingSource;
  /**
   * The person who posted the load; they hear `booking_created` on the socket too.
   */
  shipper_user_id: string;
  /**
   * Earliest pickup; null when not given. A pickup well ahead makes this booking a plan rather than a trip under way.
   */
  pickup_from: string | null;
  /**
   * Latest pickup; null when not given.
   */
  pickup_to: string | null;
  /**
   * Driving time of the haul in minutes; null when routing was unavailable.
   */
  duration_min: number | null;
  /**
   * Start of the span this booking keeps the driver busy, goway-loads' plan buffer included. Dispatch offers the driver nothing whose trip overlaps it.
   */
  busy_from: string;
  /**
   * End of that span; null when the haul's duration is unknown, which keeps the driver busy until the booking ends.
   */
  busy_until: string | null;
  /**
   * The driver booked a plan: a load scheduled ahead, taken from the plan list.
   */
  plan: boolean;
}
/**
 * A haul was booked. Sent to the booked driver and to the shipper who posted the load; the app reads the booking for the rest. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCreatedData".
 */
export interface BookingCreatedData {
  booking_id: string;
  load_id: string;
  /**
   * The driver booked a plan: a load scheduled ahead, taken from the plan list.
   */
  plan: boolean;
  source?: BookingSource;
  /**
   * Agreed price, whole so'm (a bare integer; `currency` is UZS today).
   */
  agreed_amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  /**
   * Earliest pickup; null when not given. A pickup well ahead makes this booking a plan rather than a trip under way.
   */
  pickup_from: string | null;
  /**
   * Latest pickup; null when not given.
   */
  pickup_to: string | null;
}
/**
 * Frame `booking_created` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingCreatedFrame".
 */
export interface BookingCreatedFrame {
  type: "booking_created";
  data: BookingCreatedData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The arrival window or the road position changed. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingEtaData".
 */
export interface BookingEtaData {
  booking_id: string;
  load_id: string;
  status: BookingStatus;
  /**
   * Which stop the window is for; null when no arrival is expected any more.
   */
  eta_stop: EtaStop | null;
  /**
   * Earliest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  eta_from: string | null;
  /**
   * Latest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  eta_to: string | null;
  /**
   * Road kilometres left to the next stop; null before the haul sets off or while routing is unavailable.
   */
  remaining_km: number | null;
  /**
   * The last region boundary the truck crossed, for «Angrendan o'tdi · 142 km qoldi»; null until it crosses one.
   */
  last_waypoint: PassedWaypoint | null;
}
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PassedWaypoint".
 */
export interface PassedWaypoint {
  region_id: string;
  passed_at: string;
}
/**
 * Frame `booking_eta` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingEtaFrame".
 */
export interface BookingEtaFrame {
  type: "booking_eta";
  data: BookingEtaData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The arrival window or the time the truck is free again moved: when the haul is booked (`confirmed`), at every carrier step, and on fresh GPS while `en_route` or `in_transit`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingEtaUpdated".
 */
export interface BookingEtaUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  shipper_org_id: string;
  status: BookingStatus;
  /**
   * Which stop the window is for; null when no arrival is expected any more.
   */
  eta_stop: EtaStop | null;
  /**
   * Earliest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  eta_from: string | null;
  /**
   * Latest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  eta_to: string | null;
  /**
   * When the truck is expected to be empty again. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  free_at: string | null;
  /**
   * Where the truck will be free: the drop-off region; null when unknown.
   */
  free_region_id: string | null;
  /**
   * The booked vehicle's truck class code.
   */
  truck_class: string;
  /**
   * Road kilometres left to the next stop; null before the haul sets off or while routing is unavailable.
   */
  remaining_km: number | null;
  /**
   * The last region boundary the truck crossed, for «Angrendan o'tdi · 142 km qoldi»; null until it crosses one.
   */
  last_waypoint: PassedWaypoint | null;
}
/**
 * Someone on the haul reported a problem («Muammo bor»). Until an operator resolves it the haul is under review: its step does not move, the waiting clock is paused and the driver's cancellation does not count against them.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingIssueOpened".
 */
export interface BookingIssueOpened {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  issue_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  /**
   * The person who posted the load.
   */
  shipper_user_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  opened_by: IssueSide;
  /**
   * Code from the issue reasons catalogue.
   */
  reason_code: string;
  /**
   * The booking's step when the issue was opened.
   */
  status: string;
  opened_at: string;
}
/**
 * An operator resolved a reported problem; the haul is no longer under review.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingIssueResolved".
 */
export interface BookingIssueResolved {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  issue_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  /**
   * The person who posted the load.
   */
  shipper_user_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  resolution: IssueResolution;
  /**
   * The new price for `reprice`, the added charge for `waiting_fee`, whole so'm; null otherwise.
   */
  amount: number | null;
  currency: string;
  resolved_at: string;
}
/**
 * The shipper objected to a completed haul within the objection window: the sum, damage or shortage. An operator decides it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingObjectionOpened".
 */
export interface BookingObjectionOpened {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  claim_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  /**
   * The person who posted the load.
   */
  shipper_user_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  kind: ObjectionKind;
  /**
   * The sum the shipper says is right, whole so'm; null when they named none.
   */
  amount: number | null;
  currency: string;
  opened_at: string;
}
/**
 * An operator decided a shipper's objection. An accepted one that names a sum also sends `booking.amount_settled`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingObjectionResolved".
 */
export interface BookingObjectionResolved {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  claim_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  /**
   * The person who posted the load.
   */
  shipper_user_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  kind: ObjectionKind;
  accepted: boolean;
  /**
   * What the haul now settles at when the decision changed it, whole so'm; null otherwise.
   */
  settled_amount: number | null;
  currency: string;
  /**
   * The operator's reason, shown to both sides.
   */
  note: string;
  resolved_at: string;
}
/**
 * A booked plan's pickup is near: time for the reminder, time to set off, or the pickup time came and the driver has not set off. One event per stage per booking.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingPlanDue".
 */
export interface BookingPlanDue {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  driver_user_id: string;
  carrier_org_id: string;
  shipper_org_id: string;
  stage: PlanStage;
  pickup_from: string;
  /**
   * First pickup stop's address; null only when the load has no stops.
   */
  pickup_address: string | null;
}
/**
 * Hours before a truck is empty: loads are waiting in the region where it unloads. Sent once per haul.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingReturnLoads".
 */
export interface BookingReturnLoads {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  driver_user_id: string;
  carrier_org_id: string;
  /**
   * The drop-off region, where the truck will be free.
   */
  region_id: string;
  /**
   * Matching loads waiting there.
   */
  count: number;
  /**
   * How many of them go back to the haul's pickup region.
   */
  heads_home: number;
  /**
   * When the truck is expected to be empty. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  free_at: string;
}
/**
 * The carrier moved the haul one step on: `en_route`, `at_pickup`, `loading`, `in_transit` or `delivered`. Completion and cancellation have their own subjects.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingStatusChanged".
 */
export interface BookingStatusChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  status: BookingStatus;
  shipper_org_id: string;
  carrier_org_id: string;
  /**
   * When the step happened: the driver's tap, which an app offline for a while sends late. Steps arrive in order; this is the time to show.
   */
  occurred_at: string;
}
/**
 * The haul moved to another step. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingStatusData".
 */
export interface BookingStatusData {
  booking_id: string;
  load_id: string;
  status: BookingStatus;
  /**
   * When the step happened: the driver's tap, which an app offline for a while sends late. Steps arrive in order; this is the time to show.
   */
  occurred_at: string;
}
/**
 * Frame `booking_status` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingStatusFrame".
 */
export interface BookingStatusFrame {
  type: "booking_status";
  data: BookingStatusData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Something on a haul changed besides its step; sent to the driver and the shipper without a subscription. A signal: the app reads the booking again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingUpdated".
 */
export interface BookingUpdated {
  booking_id: string;
  change: BookingChange;
  issue_id: string | null;
}
/**
 * Frame `booking_updated` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingUpdatedFrame".
 */
export interface BookingUpdatedFrame {
  type: "booking_updated";
  data: BookingUpdated;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The free waiting at the pickup ran out and paid waiting began. Once per booking.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "BookingWaitingStarted".
 */
export interface BookingWaitingStarted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  shipper_org_id: string;
  /**
   * The person who posted the load.
   */
  shipper_user_id: string;
  carrier_org_id: string;
  driver_user_id: string;
  started_at: string;
  /**
   * Whole so'm per minute.
   */
  rate_per_minute: number;
  currency: string;
}
/**
 * A carrier's document passed its expiry date; the hourly sweep marked it expired.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierDocumentExpired".
 */
export interface CarrierDocumentExpired {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  kind: DocumentKind;
  /**
   * The document's expiry date. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  expired_at: string;
}
/**
 * A carrier's document expires within the warning window; sent once per document.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierDocumentExpiring".
 */
export interface CarrierDocumentExpiring {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  kind: DocumentKind;
  /**
   * The document's expiry date. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  expires_at: string;
}
/**
 * An operator approved or rejected a carrier's document.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierDocumentReviewed".
 */
export interface CarrierDocumentReviewed {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  kind: DocumentKind;
  status: ReviewDecision;
  /**
   * What the operator wrote; a rejection always has one.
   */
  note: string | null;
}
/**
 * A carrier submitted a new version of a document; it waits for review.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierDocumentUpdated".
 */
export interface CarrierDocumentUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  kind: DocumentKind;
  status: SubmittedDocumentStatus;
  /**
   * Version of this document kind, from 1.
   */
  version: number;
}
/**
 * A driver opened or changed their line, its return direction or its truck.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierLineChanged".
 */
export interface CarrierLineChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  org_id: string;
  /**
   * Where the line starts.
   */
  from_region_id: string;
  /**
   * Where the line ends.
   */
  to_region_id: string;
  /**
   * The way back is taken too.
   */
  bidirectional: boolean;
  /**
   * The truck for the line; null after it was removed.
   */
  vehicle_id: string | null;
  changed_at: string;
}
/**
 * A carrier organization's verification moved: submitted or reopened by a replaced document (`pending`), or decided by an operator.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CarrierVerificationChanged".
 */
export interface CarrierVerificationChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  status: VerificationStatus;
  /**
   * The organization's members, so a consumer can act on each without its own member list.
   */
  user_ids?: string[];
}
/**
 * A message was written in a haul's chat between the driver and the shipper. Realtime shows it at once; notify pushes it to whoever is not looking.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatMessageCreated".
 */
export interface ChatMessageCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  message_id: string;
  /**
   * The sending app's id for the message, so a resend after a lost reply shows once.
   */
  client_message_id: string | null;
  booking_id: string;
  load_id: string;
  sender_user_id: string;
  sender_side: ChatSide;
  /**
   * People on the other side who should hear it: today the driver or the person who posted the load.
   */
  recipient_user_ids: string[];
  kind: ChatMessageKind;
  /**
   * The words; for a quick reply, its text in the sender's language. Null for a bare photo or location.
   */
  text: string | null;
  /**
   * Code from the quick replies catalogue when `kind` is `quick_reply`.
   */
  quick_reply_code: string | null;
  /**
   * The `chat_photo` file; the app reads its link over REST.
   */
  photo_file_id: string | null;
  lat: number | null;
  lng: number | null;
  created_at: string;
}
/**
 * A message in a haul's chat, to both sides and to the writer's other devices. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatMessageData".
 */
export interface ChatMessageData {
  message_id: string;
  /**
   * The sending app's id for the message, so a resend after a lost reply shows once.
   */
  client_message_id: string | null;
  booking_id: string;
  load_id: string;
  sender_user_id: string;
  sender_side: ChatSide;
  kind: ChatMessageKind;
  /**
   * The words; for a quick reply, its text in the sender's language. Null for a bare photo or location.
   */
  text: string | null;
  /**
   * Code from the quick replies catalogue when `kind` is `quick_reply`.
   */
  quick_reply_code: string | null;
  /**
   * The `chat_photo` file; the app reads its link over REST.
   */
  photo_file_id: string | null;
  lat: number | null;
  lng: number | null;
  created_at: string;
}
/**
 * Frame `chat_message` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatMessageFrame".
 */
export interface ChatMessageFrame {
  type: "chat_message";
  data: ChatMessageData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Someone read a haul's chat up to a message; the other side sees «o'qildi».
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatRead".
 */
export interface ChatRead {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  reader_user_id: string;
  reader_side: ChatSide;
  last_read_message_id: string;
  /**
   * People on the other side who should hear it: today the driver or the person who posted the load.
   */
  recipient_user_ids: string[];
  read_at: string;
}
/**
 * One side read a haul's chat up to a message. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatReadData".
 */
export interface ChatReadData {
  booking_id: string;
  reader_user_id: string;
  reader_side: ChatSide;
  last_read_message_id: string;
  read_at: string;
}
/**
 * Frame `chat_read` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatReadFrame".
 */
export interface ChatReadFrame {
  type: "chat_read";
  data: ChatReadData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The other side is typing in a haul's chat. Not stored; show it for a few seconds.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatTyping".
 */
export interface ChatTyping {
  booking_id: string;
  user_id: string;
}
/**
 * Frame `chat_typing` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ChatTypingFrame".
 */
export interface ChatTypingFrame {
  type: "chat_typing";
  data: ChatTyping;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Frame `ping` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PingFrame".
 */
export interface PingFrame {
  type: "ping";
  data?: Ping;
}
/**
 * Nothing: an empty object, or no `data` at all.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Ping".
 */
export interface Ping {}
/**
 * Frame `go_online` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "GoOnlineFrame".
 */
export interface GoOnlineFrame {
  type: "go_online";
  data: GoOnline;
}
/**
 * Start receiving instant offers on the line saved with `PUT /v1/carrier/line`, with its vehicle.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "GoOnline".
 */
export interface GoOnline {}
/**
 * Frame `go_offline` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "GoOfflineFrame".
 */
export interface GoOfflineFrame {
  type: "go_offline";
  data?: GoOffline;
}
/**
 * Leave the line. Nothing to send: an empty object, or no `data` at all.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "GoOffline".
 */
export interface GoOffline {}
/**
 * Frame `location` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LocationFrame".
 */
export interface LocationFrame {
  type: "location";
  data: Location;
}
/**
 * The driver's position. Relayed to booking viewers and sampled as evidence only while a booking is active.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Location".
 */
export interface Location {
  lat: number;
  lng: number;
  /**
   * Degrees from north, when the device knows it.
   */
  heading?: number | null;
  /**
   * Speed the device reports, when it knows it.
   */
  speed?: number | null;
}
/**
 * Frame `offer_response` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResponseFrame".
 */
export interface OfferResponseFrame {
  type: "offer_response";
  data: OfferResponse;
}
/**
 * The driver's answer to an instant offer.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResponse".
 */
export interface OfferResponse {
  load_id: string;
  action: OfferAction;
}
/**
 * Frame `subscribe_booking` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubscribeBookingFrame".
 */
export interface SubscribeBookingFrame {
  type: "subscribe_booking";
  data: SubscribeBooking;
}
/**
 * Watch a booking of the caller's organization: status, arrival window and the driver's position. The driver and the shipper who posted the load get every frame but the position without subscribing; subscribe for the position, or as another member of either organization.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubscribeBooking".
 */
export interface SubscribeBooking {
  booking_id: string;
}
/**
 * Frame `unsubscribe_booking` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UnsubscribeBookingFrame".
 */
export interface UnsubscribeBookingFrame {
  type: "unsubscribe_booking";
  data: UnsubscribeBooking;
}
/**
 * Stop watching a booking.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UnsubscribeBooking".
 */
export interface UnsubscribeBooking {
  booking_id: string;
}
/**
 * Frame `subscribe_board` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubscribeBoardFrame".
 */
export interface SubscribeBoardFrame {
  type: "subscribe_board";
  data: SubscribeBoard;
}
/**
 * Hear when loads on a lane appear, change or leave the board, for a live plan list. One direction per frame; a driver with a way back subscribes to both.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubscribeBoard".
 */
export interface SubscribeBoard {
  from_region_id: string;
  to_region_id: string;
}
/**
 * Frame `unsubscribe_board` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UnsubscribeBoardFrame".
 */
export interface UnsubscribeBoardFrame {
  type: "unsubscribe_board";
  data: UnsubscribeBoard;
}
/**
 * Stop hearing about a lane.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UnsubscribeBoard".
 */
export interface UnsubscribeBoard {
  from_region_id: string;
  to_region_id: string;
}
/**
 * Frame `typing` an app sends to goway-realtime.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "TypingFrame".
 */
export interface TypingFrame {
  type: "typing";
  data: Typing;
}
/**
 * The caller is typing in a haul's chat; sent at most every few seconds while typing.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Typing".
 */
export interface Typing {
  booking_id: string;
}
/**
 * The `auth` frame was accepted; the socket is live.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Connected".
 */
export interface Connected {
  user_id: string;
  /**
   * This socket's id, 32 hex characters.
   */
  conn_id: string;
  config: ConnectionConfig;
}
/**
 * Current realtime settings the app plays by.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ConnectionConfig".
 */
export interface ConnectionConfig {
  /**
   * Seconds a driver has to answer an offer.
   */
  offer_ttl_seconds: number;
  /**
   * Extra seconds after the deadline a late answer is still honoured.
   */
  offer_grace_seconds: number;
  /**
   * Seconds between durable GPS samples.
   */
  location_sample_seconds: number;
}
/**
 * Frame `connected` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ConnectedFrame".
 */
export interface ConnectedFrame {
  type: "connected";
  data: Connected;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The push provider rejected a device's token as dead; the device should be shown as unreachable.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DevicePushFailed".
 */
export interface DevicePushFailed {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  device_id: string;
  user_id: string;
}
/**
 * The owner removed a device; its sessions are closed and it is pushed no more.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DeviceRemoved".
 */
export interface DeviceRemoved {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  device_id: string;
  user_id: string;
  /**
   * The app install's own id.
   */
  installation_id: string;
}
/**
 * A device was registered or came back after removal, or its push token changed or was cleared (signing out clears it). `side` is the side its app signed in to: a push for one side goes only to that side's devices; a device without it, not signed in since sides were kept, hears every side.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DeviceUpserted".
 */
export interface DeviceUpserted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  device_id: string;
  user_id: string;
  platform: DevicePlatform;
  /**
   * FCM registration token; null when the device can no longer be pushed.
   */
  push_token: string | null;
  side?: MembershipSide;
}
/**
 * An instant search ran for the whole search window and no driver took the load. The search goes on; the shipper is told so they can raise the price, move the pickup or cancel.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DispatchExhausted".
 */
export interface DispatchExhausted {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper's organization.
   */
  org_id: string;
  /**
   * The person who posted the load.
   */
  created_by: string;
  from_region_id: string;
  to_region_id: string;
  /**
   * Drivers the load was offered to so far.
   */
  seen: number;
  /**
   * Current price, whole so'm.
   */
  amount: number;
  /**
   * The price GoWay suggests trying next: the current one raised by the configured step.
   */
  suggested_amount: number;
  currency: string;
  searched_seconds: number;
}
/**
 * An instant load was offered to a wave of drivers on its line, so a driver whose app is in the background still sees it. Loss-tolerant: the socket frame is the offer itself.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DispatchOffered".
 */
export interface DispatchOffered {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * Drivers the wave reached.
   *
   * @minItems 1
   */
  drivers: [string, ...string[]];
  from_region_id: string;
  to_region_id: string;
  /**
   * Truck class code from the reference list.
   */
  truck_class: string;
  /**
   * The load's fixed price, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  weight_kg: number;
  /**
   * Answer deadline, unix milliseconds in server time.
   */
  deadline_at: number;
}
/**
 * The booked driver's live position, relayed to the booking's viewers as it arrives; dropped when a socket is slow.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverLocation".
 */
export interface DriverLocation {
  booking_id: string;
  lat: number;
  lng: number;
  /**
   * Degrees from north; null when the driver's device did not send it.
   */
  heading: number | null;
  /**
   * Null when the driver's device did not send it.
   */
  speed: number | null;
}
/**
 * Frame `driver_location` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverLocationFrame".
 */
export interface DriverLocationFrame {
  type: "driver_location";
  data: DriverLocation;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A driver changed which instant offers they want: how far to the pickup at most and the lowest price.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverOfferFilterChanged".
 */
export interface DriverOfferFilterChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * Farthest pickup from the driver's last position; null for no limit. Not applied while the position is unknown.
   */
  max_pickup_km: number | null;
  /**
   * Lowest price the driver wants offered, whole so'm; null for no limit.
   */
  min_amount: number | null;
  /**
   * When it was saved; consumers ignore a change older than the one they hold.
   */
  changed_at: string;
}
/**
 * A driver left the line: went offline, the line expired, the grace after a disconnect ran out, or the sessions were revoked.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverOffline".
 */
export interface DriverOffline {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * From the presence record, which always has it.
   */
  org_id: string;
}
/**
 * A driver opened or changed a line and waits for instant offers.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverOnline".
 */
export interface DriverOnline {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  org_id: string;
  from_region_id: string;
  to_region_id: string;
  vehicle_id: string;
  /**
   * Also takes loads on the way back.
   */
  bidirectional: boolean;
}
/**
 * A driver's completed-haul count or rating changed. Counters only grow, so a late older event is recognisable.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "DriverStatsChanged".
 */
export interface DriverStatsChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  completed_count: number;
  /**
   * Null until the first rating.
   */
  rating_avg: number | null;
  rating_count: number;
}
/**
 * Frame `error` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ErrorFrame".
 */
export interface ErrorFrame {
  type: "error";
  data: FrameError;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A frame was refused; the same problem shape as REST errors. Two forms are sent: a first frame that is not `auth`, an unreadable frame and a refused booking subscription send only `code` and `key`; every other refusal sends the full problem body with `type`, `title`, `status` and `code`, plus `detail`, `key` and `params` when set.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "FrameError".
 */
export interface FrameError {
  /**
   * Problem type URI, `https://api.goway.uz/problems/<code>`.
   */
  type?: string;
  /**
   * HTTP status phrase, for example `Conflict`.
   */
  title?: string;
  /**
   * Stable code the app branches on.
   */
  code: string;
  /**
   * Translation key for the message.
   */
  key?: string;
  status?: number;
  detail?: string;
  /**
   * Values for the translated message.
   */
  params?: {
    [k: string]: unknown;
  };
}
/**
 * This socket is being closed by the server; sign in again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Kicked".
 */
export interface Kicked {
  /**
   * `device_removed`, `logout` (closed from another device), the `user.session_revoked` reason, or what a service passed to `/internal/kick`; null when none was given.
   */
  reason: string | null;
}
/**
 * Frame `kicked` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "KickedFrame".
 */
export interface KickedFrame {
  type: "kicked";
  data: Kicked;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The shipper withdrew an open load.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadCancelled".
 */
export interface LoadCancelled {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper organization.
   */
  org_id: string;
}
/**
 * An open load passed its expiry unbooked; the sweep closed it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadExpired".
 */
export interface LoadExpired {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper organization.
   */
  org_id: string;
}
/**
 * A plan is close to its pickup and no driver has booked it yet; the shipper is told they may raise the price. Once per load.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadPlanUnclaimed".
 */
export interface LoadPlanUnclaimed {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper's organization.
   */
  org_id: string;
  /**
   * The person who posted the load.
   */
  created_by: string;
  from_region_id: string;
  to_region_id: string;
  pickup_from: string;
  /**
   * Current price, whole so'm; null for a load without an asking price.
   */
  amount: number | null;
  currency: string;
  /**
   * Drivers who saw the plan so far.
   */
  views: number;
}
/**
 * A load is open on the board: just posted (one event per truck of a multi-truck post), or back on the board after its booking was cancelled, which adds `reposted: true`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadPosted".
 */
export interface LoadPosted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper organization.
   */
  org_id: string;
  /**
   * Pickup region; null when the address resolved to none.
   */
  from_region_id: string | null;
  /**
   * Drop-off region; null when the address resolved to none.
   */
  to_region_id: string | null;
  /**
   * Truck class code from the reference list.
   */
  truck_class: string;
  loading_method: LoadingMethod;
  needs_refrigeration: boolean;
  /**
   * Whether instant dispatch may offer the load.
   */
  allow_instant: boolean;
  price_mode: PriceMode;
  /**
   * The shipper's price, whole so'm (a bare integer; `currency` is UZS today); null when it asks for offers.
   */
  asking_amount: number | null;
  /**
   * Shipper bonus set aside for this truck, whole so'm (a bare integer; `currency` is UZS today); null when none.
   */
  bonus_amount: number | null;
  /**
   * Market price GoWay suggested, whole so'm (a bare integer; `currency` is UZS today); null when there was none.
   */
  suggested_amount: number | null;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  weight_kg: number;
  /**
   * Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_from: string | null;
  /**
   * Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_to: string | null;
  /**
   * Pickup point latitude; null only when the load has no stops.
   */
  pickup_lat: number | null;
  /**
   * Pickup point longitude; null only when the load has no stops.
   */
  pickup_lng: number | null;
  /**
   * Shared by the sibling loads of one multi-truck post; null for a single truck.
   */
  group_id: string | null;
  is_urgent: boolean;
  payment_method: PaymentMethod;
  /**
   * Present, and true, only when a cancelled booking put the load back.
   */
  reposted?: boolean;
  /**
   * The person who posted the load; dispatch tells them how the search goes.
   */
  created_by: string;
  /**
   * First pickup stop's address as the shipper wrote it; null only when the load has no stops.
   */
  pickup_address: string | null;
  /**
   * Last drop-off stop's address; null only when the load has no stops.
   */
  dropoff_address: string | null;
  /**
   * Pickup and drop-off stops together.
   */
  stops_count: number;
  /**
   * Driving distance through the stops; null when routing was unavailable at posting.
   */
  distance_km: number | null;
  /**
   * Driving time for the same route in minutes; null together with the distance.
   */
  duration_min: number | null;
  /**
   * Cargo type code from the reference catalogue.
   */
  cargo_category: string;
  volume_m3: number | null;
  /**
   * Loaders the shipper ordered with the truck.
   */
  loaders_count: number;
  /**
   * The pickup is scheduled more than goway-loads' instant horizon ahead: drivers book it from the plan list, and dispatch makes no instant offers for it.
   */
  plan: boolean;
}
/**
 * A load still open on the board was edited by its shipper or an operator: the price, the pickup window or the note. Carries the whole load as `load.posted` does, plus what changed.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LoadUpdated".
 */
export interface LoadUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  /**
   * The shipper organization.
   */
  org_id: string;
  /**
   * Pickup region; null when the address resolved to none.
   */
  from_region_id: string | null;
  /**
   * Drop-off region; null when the address resolved to none.
   */
  to_region_id: string | null;
  /**
   * Truck class code from the reference list.
   */
  truck_class: string;
  loading_method: LoadingMethod;
  needs_refrigeration: boolean;
  /**
   * Whether instant dispatch may offer the load.
   */
  allow_instant: boolean;
  price_mode: PriceMode;
  /**
   * The shipper's price, whole so'm (a bare integer; `currency` is UZS today); null when it asks for offers.
   */
  asking_amount: number | null;
  /**
   * Shipper bonus set aside for this truck, whole so'm (a bare integer; `currency` is UZS today); null when none.
   */
  bonus_amount: number | null;
  /**
   * Market price GoWay suggested, whole so'm (a bare integer; `currency` is UZS today); null when there was none.
   */
  suggested_amount: number | null;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
  weight_kg: number;
  /**
   * Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_from: string | null;
  /**
   * Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_to: string | null;
  /**
   * Pickup point latitude; null only when the load has no stops.
   */
  pickup_lat: number | null;
  /**
   * Pickup point longitude; null only when the load has no stops.
   */
  pickup_lng: number | null;
  /**
   * Shared by the sibling loads of one multi-truck post; null for a single truck.
   */
  group_id: string | null;
  is_urgent: boolean;
  payment_method: PaymentMethod;
  /**
   * The person who posted the load; dispatch tells them how the search goes.
   */
  created_by: string;
  /**
   * First pickup stop's address as the shipper wrote it; null only when the load has no stops.
   */
  pickup_address: string | null;
  /**
   * Last drop-off stop's address; null only when the load has no stops.
   */
  dropoff_address: string | null;
  /**
   * Pickup and drop-off stops together.
   */
  stops_count: number;
  /**
   * Driving distance through the stops; null when routing was unavailable at posting.
   */
  distance_km: number | null;
  /**
   * Driving time for the same route in minutes; null together with the distance.
   */
  duration_min: number | null;
  /**
   * Cargo type code from the reference catalogue.
   */
  cargo_category: string;
  volume_m3: number | null;
  /**
   * Loaders the shipper ordered with the truck.
   */
  loaders_count: number;
  /**
   * Fields this edit changed. A changed price or pickup window restarts the instant search, so every fitting driver is offered it again; `plan` changes when the new pickup moves the load across the instant horizon.
   *
   * @minItems 1
   */
  changed: [LoadChangedField, ...LoadChangedField[]];
  /**
   * When the edit was saved; consumers ignore an edit older than the one they hold.
   */
  updated_at: string;
  /**
   * The pickup is scheduled more than goway-loads' instant horizon ahead: drivers book it from the plan list, and dispatch makes no instant offers for it.
   */
  plan: boolean;
}
/**
 * A driver's GPS position during an active booking, sampled every `location_sample_seconds` as evidence of the haul.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "LocationSampled".
 */
export interface LocationSampled {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  driver_user_id: string;
  lat: number;
  lng: number;
  /**
   * When the position was sampled, RFC 3339 UTC with milliseconds.
   */
  recorded_at: string;
}
/**
 * A person joined an organization on one side of the market, or a removed membership came back. A person holds at most one live membership per side. The full state is sent, so a consumer upserts it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "MembershipCreated".
 */
export interface MembershipCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  /**
   * Stable id of the membership; every later event about it carries the same one.
   */
  membership_id: string;
  /**
   * The person.
   */
  user_id: string;
  /**
   * The organization the person works in on this side.
   */
  org_id: string;
  side: MembershipSide;
  /**
   * The role the side's tokens carry, for example `carrier`, `shipper` or `superadmin`.
   */
  role: string;
  org_role: OrgRole;
  /**
   * When it was created or came back. ISO 8601 with offset.
   */
  created_at: string;
}
/**
 * A membership ended: the person closed that side or the account was deleted. Keep it as a tombstone; a later `membership.created` with the same id brings it back.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "MembershipRemoved".
 */
export interface MembershipRemoved {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  /**
   * Stable id of the membership; every later event about it carries the same one.
   */
  membership_id: string;
  /**
   * The person.
   */
  user_id: string;
  /**
   * The organization the person works in on this side.
   */
  org_id: string;
  side: MembershipSide;
  /**
   * When it ended. ISO 8601 with offset.
   */
  removed_at: string;
}
/**
 * A live membership changed its organization's side, its role or the person's role inside the organization. The full state is sent; keep the newest by `updated_at`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "MembershipUpdated".
 */
export interface MembershipUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  /**
   * Stable id of the membership; every later event about it carries the same one.
   */
  membership_id: string;
  /**
   * The person.
   */
  user_id: string;
  /**
   * The organization the person works in on this side.
   */
  org_id: string;
  side: MembershipSide;
  /**
   * The role the side's tokens carry, for example `carrier`, `shipper` or `superadmin`.
   */
  role: string;
  org_role: OrgRole;
  /**
   * When it changed. ISO 8601 with offset.
   */
  updated_at: string;
}
/**
 * Something landed in the user's notification feed. `data` opens the right screen on tap and `unread` is the new badge. Copied from `notification.created`; a field that event lacked or sent empty arrives as null.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Notification".
 */
export interface Notification {
  notification_id: string | null;
  /**
   * Template kind, for example `driver_found` or `announcement`.
   */
  kind: string | null;
  /**
   * Title in the user's language.
   */
  title: string | null;
  /**
   * Text in the user's language.
   */
  body: string | null;
  /**
   * The keys that open the screen, as the push carries them (`booking_id`, `load_id`, `ticket_id`...).
   */
  data: {
    [k: string]: unknown;
  } | null;
  /**
   * Unread feed items, this one included.
   */
  unread: number | null;
}
/**
 * A message landed in a user's notification feed: from an event, or from an admin announcement (`kind: announcement`). `side` is the side the notice is for, shown only in that side's app; a notice about the person has none and every app shows it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "NotificationCreated".
 */
export interface NotificationCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  notification_id: string;
  user_id: string;
  /**
   * Template kind, for example `driver_found`, `booking_status` or `announcement`.
   */
  kind: string;
  /**
   * Title in the user's language.
   */
  title: string;
  /**
   * Text in the user's language.
   */
  body: string;
  /**
   * What the app needs to open the right screen when the item is tapped: the same keys the push carries, such as `booking_id`, `load_id` or `ticket_id`. Empty for an announcement.
   */
  data?: {
    [k: string]: unknown;
  };
  /**
   * The user's unread feed items, this one included, for the badge.
   */
  unread?: number;
  side?: MembershipSide;
}
/**
 * Frame `notification` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "NotificationFrame".
 */
export interface NotificationFrame {
  type: "notification";
  data: Notification;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Every switch, sent whole whenever one of them changed.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "NotificationPrefs".
 */
export interface NotificationPrefs {
  /**
   * Push for new offers and dispatch results.
   */
  offers: boolean;
  /**
   * Push for booking status changes.
   */
  orders: boolean;
  /**
   * Promotions and news.
   */
  marketing: boolean;
  /**
   * SMS fallback for critical events.
   */
  sms: boolean;
  /**
   * Push for chat messages about a haul and from support.
   */
  chat: boolean;
}
/**
 * An offer was accepted and became a booking; published together with `booking.created`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferAccepted".
 */
export interface OfferAccepted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  offer_id: string;
  load_id: string;
  booking_id: string;
  shipper_org_id: string;
  carrier_org_id: string;
  /**
   * The driver who made the offer; the code allows null, an offer always has one.
   */
  driver_user_id: string | null;
}
/**
 * A carrier asked for a load, or either side countered the other's price.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferCreated".
 */
export interface OfferCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  offer_id: string;
  load_id: string;
  kind: OfferKind;
  carrier_org_id: string;
  shipper_org_id: string;
  /**
   * Offered price, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
}
/**
 * An instant load offered to this driver; answer before `deadline_at`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferProposed".
 */
export interface OfferProposed {
  load_id: string;
  from_region_id: string;
  to_region_id: string;
  truck_class: string;
  /**
   * Price in minor units of `currency`.
   */
  amount: number;
  currency: string;
  weight_kg: number;
  /**
   * Answer deadline, unix milliseconds in server time.
   */
  deadline_at: number;
  server_time: string;
  /**
   * First pickup stop's address as the shipper wrote it; null only when the load has no stops.
   */
  pickup_address: string | null;
  /**
   * Last drop-off stop's address; null only when the load has no stops.
   */
  dropoff_address: string | null;
  /**
   * Pickup and drop-off stops together.
   */
  stops_count: number;
  /**
   * Driving distance through the stops; null when routing was unavailable at posting.
   */
  distance_km: number | null;
  /**
   * Driving time for the same route in minutes; null together with the distance.
   */
  duration_min: number | null;
  /**
   * Straight-line distance from the driver's last position to the pickup; null while the position is unknown.
   */
  pickup_distance_km: number | null;
  /**
   * Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_from: string | null;
  /**
   * Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  pickup_to: string | null;
  /**
   * Cargo type code from the reference catalogue.
   */
  cargo_category: string;
  volume_m3: number | null;
  /**
   * Loaders the shipper ordered with the truck.
   */
  loaders_count: number;
  loading_method: LoadingMethod;
  needs_refrigeration: boolean;
  payment_method: PaymentMethod;
  is_urgent: boolean;
}
/**
 * Frame `offer_proposed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferProposedFrame".
 */
export interface OfferProposedFrame {
  type: "offer_proposed";
  data: OfferProposed;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * One driver's instant offer ended. One event per driver per offer; it feeds the driver's acceptance rate.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResolved".
 */
export interface OfferResolved {
  /**
   * Unique id of this event, a random UUID stamped by goway-lib-rs when it publishes. Consumers dedupe on it.
   */
  event_id: string;
  load_id: string;
  driver_user_id: string;
  /**
   * The driver's organization.
   */
  org_id: string;
  outcome: OfferResolution;
  offered_at: string;
  resolved_at: string;
}
/**
 * How an offer ended for this driver.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResult".
 */
export interface OfferResult {
  load_id: string;
  outcome: OfferOutcome;
  /**
   * Sent only when the outcome is `won`; null when goway-loads' answer could not be read.
   */
  booking_id?: string | null;
}
/**
 * Frame `offer_result` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResultFrame".
 */
export interface OfferResultFrame {
  type: "offer_result";
  data: OfferResult;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The wallet stopped being eligible; no offers until it is settled.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OffersBlocked".
 */
export interface OffersBlocked {
  reason: OffersBlockedReason;
}
/**
 * Frame `offers_blocked` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OffersBlockedFrame".
 */
export interface OffersBlockedFrame {
  type: "offers_blocked";
  data: OffersBlocked;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The wallet is eligible again after being blocked; nothing in it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OffersUnblocked".
 */
export interface OffersUnblocked {}
/**
 * Frame `offers_unblocked` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OffersUnblockedFrame".
 */
export interface OffersUnblockedFrame {
  type: "offers_unblocked";
  data: OffersUnblocked;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The driver left the line: after `go_offline`, or taken off it with a `reason`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Offline".
 */
export interface Offline {
  reason?: OfflineReason;
}
/**
 * Frame `offline` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfflineFrame".
 */
export interface OfflineFrame {
  type: "offline";
  data: Offline;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The driver is on their line: after `go_online`, and again whenever the line is changed while online.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Online".
 */
export interface Online {
  /**
   * Where the line starts.
   */
  from_region_id: string;
  /**
   * Where the line ends.
   */
  to_region_id: string;
  /**
   * The way back is taken too.
   */
  bidirectional: boolean;
  /**
   * The truck the offers are matched against.
   */
  vehicle_id: string;
  /**
   * Place in the outbound line's queue, from 1.
   */
  queue_position: number;
  /**
   * Set when the wallet blocks offers, with the reason; null otherwise.
   */
  offers_blocked: OffersBlockedReason | null;
}
/**
 * Frame `online` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OnlineFrame".
 */
export interface OnlineFrame {
  type: "online";
  data: Online;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * An organization's details changed. Only what changed is sent, besides `org_id` and `updated_at`: a role switch sends `kind` and `account_type`; the organization screen any of `name`, `legal_name`, `inn`, `contact_phone`, `address`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OrgUpdated".
 */
export interface OrgUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  /**
   * Market side: `shipper` or `carrier`.
   */
  kind?: string;
  account_type?: AccountType;
  /**
   * Display name.
   */
  name?: string;
  /**
   * Registered company name.
   */
  legal_name?: string;
  /**
   * Tax id (STIR/INN), 9 digits.
   */
  inn?: string;
  /**
   * E.164 number, `+998…`.
   */
  contact_phone?: string;
  address?: string;
  /**
   * When the change was made. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  updated_at: string;
}
/**
 * The answer to `ping`; nothing in it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Pong".
 */
export interface Pong {}
/**
 * Frame `pong` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "PongFrame".
 */
export interface PongFrame {
  type: "pong";
  data: Pong;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The shipper rated a completed haul; sent once per haul.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "RatingCreated".
 */
export interface RatingCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  booking_id: string;
  load_id: string;
  target: RatingTarget;
  /**
   * The rated driver; the code allows null, a booking always has one.
   */
  driver_user_id: string | null;
  carrier_org_id: string;
  shipper_org_id: string;
  /**
   * 1 to 5.
   */
  stars: number;
  /**
   * The person who posted the load: the one rated when `target` is `shipper`.
   */
  shipper_user_id: string;
  /**
   * Codes from the rating tags catalogue, for example `on_time`, `cargo_ready`.
   */
  tags: string[];
}
/**
 * An admin opened or closed a region for the marketplace.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "RegionChanged".
 */
export interface RegionChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  region_id: string;
  is_active: boolean;
}
/**
 * To the shipper: no driver took the load within the search window. The search goes on; the app offers to raise the price, move the pickup to a plan or cancel.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SearchExhausted".
 */
export interface SearchExhausted {
  load_id: string;
  seen: number;
  /**
   * Current price, whole so'm.
   */
  amount: number;
  /**
   * The raised price GoWay suggests.
   */
  suggested_amount: number;
  currency: string;
}
/**
 * Frame `search_exhausted` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SearchExhaustedFrame".
 */
export interface SearchExhaustedFrame {
  type: "search_exhausted";
  data: SearchExhausted;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * To the shipper while their instant load is being offered: how many drivers have seen it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SearchProgress".
 */
export interface SearchProgress {
  load_id: string;
  /**
   * Drivers the load was offered to so far.
   */
  seen: number;
  /**
   * When the search began; the app counts the waiting time from it.
   */
  started_at: string;
}
/**
 * Frame `search_progress` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SearchProgressFrame".
 */
export interface SearchProgressFrame {
  type: "search_progress";
  data: SearchProgress;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Frame `subscribed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SubscribedFrame".
 */
export interface SubscribedFrame {
  type: "subscribed";
  data: Subscribed;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * `subscribe_booking` was accepted.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "Subscribed".
 */
export interface Subscribed {
  booking_id: string;
}
/**
 * Frame `session_started` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SessionStartedFrame".
 */
export interface SessionStartedFrame {
  type: "session_started";
  data: SessionStarted;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * Somebody signed in to this account; ignore it when `installation_id` is this app's own. Copied from `user.login`; a field that event lacked or sent empty arrives as null.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SessionStarted".
 */
export interface SessionStarted {
  device_id: string | null;
  installation_id: string | null;
  /**
   * Device platform.
   */
  platform: DevicePlatform | null;
  /**
   * Device model.
   */
  model: string | null;
  new_device: boolean;
  /**
   * When the sign-in happened, as `user.login` wrote it.
   */
  at: string | null;
}
/**
 * Frame `verification_changed` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VerificationChangedFrame".
 */
export interface VerificationChangedFrame {
  type: "verification_changed";
  data: VerificationChanged;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * The carrier's verification moved; the app re-reads `GET /v1/carrier/verification` for the note.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VerificationChanged".
 */
export interface VerificationChanged {
  status: VerificationStatus;
}
/**
 * Frame `support_message` goway-realtime sends to an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportMessageFrame".
 */
export interface SupportMessageFrame {
  type: "support_message";
  data: SupportMessageData;
  /**
   * When the server sent the frame, RFC 3339 UTC.
   */
  ts: string;
}
/**
 * A message in a support conversation, to the person who opened it. The internal event's ids for organizations, recipients and the event itself never reach an app.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportMessageData".
 */
export interface SupportMessageData {
  ticket_id: string;
  /**
   * The ticket's short number.
   */
  number: string;
  message_id: string;
  author: SupportAuthor;
  text: string;
  created_at: string;
}
/**
 * An admin changed or reset runtime settings of one service. Published by goway-lib for goway-auth, goway-loads, goway-billing and goway-notify, and by goway-realtime itself, with the same fields.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SettingsChanged".
 */
export interface SettingsChanged {
  /**
   * Unique id of this event: the outbox row id from a Python service, a random UUID from goway-realtime. Consumers dedupe on it.
   */
  event_id: string;
  /**
   * The service whose settings changed, for example `goway-loads`.
   */
  service: string;
  /**
   * Setting keys that changed.
   */
  keys: string[];
  /**
   * The admin who changed them; null when not known.
   */
  by: string | null;
}
/**
 * A shipper's standing moved: a driver rated them. Replicas show it on the shipper card.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ShipperStatsChanged".
 */
export interface ShipperStatsChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * Average over the rating window; null before the first rating.
   */
  rating_avg: number | null;
  /**
   * All ratings ever received; only grows.
   */
  rating_count: number;
}
/**
 * A message was added to a support conversation, by the person who opened it or by an operator.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportMessageCreated".
 */
export interface SupportMessageCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  ticket_id: string;
  /**
   * The ticket's short number.
   */
  number: string;
  message_id: string;
  author: SupportAuthor;
  /**
   * The person the conversation belongs to.
   */
  user_id: string;
  text: string;
  created_at: string;
}
/**
 * A user opened a support ticket.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportTicketCreated".
 */
export interface SupportTicketCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  ticket_id: string;
  /**
   * Reference the user sees: `T-` and six characters, for example `T-7KQ2MZ`.
   */
  number: string;
  user_id: string;
  topic: SupportTopic;
  /**
   * The haul the ticket is about; null when none was named.
   */
  order_id: string | null;
}
/**
 * Support answered or closed a ticket.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "SupportTicketReplied".
 */
export interface SupportTicketReplied {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  ticket_id: string;
  /**
   * Reference the user sees: `T-` and six characters, for example `T-7KQ2MZ`.
   */
  number: string;
  /**
   * Who opened the ticket.
   */
  user_id: string;
  status: TicketReplyStatus;
}
/**
 * An operator blocked the account; the app shows the reason on its blocked screen.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserBlocked".
 */
export interface UserBlocked {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * What the operator told the user, in their words.
   */
  reason: string;
  blocked_at: string;
}
/**
 * A person's account was opened (self-signup, admin create) or restored (with `updated_at`). The account is the person; the organization and side they work in come in `membership.created`, sent with it.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserCreated".
 */
export interface UserCreated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * E.164 number, `+998…`. A restore sends the stored value, which is `del:…` for an account its owner deleted.
   */
  phone: string;
  /**
   * App language code: uz, uzc, ru, en, kaa or zh.
   */
  language: string;
  /**
   * Admin create only; null when the admin gave none.
   */
  first_name?: string | null;
  /**
   * Admin create only; null when the admin gave none.
   */
  last_name?: string | null;
  /**
   * Admin restore only: when the account came back. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  updated_at?: string;
}
/**
 * An account was deleted: by its owner, without `updated_at`, or by an admin, soft or hard, with `updated_at`. Each side the person held ends with its own `membership.removed`.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserDeleted".
 */
export interface UserDeleted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * Admin deletes only: when the account was deleted. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  updated_at?: string;
}
/**
 * Somebody signed in to the account: by code, password, password reset or phone change. The device fields are null when the app sent no device.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserLogin".
 */
export interface UserLogin {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * Device row the session belongs to.
   */
  device_id: string | null;
  /**
   * The app install's own id.
   */
  installation_id: string | null;
  /**
   * Device platform.
   */
  platform: DevicePlatform | null;
  /**
   * Device model as the app reported it.
   */
  model: string | null;
  /**
   * True the first time this install signs in, or when a removed device comes back.
   */
  new_device: boolean;
  /**
   * Client address.
   */
  ip: string | null;
  /**
   * When the sign-in happened. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  at: string;
}
/**
 * Every session of the account was closed; each app must sign in again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserSessionRevoked".
 */
export interface UserSessionRevoked {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  reason: SessionRevokedReason;
}
/**
 * Some of the account's sessions were closed from another device; those installs must sign in again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserSessionsClosed".
 */
export interface UserSessionsClosed {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * Installs whose sessions were closed; never empty.
   *
   * @minItems 1
   */
  installation_ids: [string, ...string[]];
  reason: SessionsClosedReason;
}
/**
 * An operator lifted the block; the user may sign in again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserUnblocked".
 */
export interface UserUnblocked {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  unblocked_at: string;
}
/**
 * Something about an account changed. Only what changed is sent, besides `user_id` and `updated_at`, and four paths send four field sets: a confirmed phone change sends `phone`; the account settings screen any of `language`, `currency`, `theme`, `notification_prefs`; an admin edit any of `phone`, `language`, `first_name`, `last_name`; a profile edit `first_name`, `last_name`, `email` and `avatar_url` together, each of them possibly null. A change of side or role is a `membership.*` event.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "UserUpdated".
 */
export interface UserUpdated {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  user_id: string;
  /**
   * New E.164 number, `+998…`.
   */
  phone?: string;
  /**
   * App language code: uz, uzc, ru, en, kaa or zh.
   */
  language?: string;
  currency?: DisplayCurrency;
  theme?: Theme;
  notification_prefs?: NotificationPrefs;
  /**
   * Null only from a profile edit, when the profile has no first name.
   */
  first_name?: string | null;
  /**
   * Null only from a profile edit, when the profile has no last name.
   */
  last_name?: string | null;
  /**
   * Profile edit only; null when the profile has no email.
   */
  email?: string | null;
  /**
   * Profile edit only. Public avatar address; null when there is none or it would expire.
   */
  avatar_url?: string | null;
  /**
   * When the change was made. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  updated_at: string;
}
/**
 * A fleet vehicle was removed.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VehicleRemoved".
 */
export interface VehicleRemoved {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  vehicle_id: string;
  org_id: string;
}
/**
 * A fleet vehicle was added or changed; the whole vehicle is sent every time.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "VehicleUpserted".
 */
export interface VehicleUpserted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  vehicle_id: string;
  org_id: string;
  /**
   * Normalized plate: no spaces, upper case.
   */
  plate_number: string;
  /**
   * Truck class code from the reference list.
   */
  truck_class: string;
  /**
   * Capability codes: rear_ramp, open_top, side_curtain, tail_lift, refrigerated.
   */
  capabilities: string[];
  loading_method: VehicleLoadingMethod;
  /**
   * Payload in kilograms; null when not given.
   */
  capacity_kg: number | null;
  /**
   * Make, as the driver entered it.
   */
  brand?: string | null;
  /**
   * Model, as the driver entered it.
   */
  model?: string | null;
  /**
   * Body colour code; apps name it in their language.
   */
  color?: VehicleColor | null;
  /**
   * Year of manufacture.
   */
  year?: number | null;
  /**
   * Body volume in m³.
   */
  volume_m3?: number | null;
  /**
   * When the vehicle was saved. ISO 8601 with offset, as Python's `isoformat()` writes it.
   */
  updated_at: string;
}
/**
 * An organization's wallet started or stopped being allowed to receive offers. One event per user of the organization, or a single event with a null `user_id` when billing knows no user of it yet. `side` says which: realtime acts only on carriers, goway-loads refuses posting for shippers.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletEligibilityChanged".
 */
export interface WalletEligibilityChanged {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  /**
   * Null when billing's replica holds no user of the organization.
   */
  user_id: string | null;
  eligible: boolean;
  /**
   * Wallet balance in so'm; negative means debt.
   */
  balance: number;
  /**
   * How far below zero the balance may go, so'm.
   */
  debt_limit: number;
  /**
   * Why offers stopped; null when the wallet is eligible.
   */
  reason: IneligibleReason | null;
  side: WalletSide;
}
/**
 * A line was written to an organization's wallet ledger.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletEntryPosted".
 */
export interface WalletEntryPosted {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  entry_id: string;
  kind: WalletEntryKind;
  /**
   * Signed change of the balance, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  /**
   * Balance after the line, whole so'm (a bare integer; `currency` is UZS today); negative means debt.
   */
  balance: number;
  /**
   * The haul the line is for; null for lines not tied to a haul.
   */
  booking_id: string | null;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
}
/**
 * A carrier's wallet came within `low_balance_warning` of the debt limit, before offers stop. Once until the headroom grows past the warning again.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletLowBalance".
 */
export interface WalletLowBalance {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  /**
   * Whole so'm; negative while the carrier owes.
   */
  balance: number;
  /**
   * How far below zero the wallet may go before offers stop.
   */
  debt_limit: number;
  /**
   * What is left before offers stop: `balance + debt_limit`.
   */
  headroom: number;
  currency: string;
}
/**
 * An operator took cash for an organization's wallet and issued a receipt.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "WalletToppedUp".
 */
export interface WalletToppedUp {
  /**
   * Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
   */
  event_id: string;
  org_id: string;
  payment_id: string;
  /**
   * Receipt number, `GW-<year>-<6 digits>`, for example `GW-2026-000123`.
   */
  receipt_no: string;
  /**
   * Cash received, whole so'm (a bare integer; `currency` is UZS today).
   */
  amount: number;
  /**
   * Balance after the top-up, whole so'm (a bare integer; `currency` is UZS today); negative means debt.
   */
  balance: number;
  /**
   * ISO 4217 code; `UZS` today.
   */
  currency: string;
}
