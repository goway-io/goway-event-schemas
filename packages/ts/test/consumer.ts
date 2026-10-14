import type { ClientFrame, OfferOutcome, ServerFrame, WalletEligibilityChanged } from "../src/index.ts";

export function describe(frame: ServerFrame): string {
  switch (frame.type) {
    case "offer_proposed":
      return `${frame.data.amount} ${frame.data.currency} until ${frame.data.deadline_at}`;
    case "offer_result":
      return frame.data.outcome === "won" ? `booked ${frame.data.booking_id ?? "?"}` : frame.data.outcome;
    case "error":
      return frame.data.code;
    default: {
      const unhandled: never = frame;
      return unhandled;
    }
  }
}

export const online: ClientFrame = {
  type: "go_online",
  data: {
    from_region_id: "0192f6a0-7c1e-7b3a-9d2e-1a2b3c4d5e07",
    to_region_id: "0192f6a0-7c1e-7b3a-9d2e-1a2b3c4d5e08",
    vehicle_id: "0192f6a0-7c1e-7b3a-9d2e-1a2b3c4d5e09",
  },
};

export const outcomes: OfferOutcome[] = ["won", "lost", "declined", "expired", "withdrawn"];

export const stillEligible: WalletEligibilityChanged["reason"] = null;

// @ts-expect-error a frame type the server never sends
export const unknownFrame: ServerFrame = { type: "teleport", data: {}, ts: "2026-10-14T09:30:00Z" };

// @ts-expect-error go_online needs a vehicle
export const noVehicle: ClientFrame = { type: "go_online", data: { from_region_id: "a", to_region_id: "b" } };

// @ts-expect-error the data of one frame cannot travel under another frame's type
export const crossed: ClientFrame = { type: "offer_response", data: online.data };
