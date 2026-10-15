// Written by hand: no Dart generator emits a discriminated union from JSON Schema. When a frame is added to
// the schemas, add its case here; the example tests fail until it is (a valid example must not read as
// UnknownServerFrame).

import 'models.dart';

/// Any frame an app sends; `type` tells which.
sealed class ClientFrame {
  const ClientFrame();

  Map<String, dynamic> toJson();

  static ClientFrame fromJson(Map<String, dynamic> json) => switch (json['type']) {
        'auth' => AuthMessage(AuthFrame.fromJson(json)),
        'ping' => PingMessage(PingFrame.fromJson(json)),
        'go_online' => GoOnlineMessage(GoOnlineFrame.fromJson(json)),
        'go_offline' => GoOfflineMessage(GoOfflineFrame.fromJson(json)),
        'location' => LocationMessage(LocationFrame.fromJson(json)),
        'offer_response' => OfferResponseMessage(OfferResponseFrame.fromJson(json)),
        'subscribe_booking' => SubscribeBookingMessage(SubscribeBookingFrame.fromJson(json)),
        'unsubscribe_booking' => UnsubscribeBookingMessage(UnsubscribeBookingFrame.fromJson(json)),
        'subscribe_board' => SubscribeBoardMessage(SubscribeBoardFrame.fromJson(json)),
        'unsubscribe_board' => UnsubscribeBoardMessage(UnsubscribeBoardFrame.fromJson(json)),
        final other => throw FormatException('unknown client frame type: $other'),
      };
}

