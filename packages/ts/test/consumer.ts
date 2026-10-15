import type { ClientFrame, OfferOutcome, ServerFrame, WalletEligibilityChanged } from "../src/index.ts";

export function describe(frame: ServerFrame): string {
  switch (frame.type) {
    case "connected":
      return `${frame.data.conn_id} on ${frame.data.worker}, offers last ${frame.data.config.offer_ttl_seconds}s`;
    case "pong":
    case "offline":
    case "offers_unblocked":
      return frame.type;
    case "online":
      return `#${frame.data.queue_position} on ${frame.data.from_region_id}→${frame.data.to_region_id}, blocked: ${frame.data.offers_blocked ?? "no"}`;
    case "offer_proposed":
      return `${frame.data.amount} ${frame.data.currency} until ${frame.data.deadline_at}`;
    case "offer_result":
      return frame.data.outcome === "won" ? `booked ${frame.data.booking_id ?? "?"}` : frame.data.outcome;
    case "subscribed":
      return frame.data.booking_id;
    case "booking_created":
      return `${frame.data.agreed_amount} ${frame.data.currency} ${frame.data.source ?? "offer"}`;
    case "booking_status":
      return frame.data.status;
    case "booking_eta":
      return frame.data.eta_to ?? "no window";
    case "booking_completed":
      return `${frame.data.settled_amount} by ${frame.data.completed_by}`;
    case "booking_cancelled":
      return frame.data.by;
    case "driver_location":
      return `${frame.data.lat},${frame.data.lng} heading ${frame.data.heading ?? "?"}`;
    case "verification_changed":
      return frame.data.status;
    case "offers_blocked":
      return frame.data.reason;
    case "search_progress":
      return `${frame.data.seen} saw it since ${frame.data.started_at}`;
    case "board_subscribed":
      return `${frame.data.from_region_id}→${frame.data.to_region_id}`;
    case "board_load_added":
    case "board_load_updated":
      return `${frame.data.load_id} plan: ${frame.data.plan}`;
    case "board_load_removed":
      return `${frame.data.load_id} ${frame.data.reason}`;
    case "search_exhausted":
      return `nobody took it at ${frame.data.amount}; try ${frame.data.suggested_amount} ${frame.data.currency}`;
    case "notification":
      return frame.data.title ?? "";
    case "session_started":
      return frame.data.installation_id ?? "unknown install";
    case "kicked":
      return frame.data.reason ?? "signed out";
    case "error":
      return frame.data.code;
    default: {
      const unhandled: never = frame;
      return unhandled;
    }
  }
}

export const online: ClientFrame = { type: "go_online", data: {} };

export const ping: ClientFrame = { type: "ping" };

export const outcomes: OfferOutcome[] =["won", "lost", "declined", "expired", "withdrawn"];

export const stillEligible: WalletEligibilityChanged["reason"] = null;

// @ts-expect-error a frame type the server never sends
export const unknownFrame: ServerFrame = { type: "teleport", data: {}, ts: "2026-10-14T09:30:00Z" };

// @ts-expect-error the data of one frame cannot travel under another frame's type
export const crossed: ClientFrame = { type: "offer_response", data: online.data };
