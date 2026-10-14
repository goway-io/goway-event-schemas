/* Generated from goway-event-schemas by make generate; change the schemas, never this file. */

/**
 * Who cancelled.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "CancelledBy".
 */
export type CancelledBy = "shipper" | "carrier" | "operator";
/**
 * Any frame an app may send; `type` tells which.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ClientFrame".
 */
export type ClientFrame = GoOnlineFrame | OfferResponseFrame;
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferAction".
 */
export type OfferAction = "accept" | "decline";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "IneligibleReason".
 */
export type IneligibleReason = "debt" | "subscription";
/**
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferOutcome".
 */
export type OfferOutcome = "won" | "lost" | "declined" | "expired" | "withdrawn";
/**
 * Any frame goway-realtime may send; `type` tells which.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "ServerFrame".
 */
export type ServerFrame = OfferProposedFrame | OfferResultFrame | ErrorFrame;

export interface GowayEventSchemas {
  [k: string]: unknown;
}
/**
 * A booking ended before the cargo was loaded. The load goes back to the board.
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
}
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
 * Start receiving instant offers on a line with one vehicle.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "GoOnline".
 */
export interface GoOnline {
  from_region_id: string;
  to_region_id: string;
  vehicle_id: string;
  /**
   * How long to stay online; must be one of the allowed durations.
   */
  duration_hours?: number;
  /**
   * Also take loads on the way back.
   */
  bidirectional?: boolean;
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
 * A frame was refused; the same problem shape as REST errors.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "FrameError".
 */
export interface FrameError {
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
 * How an offer ended for this driver.
 *
 * This interface was referenced by `GowayEventSchemas`'s JSON-Schema
 * via the `definition` "OfferResult".
 */
export interface OfferResult {
  load_id: string;
  outcome: OfferOutcome;
  /**
   * Set when the outcome is `won`.
   */
  booking_id?: string;
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
 * An organization's wallet started or stopped being allowed to receive offers. One event per user of the organization.
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
  user_id: string;
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
}