final class AuthMessage extends ClientFrame {
  const AuthMessage(this.frame);
  final AuthFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class PingMessage extends ClientFrame {
  const PingMessage(this.frame);
  final PingFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class GoOnlineMessage extends ClientFrame {
  const GoOnlineMessage(this.frame);
  final GoOnlineFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class GoOfflineMessage extends ClientFrame {
  const GoOfflineMessage(this.frame);
  final GoOfflineFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class LocationMessage extends ClientFrame {
  const LocationMessage(this.frame);
  final LocationFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OfferResponseMessage extends ClientFrame {
  const OfferResponseMessage(this.frame);
  final OfferResponseFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SubscribeBookingMessage extends ClientFrame {
  const SubscribeBookingMessage(this.frame);
  final SubscribeBookingFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class UnsubscribeBookingMessage extends ClientFrame {
  const UnsubscribeBookingMessage(this.frame);
  final UnsubscribeBookingFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SubscribeBoardMessage extends ClientFrame {
  const SubscribeBoardMessage(this.frame);
  final SubscribeBoardFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class UnsubscribeBoardMessage extends ClientFrame {
  const UnsubscribeBoardMessage(this.frame);
  final UnsubscribeBoardFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

/// Any frame goway-realtime sends; `type` tells which.
///
/// A new server frame type is additive: a server newer than this package may send one, and it arrives as
/// [UnknownServerFrame] instead of an error, so an installed app keeps its socket. Ignore it.
sealed class ServerFrame {
  const ServerFrame();

  Map<String, dynamic> toJson();

  static ServerFrame fromJson(Map<String, dynamic> json) => switch (json['type']) {
        'connected' => ConnectedMessage(ConnectedFrame.fromJson(json)),
        'pong' => PongMessage(PongFrame.fromJson(json)),
        'online' => OnlineMessage(OnlineFrame.fromJson(json)),
        'offline' => OfflineMessage(OfflineFrame.fromJson(json)),
        'offer_proposed' => OfferProposedMessage(OfferProposedFrame.fromJson(json)),
        'offer_result' => OfferResultMessage(OfferResultFrame.fromJson(json)),
        'subscribed' => SubscribedMessage(SubscribedFrame.fromJson(json)),
        'booking_created' => BookingCreatedMessage(BookingCreatedFrame.fromJson(json)),
        'booking_status' => BookingStatusMessage(BookingStatusFrame.fromJson(json)),
        'booking_eta' => BookingEtaMessage(BookingEtaFrame.fromJson(json)),
        'booking_completed' => BookingCompletedMessage(BookingCompletedFrame.fromJson(json)),
        'booking_cancelled' => BookingCancelledMessage(BookingCancelledFrame.fromJson(json)),
        'driver_location' => DriverLocationMessage(DriverLocationFrame.fromJson(json)),
        'offers_blocked' => OffersBlockedMessage(OffersBlockedFrame.fromJson(json)),
        'offers_unblocked' => OffersUnblockedMessage(OffersUnblockedFrame.fromJson(json)),
        'verification_changed' => VerificationChangedMessage(VerificationChangedFrame.fromJson(json)),
        'search_progress' => SearchProgressMessage(SearchProgressFrame.fromJson(json)),
        'search_exhausted' => SearchExhaustedMessage(SearchExhaustedFrame.fromJson(json)),
        'board_subscribed' => BoardSubscribedMessage(BoardSubscribedFrame.fromJson(json)),
        'board_load_added' => BoardLoadAddedMessage(BoardLoadAddedFrame.fromJson(json)),
        'board_load_updated' => BoardLoadUpdatedMessage(BoardLoadUpdatedFrame.fromJson(json)),
        'board_load_removed' => BoardLoadRemovedMessage(BoardLoadRemovedFrame.fromJson(json)),
        'notification' => NotificationMessage(NotificationFrame.fromJson(json)),
        'session_started' => SessionStartedMessage(SessionStartedFrame.fromJson(json)),
        'kicked' => KickedMessage(KickedFrame.fromJson(json)),
        'error' => ErrorMessage(ErrorFrame.fromJson(json)),
        final String other => UnknownServerFrame(other, json),
        final other => throw FormatException('server frame type must be a string, got: $other'),
      };
}

/// A frame type this version of the package does not know yet; [type] and [data] are kept as they came.
final class UnknownServerFrame extends ServerFrame {
  const UnknownServerFrame(this.type, this.json);
  final String type;
  final Map<String, dynamic> json;
  Object? get data => json['data'];
  @override
  Map<String, dynamic> toJson() => json;
}

final class ConnectedMessage extends ServerFrame {
  const ConnectedMessage(this.frame);
  final ConnectedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class PongMessage extends ServerFrame {
  const PongMessage(this.frame);
  final PongFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OnlineMessage extends ServerFrame {
  const OnlineMessage(this.frame);
  final OnlineFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OfflineMessage extends ServerFrame {
  const OfflineMessage(this.frame);
  final OfflineFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OfferProposedMessage extends ServerFrame {
  const OfferProposedMessage(this.frame);
  final OfferProposedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OfferResultMessage extends ServerFrame {
  const OfferResultMessage(this.frame);
  final OfferResultFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SubscribedMessage extends ServerFrame {
  const SubscribedMessage(this.frame);
  final SubscribedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BookingCreatedMessage extends ServerFrame {
  const BookingCreatedMessage(this.frame);
  final BookingCreatedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BookingStatusMessage extends ServerFrame {
  const BookingStatusMessage(this.frame);
  final BookingStatusFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BookingEtaMessage extends ServerFrame {
  const BookingEtaMessage(this.frame);
  final BookingEtaFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BookingCompletedMessage extends ServerFrame {
  const BookingCompletedMessage(this.frame);
  final BookingCompletedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BookingCancelledMessage extends ServerFrame {
  const BookingCancelledMessage(this.frame);
  final BookingCancelledFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class DriverLocationMessage extends ServerFrame {
  const DriverLocationMessage(this.frame);
  final DriverLocationFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class VerificationChangedMessage extends ServerFrame {
  const VerificationChangedMessage(this.frame);
  final VerificationChangedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OffersBlockedMessage extends ServerFrame {
  const OffersBlockedMessage(this.frame);
  final OffersBlockedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OffersUnblockedMessage extends ServerFrame {
  const OffersUnblockedMessage(this.frame);
  final OffersUnblockedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SearchProgressMessage extends ServerFrame {
  const SearchProgressMessage(this.frame);
  final SearchProgressFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SearchExhaustedMessage extends ServerFrame {
  const SearchExhaustedMessage(this.frame);
  final SearchExhaustedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BoardSubscribedMessage extends ServerFrame {
  const BoardSubscribedMessage(this.frame);
  final BoardSubscribedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BoardLoadAddedMessage extends ServerFrame {
  const BoardLoadAddedMessage(this.frame);
  final BoardLoadAddedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BoardLoadUpdatedMessage extends ServerFrame {
  const BoardLoadUpdatedMessage(this.frame);
  final BoardLoadUpdatedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class BoardLoadRemovedMessage extends ServerFrame {
  const BoardLoadRemovedMessage(this.frame);
  final BoardLoadRemovedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class NotificationMessage extends ServerFrame {
  const NotificationMessage(this.frame);
  final NotificationFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class SessionStartedMessage extends ServerFrame {
  const SessionStartedMessage(this.frame);
  final SessionStartedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class KickedMessage extends ServerFrame {
  const KickedMessage(this.frame);
  final KickedFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class ErrorMessage extends ServerFrame {
  const ErrorMessage(this.frame);
  final ErrorFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}
