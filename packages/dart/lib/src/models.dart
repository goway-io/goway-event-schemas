// ignore_for_file: constant_identifier_names
// To parse this JSON data, do
//
//     final authFrame = authFrameFromJson(jsonString);
//     final goOfflineFrame = goOfflineFrameFromJson(jsonString);
//     final goOnlineFrame = goOnlineFrameFromJson(jsonString);
//     final locationFrame = locationFrameFromJson(jsonString);
//     final offerResponseFrame = offerResponseFrameFromJson(jsonString);
//     final pingFrame = pingFrameFromJson(jsonString);
//     final subscribeBoardFrame = subscribeBoardFrameFromJson(jsonString);
//     final subscribeBookingFrame = subscribeBookingFrameFromJson(jsonString);
//     final typingFrame = typingFrameFromJson(jsonString);
//     final unsubscribeBoardFrame = unsubscribeBoardFrameFromJson(jsonString);
//     final unsubscribeBookingFrame = unsubscribeBookingFrameFromJson(jsonString);
//     final boardLoadAddedFrame = boardLoadAddedFrameFromJson(jsonString);
//     final boardLoadRemovedFrame = boardLoadRemovedFrameFromJson(jsonString);
//     final boardLoadUpdatedFrame = boardLoadUpdatedFrameFromJson(jsonString);
//     final boardSubscribedFrame = boardSubscribedFrameFromJson(jsonString);
//     final bookingCancelledFrame = bookingCancelledFrameFromJson(jsonString);
//     final bookingCompletedFrame = bookingCompletedFrameFromJson(jsonString);
//     final bookingCreatedFrame = bookingCreatedFrameFromJson(jsonString);
//     final bookingEtaFrame = bookingEtaFrameFromJson(jsonString);
//     final bookingStatusFrame = bookingStatusFrameFromJson(jsonString);
//     final bookingUpdatedFrame = bookingUpdatedFrameFromJson(jsonString);
//     final chatMessageFrame = chatMessageFrameFromJson(jsonString);
//     final chatReadFrame = chatReadFrameFromJson(jsonString);
//     final chatTypingFrame = chatTypingFrameFromJson(jsonString);
//     final connectedFrame = connectedFrameFromJson(jsonString);
//     final driverLocationFrame = driverLocationFrameFromJson(jsonString);
//     final errorFrame = errorFrameFromJson(jsonString);
//     final kickedFrame = kickedFrameFromJson(jsonString);
//     final notificationFrame = notificationFrameFromJson(jsonString);
//     final offerProposedFrame = offerProposedFrameFromJson(jsonString);
//     final offerResultFrame = offerResultFrameFromJson(jsonString);
//     final offersBlockedFrame = offersBlockedFrameFromJson(jsonString);
//     final offersUnblockedFrame = offersUnblockedFrameFromJson(jsonString);
//     final offlineFrame = offlineFrameFromJson(jsonString);
//     final onlineFrame = onlineFrameFromJson(jsonString);
//     final pongFrame = pongFrameFromJson(jsonString);
//     final searchExhaustedFrame = searchExhaustedFrameFromJson(jsonString);
//     final searchProgressFrame = searchProgressFrameFromJson(jsonString);
//     final sessionStartedFrame = sessionStartedFrameFromJson(jsonString);
//     final subscribedFrame = subscribedFrameFromJson(jsonString);
//     final supportMessageFrame = supportMessageFrameFromJson(jsonString);
//     final verificationChangedFrame = verificationChangedFrameFromJson(jsonString);

import 'package:json_annotation/json_annotation.dart';
import 'dart:convert';

part 'models.g.dart';

AuthFrame authFrameFromJson(String str) => AuthFrame.fromJson(json.decode(str));

String authFrameToJson(AuthFrame data) => json.encode(data.toJson());

GoOfflineFrame goOfflineFrameFromJson(String str) => GoOfflineFrame.fromJson(json.decode(str));

String goOfflineFrameToJson(GoOfflineFrame data) => json.encode(data.toJson());

GoOnlineFrame goOnlineFrameFromJson(String str) => GoOnlineFrame.fromJson(json.decode(str));

String goOnlineFrameToJson(GoOnlineFrame data) => json.encode(data.toJson());

LocationFrame locationFrameFromJson(String str) => LocationFrame.fromJson(json.decode(str));

String locationFrameToJson(LocationFrame data) => json.encode(data.toJson());

OfferResponseFrame offerResponseFrameFromJson(String str) => OfferResponseFrame.fromJson(json.decode(str));

String offerResponseFrameToJson(OfferResponseFrame data) => json.encode(data.toJson());

PingFrame pingFrameFromJson(String str) => PingFrame.fromJson(json.decode(str));

String pingFrameToJson(PingFrame data) => json.encode(data.toJson());

SubscribeBoardFrame subscribeBoardFrameFromJson(String str) => SubscribeBoardFrame.fromJson(json.decode(str));

String subscribeBoardFrameToJson(SubscribeBoardFrame data) => json.encode(data.toJson());

SubscribeBookingFrame subscribeBookingFrameFromJson(String str) => SubscribeBookingFrame.fromJson(json.decode(str));

String subscribeBookingFrameToJson(SubscribeBookingFrame data) => json.encode(data.toJson());

TypingFrame typingFrameFromJson(String str) => TypingFrame.fromJson(json.decode(str));

String typingFrameToJson(TypingFrame data) => json.encode(data.toJson());

UnsubscribeBoardFrame unsubscribeBoardFrameFromJson(String str) => UnsubscribeBoardFrame.fromJson(json.decode(str));

String unsubscribeBoardFrameToJson(UnsubscribeBoardFrame data) => json.encode(data.toJson());

UnsubscribeBookingFrame unsubscribeBookingFrameFromJson(String str) => UnsubscribeBookingFrame.fromJson(json.decode(str));

String unsubscribeBookingFrameToJson(UnsubscribeBookingFrame data) => json.encode(data.toJson());

BoardLoadAddedFrame boardLoadAddedFrameFromJson(String str) => BoardLoadAddedFrame.fromJson(json.decode(str));

String boardLoadAddedFrameToJson(BoardLoadAddedFrame data) => json.encode(data.toJson());

BoardLoadRemovedFrame boardLoadRemovedFrameFromJson(String str) => BoardLoadRemovedFrame.fromJson(json.decode(str));

String boardLoadRemovedFrameToJson(BoardLoadRemovedFrame data) => json.encode(data.toJson());

BoardLoadUpdatedFrame boardLoadUpdatedFrameFromJson(String str) => BoardLoadUpdatedFrame.fromJson(json.decode(str));

String boardLoadUpdatedFrameToJson(BoardLoadUpdatedFrame data) => json.encode(data.toJson());

BoardSubscribedFrame boardSubscribedFrameFromJson(String str) => BoardSubscribedFrame.fromJson(json.decode(str));

String boardSubscribedFrameToJson(BoardSubscribedFrame data) => json.encode(data.toJson());

BookingCancelledFrame bookingCancelledFrameFromJson(String str) => BookingCancelledFrame.fromJson(json.decode(str));

String bookingCancelledFrameToJson(BookingCancelledFrame data) => json.encode(data.toJson());

BookingCompletedFrame bookingCompletedFrameFromJson(String str) => BookingCompletedFrame.fromJson(json.decode(str));

String bookingCompletedFrameToJson(BookingCompletedFrame data) => json.encode(data.toJson());

BookingCreatedFrame bookingCreatedFrameFromJson(String str) => BookingCreatedFrame.fromJson(json.decode(str));

String bookingCreatedFrameToJson(BookingCreatedFrame data) => json.encode(data.toJson());

BookingEtaFrame bookingEtaFrameFromJson(String str) => BookingEtaFrame.fromJson(json.decode(str));

String bookingEtaFrameToJson(BookingEtaFrame data) => json.encode(data.toJson());

BookingStatusFrame bookingStatusFrameFromJson(String str) => BookingStatusFrame.fromJson(json.decode(str));

String bookingStatusFrameToJson(BookingStatusFrame data) => json.encode(data.toJson());

BookingUpdatedFrame bookingUpdatedFrameFromJson(String str) => BookingUpdatedFrame.fromJson(json.decode(str));

String bookingUpdatedFrameToJson(BookingUpdatedFrame data) => json.encode(data.toJson());

ChatMessageFrame chatMessageFrameFromJson(String str) => ChatMessageFrame.fromJson(json.decode(str));

String chatMessageFrameToJson(ChatMessageFrame data) => json.encode(data.toJson());

ChatReadFrame chatReadFrameFromJson(String str) => ChatReadFrame.fromJson(json.decode(str));

String chatReadFrameToJson(ChatReadFrame data) => json.encode(data.toJson());

ChatTypingFrame chatTypingFrameFromJson(String str) => ChatTypingFrame.fromJson(json.decode(str));

String chatTypingFrameToJson(ChatTypingFrame data) => json.encode(data.toJson());

ConnectedFrame connectedFrameFromJson(String str) => ConnectedFrame.fromJson(json.decode(str));

String connectedFrameToJson(ConnectedFrame data) => json.encode(data.toJson());

DriverLocationFrame driverLocationFrameFromJson(String str) => DriverLocationFrame.fromJson(json.decode(str));

String driverLocationFrameToJson(DriverLocationFrame data) => json.encode(data.toJson());

ErrorFrame errorFrameFromJson(String str) => ErrorFrame.fromJson(json.decode(str));

String errorFrameToJson(ErrorFrame data) => json.encode(data.toJson());

KickedFrame kickedFrameFromJson(String str) => KickedFrame.fromJson(json.decode(str));

String kickedFrameToJson(KickedFrame data) => json.encode(data.toJson());

NotificationFrame notificationFrameFromJson(String str) => NotificationFrame.fromJson(json.decode(str));

String notificationFrameToJson(NotificationFrame data) => json.encode(data.toJson());

OfferProposedFrame offerProposedFrameFromJson(String str) => OfferProposedFrame.fromJson(json.decode(str));

String offerProposedFrameToJson(OfferProposedFrame data) => json.encode(data.toJson());

OfferResultFrame offerResultFrameFromJson(String str) => OfferResultFrame.fromJson(json.decode(str));

String offerResultFrameToJson(OfferResultFrame data) => json.encode(data.toJson());

OffersBlockedFrame offersBlockedFrameFromJson(String str) => OffersBlockedFrame.fromJson(json.decode(str));

String offersBlockedFrameToJson(OffersBlockedFrame data) => json.encode(data.toJson());

OffersUnblockedFrame offersUnblockedFrameFromJson(String str) => OffersUnblockedFrame.fromJson(json.decode(str));

String offersUnblockedFrameToJson(OffersUnblockedFrame data) => json.encode(data.toJson());

OfflineFrame offlineFrameFromJson(String str) => OfflineFrame.fromJson(json.decode(str));

String offlineFrameToJson(OfflineFrame data) => json.encode(data.toJson());

OnlineFrame onlineFrameFromJson(String str) => OnlineFrame.fromJson(json.decode(str));

String onlineFrameToJson(OnlineFrame data) => json.encode(data.toJson());

PongFrame pongFrameFromJson(String str) => PongFrame.fromJson(json.decode(str));

String pongFrameToJson(PongFrame data) => json.encode(data.toJson());

SearchExhaustedFrame searchExhaustedFrameFromJson(String str) => SearchExhaustedFrame.fromJson(json.decode(str));

String searchExhaustedFrameToJson(SearchExhaustedFrame data) => json.encode(data.toJson());

SearchProgressFrame searchProgressFrameFromJson(String str) => SearchProgressFrame.fromJson(json.decode(str));

String searchProgressFrameToJson(SearchProgressFrame data) => json.encode(data.toJson());

SessionStartedFrame sessionStartedFrameFromJson(String str) => SessionStartedFrame.fromJson(json.decode(str));

String sessionStartedFrameToJson(SessionStartedFrame data) => json.encode(data.toJson());

SubscribedFrame subscribedFrameFromJson(String str) => SubscribedFrame.fromJson(json.decode(str));

String subscribedFrameToJson(SubscribedFrame data) => json.encode(data.toJson());

SupportMessageFrame supportMessageFrameFromJson(String str) => SupportMessageFrame.fromJson(json.decode(str));

String supportMessageFrameToJson(SupportMessageFrame data) => json.encode(data.toJson());

VerificationChangedFrame verificationChangedFrameFromJson(String str) => VerificationChangedFrame.fromJson(json.decode(str));

String verificationChangedFrameToJson(VerificationChangedFrame data) => json.encode(data.toJson());


///Frame `auth` an app sends to goway-realtime.
@JsonSerializable()
class AuthFrame {
    @JsonKey(name: "data")
    final Auth data;
    @JsonKey(name: "type")
    final AuthFrameType type;

    AuthFrame({
        required this.data,
        required this.type,
    });

    factory AuthFrame.fromJson(Map<String, dynamic> json) => _$AuthFrameFromJson(json);

    Map<String, dynamic> toJson() => _$AuthFrameToJson(this);
}


///The first frame on a new socket, within five seconds: the access token.
@JsonSerializable()
class Auth {
    
    ///The app install's id, the one the device was registered under; lets this socket be signed
    ///out on its own when the device is removed.
    @JsonKey(name: "installation_id")
    final String? installationId;
    
    ///Access token (JWT).
    @JsonKey(name: "token")
    final String token;

    Auth({
        this.installationId,
        required this.token,
    });

    factory Auth.fromJson(Map<String, dynamic> json) => _$AuthFromJson(json);

    Map<String, dynamic> toJson() => _$AuthToJson(this);
}

enum AuthFrameType {
    @JsonValue("auth")
    AUTH
}

final authFrameTypeValues = EnumValues({
    "auth": AuthFrameType.AUTH
});


///Frame `go_offline` an app sends to goway-realtime.
@JsonSerializable()
class GoOfflineFrame {
    @JsonKey(name: "data")
    final Map<String, dynamic>? data;
    @JsonKey(name: "type")
    final GoOfflineFrameType type;

    GoOfflineFrame({
        this.data,
        required this.type,
    });

    factory GoOfflineFrame.fromJson(Map<String, dynamic> json) => _$GoOfflineFrameFromJson(json);

    Map<String, dynamic> toJson() => _$GoOfflineFrameToJson(this);
}

enum GoOfflineFrameType {
    @JsonValue("go_offline")
    GO_OFFLINE
}

final goOfflineFrameTypeValues = EnumValues({
    "go_offline": GoOfflineFrameType.GO_OFFLINE
});


///Frame `go_online` an app sends to goway-realtime.
@JsonSerializable()
class GoOnlineFrame {
    @JsonKey(name: "data")
    final Map<String, dynamic> data;
    @JsonKey(name: "type")
    final GoOnlineFrameType type;

    GoOnlineFrame({
        required this.data,
        required this.type,
    });

    factory GoOnlineFrame.fromJson(Map<String, dynamic> json) => _$GoOnlineFrameFromJson(json);

    Map<String, dynamic> toJson() => _$GoOnlineFrameToJson(this);
}

enum GoOnlineFrameType {
    @JsonValue("go_online")
    GO_ONLINE
}

final goOnlineFrameTypeValues = EnumValues({
    "go_online": GoOnlineFrameType.GO_ONLINE
});


///Frame `location` an app sends to goway-realtime.
@JsonSerializable()
class LocationFrame {
    @JsonKey(name: "data")
    final Location data;
    @JsonKey(name: "type")
    final LocationFrameType type;

    LocationFrame({
        required this.data,
        required this.type,
    });

    factory LocationFrame.fromJson(Map<String, dynamic> json) => _$LocationFrameFromJson(json);

    Map<String, dynamic> toJson() => _$LocationFrameToJson(this);
}


///The driver's position. Relayed to booking viewers and sampled as evidence only while a
///booking is active.
@JsonSerializable()
class Location {
    
    ///Degrees from north, when the device knows it.
    @JsonKey(name: "heading")
    final double? heading;
    @JsonKey(name: "lat")
    final double lat;
    @JsonKey(name: "lng")
    final double lng;
    
    ///Speed the device reports, when it knows it.
    @JsonKey(name: "speed")
    final double? speed;

    Location({
        this.heading,
        required this.lat,
        required this.lng,
        this.speed,
    });

    factory Location.fromJson(Map<String, dynamic> json) => _$LocationFromJson(json);

    Map<String, dynamic> toJson() => _$LocationToJson(this);
}

enum LocationFrameType {
    @JsonValue("location")
    LOCATION
}

final locationFrameTypeValues = EnumValues({
    "location": LocationFrameType.LOCATION
});


///Frame `offer_response` an app sends to goway-realtime.
@JsonSerializable()
class OfferResponseFrame {
    @JsonKey(name: "data")
    final OfferResponse data;
    @JsonKey(name: "type")
    final OfferResponseFrameType type;

    OfferResponseFrame({
        required this.data,
        required this.type,
    });

    factory OfferResponseFrame.fromJson(Map<String, dynamic> json) => _$OfferResponseFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OfferResponseFrameToJson(this);
}


///The driver's answer to an instant offer.
@JsonSerializable()
class OfferResponse {
    @JsonKey(name: "action")
    final OfferAction action;
    @JsonKey(name: "load_id")
    final String loadId;

    OfferResponse({
        required this.action,
        required this.loadId,
    });

    factory OfferResponse.fromJson(Map<String, dynamic> json) => _$OfferResponseFromJson(json);

    Map<String, dynamic> toJson() => _$OfferResponseToJson(this);
}

enum OfferAction {
    @JsonValue("accept")
    ACCEPT,
    @JsonValue("decline")
    DECLINE
}

final offerActionValues = EnumValues({
    "accept": OfferAction.ACCEPT,
    "decline": OfferAction.DECLINE
});

enum OfferResponseFrameType {
    @JsonValue("offer_response")
    OFFER_RESPONSE
}

final offerResponseFrameTypeValues = EnumValues({
    "offer_response": OfferResponseFrameType.OFFER_RESPONSE
});


///Frame `ping` an app sends to goway-realtime.
@JsonSerializable()
class PingFrame {
    @JsonKey(name: "data")
    final Map<String, dynamic>? data;
    @JsonKey(name: "type")
    final PingFrameType type;

    PingFrame({
        this.data,
        required this.type,
    });

    factory PingFrame.fromJson(Map<String, dynamic> json) => _$PingFrameFromJson(json);

    Map<String, dynamic> toJson() => _$PingFrameToJson(this);
}

enum PingFrameType {
    @JsonValue("ping")
    PING
}

final pingFrameTypeValues = EnumValues({
    "ping": PingFrameType.PING
});


///Frame `subscribe_board` an app sends to goway-realtime.
@JsonSerializable()
class SubscribeBoardFrame {
    @JsonKey(name: "data")
    final SubscribeBoard data;
    @JsonKey(name: "type")
    final SubscribeBoardFrameType type;

    SubscribeBoardFrame({
        required this.data,
        required this.type,
    });

    factory SubscribeBoardFrame.fromJson(Map<String, dynamic> json) => _$SubscribeBoardFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribeBoardFrameToJson(this);
}


///Hear when loads on a lane appear, change or leave the board, for a live plan list. One
///direction per frame; a driver with a way back subscribes to both.
@JsonSerializable()
class SubscribeBoard {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    SubscribeBoard({
        required this.fromRegionId,
        required this.toRegionId,
    });

    factory SubscribeBoard.fromJson(Map<String, dynamic> json) => _$SubscribeBoardFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribeBoardToJson(this);
}

enum SubscribeBoardFrameType {
    @JsonValue("subscribe_board")
    SUBSCRIBE_BOARD
}

final subscribeBoardFrameTypeValues = EnumValues({
    "subscribe_board": SubscribeBoardFrameType.SUBSCRIBE_BOARD
});


///Frame `subscribe_booking` an app sends to goway-realtime.
@JsonSerializable()
class SubscribeBookingFrame {
    @JsonKey(name: "data")
    final SubscribeBooking data;
    @JsonKey(name: "type")
    final SubscribeBookingFrameType type;

    SubscribeBookingFrame({
        required this.data,
        required this.type,
    });

    factory SubscribeBookingFrame.fromJson(Map<String, dynamic> json) => _$SubscribeBookingFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribeBookingFrameToJson(this);
}


///Watch a booking of the caller's organization: status, arrival window and the driver's
///position. The driver and the shipper who posted the load get every frame but the position
///without subscribing; subscribe for the position, or as another member of either
///organization.
@JsonSerializable()
class SubscribeBooking {
    @JsonKey(name: "booking_id")
    final String bookingId;

    SubscribeBooking({
        required this.bookingId,
    });

    factory SubscribeBooking.fromJson(Map<String, dynamic> json) => _$SubscribeBookingFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribeBookingToJson(this);
}

enum SubscribeBookingFrameType {
    @JsonValue("subscribe_booking")
    SUBSCRIBE_BOOKING
}

final subscribeBookingFrameTypeValues = EnumValues({
    "subscribe_booking": SubscribeBookingFrameType.SUBSCRIBE_BOOKING
});


///Frame `typing` an app sends to goway-realtime.
@JsonSerializable()
class TypingFrame {
    @JsonKey(name: "data")
    final Typing data;
    @JsonKey(name: "type")
    final TypingFrameType type;

    TypingFrame({
        required this.data,
        required this.type,
    });

    factory TypingFrame.fromJson(Map<String, dynamic> json) => _$TypingFrameFromJson(json);

    Map<String, dynamic> toJson() => _$TypingFrameToJson(this);
}


///The caller is typing in a haul's chat; sent at most every few seconds while typing.
@JsonSerializable()
class Typing {
    @JsonKey(name: "booking_id")
    final String bookingId;

    Typing({
        required this.bookingId,
    });

    factory Typing.fromJson(Map<String, dynamic> json) => _$TypingFromJson(json);

    Map<String, dynamic> toJson() => _$TypingToJson(this);
}

enum TypingFrameType {
    @JsonValue("typing")
    TYPING
}

final typingFrameTypeValues = EnumValues({
    "typing": TypingFrameType.TYPING
});


///Frame `unsubscribe_board` an app sends to goway-realtime.
@JsonSerializable()
class UnsubscribeBoardFrame {
    @JsonKey(name: "data")
    final UnsubscribeBoard data;
    @JsonKey(name: "type")
    final UnsubscribeBoardFrameType type;

    UnsubscribeBoardFrame({
        required this.data,
        required this.type,
    });

    factory UnsubscribeBoardFrame.fromJson(Map<String, dynamic> json) => _$UnsubscribeBoardFrameFromJson(json);

    Map<String, dynamic> toJson() => _$UnsubscribeBoardFrameToJson(this);
}


///Stop hearing about a lane.
@JsonSerializable()
class UnsubscribeBoard {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    UnsubscribeBoard({
        required this.fromRegionId,
        required this.toRegionId,
    });

    factory UnsubscribeBoard.fromJson(Map<String, dynamic> json) => _$UnsubscribeBoardFromJson(json);

    Map<String, dynamic> toJson() => _$UnsubscribeBoardToJson(this);
}

enum UnsubscribeBoardFrameType {
    @JsonValue("unsubscribe_board")
    UNSUBSCRIBE_BOARD
}

final unsubscribeBoardFrameTypeValues = EnumValues({
    "unsubscribe_board": UnsubscribeBoardFrameType.UNSUBSCRIBE_BOARD
});


///Frame `unsubscribe_booking` an app sends to goway-realtime.
@JsonSerializable()
class UnsubscribeBookingFrame {
    @JsonKey(name: "data")
    final UnsubscribeBooking data;
    @JsonKey(name: "type")
    final UnsubscribeBookingFrameType type;

    UnsubscribeBookingFrame({
        required this.data,
        required this.type,
    });

    factory UnsubscribeBookingFrame.fromJson(Map<String, dynamic> json) => _$UnsubscribeBookingFrameFromJson(json);

    Map<String, dynamic> toJson() => _$UnsubscribeBookingFrameToJson(this);
}


///Stop watching a booking.
@JsonSerializable()
class UnsubscribeBooking {
    @JsonKey(name: "booking_id")
    final String bookingId;

    UnsubscribeBooking({
        required this.bookingId,
    });

    factory UnsubscribeBooking.fromJson(Map<String, dynamic> json) => _$UnsubscribeBookingFromJson(json);

    Map<String, dynamic> toJson() => _$UnsubscribeBookingToJson(this);
}

enum UnsubscribeBookingFrameType {
    @JsonValue("unsubscribe_booking")
    UNSUBSCRIBE_BOOKING
}

final unsubscribeBookingFrameTypeValues = EnumValues({
    "unsubscribe_booking": UnsubscribeBookingFrameType.UNSUBSCRIBE_BOOKING
});


///Frame `board_load_added` goway-realtime sends to an app.
@JsonSerializable()
class BoardLoadAddedFrame {
    @JsonKey(name: "data")
    final BoardLoadAdded data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BoardLoadAddedFrameType type;

    BoardLoadAddedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BoardLoadAddedFrame.fromJson(Map<String, dynamic> json) => _$BoardLoadAddedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadAddedFrameToJson(this);
}


///A load appeared on a subscribed lane: newly posted or back after a cancelled booking. A
///signal; the app reads the list again over REST.
@JsonSerializable()
class BoardLoadAdded {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "plan")
    final bool plan;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    BoardLoadAdded({
        required this.fromRegionId,
        required this.loadId,
        required this.plan,
        required this.toRegionId,
    });

    factory BoardLoadAdded.fromJson(Map<String, dynamic> json) => _$BoardLoadAddedFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadAddedToJson(this);
}

enum BoardLoadAddedFrameType {
    @JsonValue("board_load_added")
    BOARD_LOAD_ADDED
}

final boardLoadAddedFrameTypeValues = EnumValues({
    "board_load_added": BoardLoadAddedFrameType.BOARD_LOAD_ADDED
});


///Frame `board_load_removed` goway-realtime sends to an app.
@JsonSerializable()
class BoardLoadRemovedFrame {
    @JsonKey(name: "data")
    final BoardLoadRemoved data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BoardLoadRemovedFrameType type;

    BoardLoadRemovedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BoardLoadRemovedFrame.fromJson(Map<String, dynamic> json) => _$BoardLoadRemovedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadRemovedFrameToJson(this);
}


///A load left a subscribed lane's board. A signal; the app drops it from the list.
@JsonSerializable()
class BoardLoadRemoved {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "reason")
    final BoardRemovalReason reason;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    BoardLoadRemoved({
        required this.fromRegionId,
        required this.loadId,
        required this.reason,
        required this.toRegionId,
    });

    factory BoardLoadRemoved.fromJson(Map<String, dynamic> json) => _$BoardLoadRemovedFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadRemovedToJson(this);
}

enum BoardRemovalReason {
    @JsonValue("booked")
    BOOKED,
    @JsonValue("cancelled")
    CANCELLED,
    @JsonValue("expired")
    EXPIRED
}

final boardRemovalReasonValues = EnumValues({
    "booked": BoardRemovalReason.BOOKED,
    "cancelled": BoardRemovalReason.CANCELLED,
    "expired": BoardRemovalReason.EXPIRED
});

enum BoardLoadRemovedFrameType {
    @JsonValue("board_load_removed")
    BOARD_LOAD_REMOVED
}

final boardLoadRemovedFrameTypeValues = EnumValues({
    "board_load_removed": BoardLoadRemovedFrameType.BOARD_LOAD_REMOVED
});


///Frame `board_load_updated` goway-realtime sends to an app.
@JsonSerializable()
class BoardLoadUpdatedFrame {
    @JsonKey(name: "data")
    final BoardLoadUpdated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BoardLoadUpdatedFrameType type;

    BoardLoadUpdatedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BoardLoadUpdatedFrame.fromJson(Map<String, dynamic> json) => _$BoardLoadUpdatedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadUpdatedFrameToJson(this);
}


///A load on a subscribed lane was edited: price, pickup window or note. A signal; the app
///reads it again over REST.
@JsonSerializable()
class BoardLoadUpdated {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "plan")
    final bool plan;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    BoardLoadUpdated({
        required this.fromRegionId,
        required this.loadId,
        required this.plan,
        required this.toRegionId,
    });

    factory BoardLoadUpdated.fromJson(Map<String, dynamic> json) => _$BoardLoadUpdatedFromJson(json);

    Map<String, dynamic> toJson() => _$BoardLoadUpdatedToJson(this);
}

enum BoardLoadUpdatedFrameType {
    @JsonValue("board_load_updated")
    BOARD_LOAD_UPDATED
}

final boardLoadUpdatedFrameTypeValues = EnumValues({
    "board_load_updated": BoardLoadUpdatedFrameType.BOARD_LOAD_UPDATED
});


///Frame `board_subscribed` goway-realtime sends to an app.
@JsonSerializable()
class BoardSubscribedFrame {
    @JsonKey(name: "data")
    final BoardSubscribed data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BoardSubscribedFrameType type;

    BoardSubscribedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BoardSubscribedFrame.fromJson(Map<String, dynamic> json) => _$BoardSubscribedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BoardSubscribedFrameToJson(this);
}


///`subscribe_board` was accepted.
@JsonSerializable()
class BoardSubscribed {
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "to_region_id")
    final String toRegionId;

    BoardSubscribed({
        required this.fromRegionId,
        required this.toRegionId,
    });

    factory BoardSubscribed.fromJson(Map<String, dynamic> json) => _$BoardSubscribedFromJson(json);

    Map<String, dynamic> toJson() => _$BoardSubscribedToJson(this);
}

enum BoardSubscribedFrameType {
    @JsonValue("board_subscribed")
    BOARD_SUBSCRIBED
}

final boardSubscribedFrameTypeValues = EnumValues({
    "board_subscribed": BoardSubscribedFrameType.BOARD_SUBSCRIBED
});


///Frame `booking_cancelled` goway-realtime sends to an app. Sent to the driver, to the
///shipper who posted the load (no subscription needed) and to the booking's viewers. It
///currently forwards the internal `booking.cancelled` event as-is, so `data` is that
///event's payload, `event_id` and organization ids included.
@JsonSerializable()
class BookingCancelledFrame {
    @JsonKey(name: "data")
    final BookingCancelled data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingCancelledFrameType type;

    BookingCancelledFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingCancelledFrame.fromJson(Map<String, dynamic> json) => _$BookingCancelledFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCancelledFrameToJson(this);
}


///A booking ended without being completed: either side cancelled before the cargo was
///loaded (`by` is `shipper` or `carrier`), or an operator cancelled it (`by` is
///`operator`), which after loading happens only through a reported issue. The load goes
///back to the board unless the operator withdrew it, which a `load.cancelled` then says.
@JsonSerializable()
class BookingCancelled {
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "by")
    final CancelledBy by;
    @JsonKey(name: "carrier_org_id")
    final String carrierOrgId;
    @JsonKey(name: "currency")
    final String currency;
    
    ///Null when no driver was assigned.
    @JsonKey(name: "driver_user_id")
    final String? driverUserId;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    
    ///Cancellation fee the shipper owes the carrier, whole so'm (`cancel_fee_percent` once the
    ///driver set off); null when none.
    @JsonKey(name: "fee_amount")
    final int? feeAmount;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///The cancellation counts against the driver's acceptance rate: the driver's own
    ///cancellation of an instant haul, or of a plan later than goway-loads' free-cancel window
    ///before pickup, or an operator's cancellation of a booking whose driver never showed up.
    ///False for every other shipper and operator cancellation.
    @JsonKey(name: "penalized")
    final bool penalized;
    
    ///The cancelled booking was a plan.
    @JsonKey(name: "plan")
    final bool plan;
    
    ///What the canceller wrote; empty when nothing was written.
    @JsonKey(name: "reason")
    final String reason;
    
    ///Code from the cancellation reasons catalogue; null for older clients and system
    ///cancellations.
    @JsonKey(name: "reason_code")
    final String? reasonCode;
    @JsonKey(name: "shipper_org_id")
    final String shipperOrgId;

    BookingCancelled({
        required this.bookingId,
        required this.by,
        required this.carrierOrgId,
        required this.currency,
        required this.driverUserId,
        required this.eventId,
        required this.feeAmount,
        required this.loadId,
        required this.penalized,
        required this.plan,
        required this.reason,
        required this.reasonCode,
        required this.shipperOrgId,
    });

    factory BookingCancelled.fromJson(Map<String, dynamic> json) => _$BookingCancelledFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCancelledToJson(this);
}


///Who cancelled.
enum CancelledBy {
    @JsonValue("carrier")
    CARRIER,
    @JsonValue("operator")
    OPERATOR,
    @JsonValue("shipper")
    SHIPPER
}

final cancelledByValues = EnumValues({
    "carrier": CancelledBy.CARRIER,
    "operator": CancelledBy.OPERATOR,
    "shipper": CancelledBy.SHIPPER
});

enum BookingCancelledFrameType {
    @JsonValue("booking_cancelled")
    BOOKING_CANCELLED
}

final bookingCancelledFrameTypeValues = EnumValues({
    "booking_cancelled": BookingCancelledFrameType.BOOKING_CANCELLED
});


///Frame `booking_completed` goway-realtime sends to an app. Sent to the driver, to the
///shipper who posted the load (no subscription needed) and to the booking's viewers. It
///currently forwards the internal `booking.completed` event as-is, so `data` is that
///event's payload, `event_id` and organization ids included.
@JsonSerializable()
class BookingCompletedFrame {
    @JsonKey(name: "data")
    final BookingCompleted data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingCompletedFrameType type;

    BookingCompletedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingCompletedFrame.fromJson(Map<String, dynamic> json) => _$BookingCompletedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCompletedFrameToJson(this);
}


///A delivered haul was closed: by the shipper, by an operator, or by the auto-complete
///sweep. All three send the same fields. Billing charges its commission from this event.
@JsonSerializable()
class BookingCompleted {
    
    ///Price agreed at booking, whole so'm (a bare integer; `currency` is UZS today).
    @JsonKey(name: "agreed_amount")
    final int agreedAmount;
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "carrier_org_id")
    final String carrierOrgId;
    
    ///Price lines; the first is always the agreed tariff.
    @JsonKey(name: "charges")
    final List<ChargeLine> charges;
    
    ///When the haul was closed; the code allows null, it is always set. ISO 8601 with offset,
    ///as Python's `isoformat()` writes it.
    @JsonKey(name: "completed_at")
    final DateTime? completedAt;
    @JsonKey(name: "completed_by")
    final CompletedBy completedBy;
    
    ///ISO 4217 code; `UZS` today.
    @JsonKey(name: "currency")
    final String currency;
    @JsonKey(name: "driver_user_id")
    final String driverUserId;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///What the haul actually settled at: the agreed price unless another sum was confirmed,
    ///whole so'm (a bare integer; `currency` is UZS today).
    @JsonKey(name: "settled_amount")
    final int settledAmount;
    @JsonKey(name: "shipper_org_id")
    final String shipperOrgId;
    
    ///Charges plus waiting, whole so'm (a bare integer; `currency` is UZS today); the code
    ///allows null.
    @JsonKey(name: "total_amount")
    final int? totalAmount;
    
    ///What the waiting costs, whole so'm (a bare integer; `currency` is UZS today).
    @JsonKey(name: "waiting_amount")
    final int waitingAmount;
    
    ///Whether the waiting is charged to the shipper.
    @JsonKey(name: "waiting_billable")
    final bool waitingBillable;
    
    ///Billable minutes the driver waited at pickup.
    @JsonKey(name: "waiting_minutes")
    final int waitingMinutes;

    BookingCompleted({
        required this.agreedAmount,
        required this.bookingId,
        required this.carrierOrgId,
        required this.charges,
        required this.completedAt,
        required this.completedBy,
        required this.currency,
        required this.driverUserId,
        required this.eventId,
        required this.loadId,
        required this.settledAmount,
        required this.shipperOrgId,
        required this.totalAmount,
        required this.waitingAmount,
        required this.waitingBillable,
        required this.waitingMinutes,
    });

    factory BookingCompleted.fromJson(Map<String, dynamic> json) => _$BookingCompletedFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCompletedToJson(this);
}


///One priced line of the haul.
@JsonSerializable()
class ChargeLine {
    
    ///Line total, whole so'm (a bare integer; `currency` is UZS today).
    @JsonKey(name: "amount")
    final int amount;
    
    ///`tariff`, `loaders` or `req:<requirement code>`.
    @JsonKey(name: "code")
    final String code;
    @JsonKey(name: "qty")
    final int qty;

    ChargeLine({
        required this.amount,
        required this.code,
        required this.qty,
    });

    factory ChargeLine.fromJson(Map<String, dynamic> json) => _$ChargeLineFromJson(json);

    Map<String, dynamic> toJson() => _$ChargeLineToJson(this);
}


///Who closed the haul: `carrier` when the driver finishes it at the drop-off, `system` for
///the auto-complete sweep.
enum CompletedBy {
    @JsonValue("carrier")
    CARRIER,
    @JsonValue("operator")
    OPERATOR,
    @JsonValue("shipper")
    SHIPPER,
    @JsonValue("system")
    SYSTEM
}

final completedByValues = EnumValues({
    "carrier": CompletedBy.CARRIER,
    "operator": CompletedBy.OPERATOR,
    "shipper": CompletedBy.SHIPPER,
    "system": CompletedBy.SYSTEM
});

enum BookingCompletedFrameType {
    @JsonValue("booking_completed")
    BOOKING_COMPLETED
}

final bookingCompletedFrameTypeValues = EnumValues({
    "booking_completed": BookingCompletedFrameType.BOOKING_COMPLETED
});


///Frame `booking_created` goway-realtime sends to an app. Sent to the booked driver and to
///the shipper who posted the load. It currently forwards the internal `booking.created`
///event as-is, so `data` is that event's payload, `event_id` and organization ids included.
@JsonSerializable()
class BookingCreatedFrame {
    @JsonKey(name: "data")
    final BookingCreated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingCreatedFrameType type;

    BookingCreatedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingCreatedFrame.fromJson(Map<String, dynamic> json) => _$BookingCreatedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCreatedFrameToJson(this);
}


///A load was booked. Two paths: an accepted offer, or an instant claim from dispatch, which
///adds `source: instant`.
@JsonSerializable()
class BookingCreated {
    
    ///Agreed price, whole so'm (a bare integer; `currency` is UZS today).
    @JsonKey(name: "agreed_amount")
    final int agreedAmount;
    @JsonKey(name: "booking_id")
    final String bookingId;
    
    ///Start of the span this booking keeps the driver busy, goway-loads' plan buffer included.
    ///Dispatch offers the driver nothing whose trip overlaps it.
    @JsonKey(name: "busy_from")
    final DateTime busyFrom;
    
    ///End of that span; null when the haul's duration is unknown, which keeps the driver busy
    ///until the booking ends.
    @JsonKey(name: "busy_until")
    final DateTime? busyUntil;
    @JsonKey(name: "carrier_org_id")
    final String carrierOrgId;
    
    ///ISO 4217 code; `UZS` today.
    @JsonKey(name: "currency")
    final String currency;
    
    ///The driver; the offer path allows null, a booking always has one.
    @JsonKey(name: "driver_user_id")
    final String? driverUserId;
    
    ///Driving time of the haul in minutes; null when routing was unavailable.
    @JsonKey(name: "duration_min")
    final int? durationMin;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///Earliest pickup; null when not given. A pickup well ahead makes this booking a plan
    ///rather than a trip under way.
    @JsonKey(name: "pickup_from")
    final DateTime? pickupFrom;
    
    ///Latest pickup; null when not given.
    @JsonKey(name: "pickup_to")
    final DateTime? pickupTo;
    
    ///The driver booked a plan: a load scheduled ahead, taken from the plan list.
    @JsonKey(name: "plan")
    final bool plan;
    @JsonKey(name: "shipper_org_id")
    final String shipperOrgId;
    
    ///The person who posted the load; they hear `booking_created` on the socket too.
    @JsonKey(name: "shipper_user_id")
    final String shipperUserId;
    @JsonKey(name: "source")
    final BookingSource? source;

    BookingCreated({
        required this.agreedAmount,
        required this.bookingId,
        required this.busyFrom,
        required this.busyUntil,
        required this.carrierOrgId,
        required this.currency,
        required this.driverUserId,
        required this.durationMin,
        required this.eventId,
        required this.loadId,
        required this.pickupFrom,
        required this.pickupTo,
        required this.plan,
        required this.shipperOrgId,
        required this.shipperUserId,
        this.source,
    });

    factory BookingCreated.fromJson(Map<String, dynamic> json) => _$BookingCreatedFromJson(json);

    Map<String, dynamic> toJson() => _$BookingCreatedToJson(this);
}


///Present only for an instant claim from dispatch; an accepted offer leaves it out.
enum BookingSource {
    @JsonValue("instant")
    INSTANT
}

final bookingSourceValues = EnumValues({
    "instant": BookingSource.INSTANT
});

enum BookingCreatedFrameType {
    @JsonValue("booking_created")
    BOOKING_CREATED
}

final bookingCreatedFrameTypeValues = EnumValues({
    "booking_created": BookingCreatedFrameType.BOOKING_CREATED
});


///Frame `booking_eta` goway-realtime sends to an app. Sent to the driver, the shipper who
///posted the load and the booking's viewers, and dropped when a socket is slow. It
///currently forwards the internal `booking.eta_updated` event as-is, so `data` is that
///event's payload, `event_id` and organization ids included.
@JsonSerializable()
class BookingEtaFrame {
    @JsonKey(name: "data")
    final BookingEtaUpdated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingEtaFrameType type;

    BookingEtaFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingEtaFrame.fromJson(Map<String, dynamic> json) => _$BookingEtaFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingEtaFrameToJson(this);
}


///The arrival window or the time the truck is free again moved: when the haul is booked
///(`confirmed`), at every carrier step, and on fresh GPS while `en_route` or `in_transit`.
@JsonSerializable()
class BookingEtaUpdated {
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "carrier_org_id")
    final String carrierOrgId;
    @JsonKey(name: "driver_user_id")
    final String driverUserId;
    
    ///Earliest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's
    ///`isoformat()` writes it.
    @JsonKey(name: "eta_from")
    final DateTime? etaFrom;
    
    ///Which stop the window is for; null when no arrival is expected any more.
    @JsonKey(name: "eta_stop")
    final EtaStop? etaStop;
    
    ///Latest expected arrival at `eta_stop`; null with it. ISO 8601 with offset, as Python's
    ///`isoformat()` writes it.
    @JsonKey(name: "eta_to")
    final DateTime? etaTo;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    
    ///When the truck is expected to be empty again. ISO 8601 with offset, as Python's
    ///`isoformat()` writes it.
    @JsonKey(name: "free_at")
    final DateTime? freeAt;
    
    ///Where the truck will be free: the drop-off region; null when unknown.
    @JsonKey(name: "free_region_id")
    final String? freeRegionId;
    
    ///The last region boundary the truck crossed, for «Angrendan o'tdi · 142 km qoldi»; null
    ///until it crosses one.
    @JsonKey(name: "last_waypoint")
    final PassedWaypoint? lastWaypoint;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///Road kilometres left to the next stop; null before the haul sets off or while routing is
    ///unavailable.
    @JsonKey(name: "remaining_km")
    final double? remainingKm;
    @JsonKey(name: "shipper_org_id")
    final String shipperOrgId;
    @JsonKey(name: "status")
    final BookingStatus status;
    
    ///The booked vehicle's truck class code.
    @JsonKey(name: "truck_class")
    final String truckClass;

    BookingEtaUpdated({
        required this.bookingId,
        required this.carrierOrgId,
        required this.driverUserId,
        required this.etaFrom,
        required this.etaStop,
        required this.etaTo,
        required this.eventId,
        required this.freeAt,
        required this.freeRegionId,
        required this.lastWaypoint,
        required this.loadId,
        required this.remainingKm,
        required this.shipperOrgId,
        required this.status,
        required this.truckClass,
    });

    factory BookingEtaUpdated.fromJson(Map<String, dynamic> json) => _$BookingEtaUpdatedFromJson(json);

    Map<String, dynamic> toJson() => _$BookingEtaUpdatedToJson(this);
}

enum EtaStop {
    @JsonValue("dropoff")
    DROPOFF,
    @JsonValue("pickup")
    PICKUP
}

final etaStopValues = EnumValues({
    "dropoff": EtaStop.DROPOFF,
    "pickup": EtaStop.PICKUP
});

@JsonSerializable()
class PassedWaypoint {
    @JsonKey(name: "passed_at")
    final DateTime passedAt;
    @JsonKey(name: "region_id")
    final String regionId;

    PassedWaypoint({
        required this.passedAt,
        required this.regionId,
    });

    factory PassedWaypoint.fromJson(Map<String, dynamic> json) => _$PassedWaypointFromJson(json);

    Map<String, dynamic> toJson() => _$PassedWaypointToJson(this);
}

enum BookingStatus {
    @JsonValue("at_pickup")
    AT_PICKUP,
    @JsonValue("cancelled")
    CANCELLED,
    @JsonValue("completed")
    COMPLETED,
    @JsonValue("confirmed")
    CONFIRMED,
    @JsonValue("delivered")
    DELIVERED,
    @JsonValue("en_route")
    EN_ROUTE,
    @JsonValue("in_transit")
    IN_TRANSIT,
    @JsonValue("loading")
    LOADING
}

final bookingStatusValues = EnumValues({
    "at_pickup": BookingStatus.AT_PICKUP,
    "cancelled": BookingStatus.CANCELLED,
    "completed": BookingStatus.COMPLETED,
    "confirmed": BookingStatus.CONFIRMED,
    "delivered": BookingStatus.DELIVERED,
    "en_route": BookingStatus.EN_ROUTE,
    "in_transit": BookingStatus.IN_TRANSIT,
    "loading": BookingStatus.LOADING
});

enum BookingEtaFrameType {
    @JsonValue("booking_eta")
    BOOKING_ETA
}

final bookingEtaFrameTypeValues = EnumValues({
    "booking_eta": BookingEtaFrameType.BOOKING_ETA
});


///Frame `booking_status` goway-realtime sends to an app. Sent to the driver, to the shipper
///who posted the load (no subscription needed) and to the booking's viewers. It currently
///forwards the internal `booking.status_changed` event as-is, so `data` is that event's
///payload, `event_id` and organization ids included.
@JsonSerializable()
class BookingStatusFrame {
    @JsonKey(name: "data")
    final BookingStatusChanged data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingStatusFrameType type;

    BookingStatusFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingStatusFrame.fromJson(Map<String, dynamic> json) => _$BookingStatusFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingStatusFrameToJson(this);
}


///The carrier moved the haul one step on: `en_route`, `at_pickup`, `loading`, `in_transit`
///or `delivered`. Completion and cancellation have their own subjects.
@JsonSerializable()
class BookingStatusChanged {
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "carrier_org_id")
    final String carrierOrgId;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///When the step happened: the driver's tap, which an app offline for a while sends late.
    ///Steps arrive in order; this is the time to show.
    @JsonKey(name: "occurred_at")
    final DateTime occurredAt;
    @JsonKey(name: "shipper_org_id")
    final String shipperOrgId;
    @JsonKey(name: "status")
    final BookingStatus status;

    BookingStatusChanged({
        required this.bookingId,
        required this.carrierOrgId,
        required this.eventId,
        required this.loadId,
        required this.occurredAt,
        required this.shipperOrgId,
        required this.status,
    });

    factory BookingStatusChanged.fromJson(Map<String, dynamic> json) => _$BookingStatusChangedFromJson(json);

    Map<String, dynamic> toJson() => _$BookingStatusChangedToJson(this);
}

enum BookingStatusFrameType {
    @JsonValue("booking_status")
    BOOKING_STATUS
}

final bookingStatusFrameTypeValues = EnumValues({
    "booking_status": BookingStatusFrameType.BOOKING_STATUS
});


///Frame `booking_updated` goway-realtime sends to an app.
@JsonSerializable()
class BookingUpdatedFrame {
    @JsonKey(name: "data")
    final BookingUpdated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final BookingUpdatedFrameType type;

    BookingUpdatedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory BookingUpdatedFrame.fromJson(Map<String, dynamic> json) => _$BookingUpdatedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$BookingUpdatedFrameToJson(this);
}


///Something on a haul changed besides its step; sent to the driver and the shipper without
///a subscription. A signal: the app reads the booking again.
@JsonSerializable()
class BookingUpdated {
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "change")
    final BookingChange change;
    @JsonKey(name: "issue_id")
    final String? issueId;

    BookingUpdated({
        required this.bookingId,
        required this.change,
        required this.issueId,
    });

    factory BookingUpdated.fromJson(Map<String, dynamic> json) => _$BookingUpdatedFromJson(json);

    Map<String, dynamic> toJson() => _$BookingUpdatedToJson(this);
}

enum BookingChange {
    @JsonValue("issue_opened")
    ISSUE_OPENED,
    @JsonValue("issue_resolved")
    ISSUE_RESOLVED,
    @JsonValue("waiting_started")
    WAITING_STARTED
}

final bookingChangeValues = EnumValues({
    "issue_opened": BookingChange.ISSUE_OPENED,
    "issue_resolved": BookingChange.ISSUE_RESOLVED,
    "waiting_started": BookingChange.WAITING_STARTED
});

enum BookingUpdatedFrameType {
    @JsonValue("booking_updated")
    BOOKING_UPDATED
}

final bookingUpdatedFrameTypeValues = EnumValues({
    "booking_updated": BookingUpdatedFrameType.BOOKING_UPDATED
});


///Frame `chat_message` goway-realtime sends to an app. Sent to the sender and the
///recipients. It forwards the internal `chat.message_created` event as-is, so `data` is
///that event's payload.
@JsonSerializable()
class ChatMessageFrame {
    @JsonKey(name: "data")
    final ChatMessageCreated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final ChatMessageFrameType type;

    ChatMessageFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory ChatMessageFrame.fromJson(Map<String, dynamic> json) => _$ChatMessageFrameFromJson(json);

    Map<String, dynamic> toJson() => _$ChatMessageFrameToJson(this);
}


///A message was written in a haul's chat between the driver and the shipper. Realtime shows
///it at once; notify pushes it to whoever is not looking.
@JsonSerializable()
class ChatMessageCreated {
    @JsonKey(name: "booking_id")
    final String bookingId;
    
    ///The sending app's id for the message, so a resend after a lost reply shows once.
    @JsonKey(name: "client_message_id")
    final String? clientMessageId;
    @JsonKey(name: "created_at")
    final DateTime createdAt;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "kind")
    final ChatMessageKind kind;
    @JsonKey(name: "lat")
    final double? lat;
    @JsonKey(name: "lng")
    final double? lng;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "message_id")
    final String messageId;
    
    ///The `chat_photo` file; the app reads its link over REST.
    @JsonKey(name: "photo_file_id")
    final String? photoFileId;
    
    ///Code from the quick replies catalogue when `kind` is `quick_reply`.
    @JsonKey(name: "quick_reply_code")
    final String? quickReplyCode;
    
    ///People on the other side who should hear it: today the driver or the person who posted
    ///the load.
    @JsonKey(name: "recipient_user_ids")
    final List<String> recipientUserIds;
    @JsonKey(name: "sender_side")
    final ChatSide senderSide;
    @JsonKey(name: "sender_user_id")
    final String senderUserId;
    
    ///The words; for a quick reply, its text in the sender's language. Null for a bare photo or
    ///location.
    @JsonKey(name: "text")
    final String? text;

    ChatMessageCreated({
        required this.bookingId,
        required this.clientMessageId,
        required this.createdAt,
        required this.eventId,
        required this.kind,
        required this.lat,
        required this.lng,
        required this.loadId,
        required this.messageId,
        required this.photoFileId,
        required this.quickReplyCode,
        required this.recipientUserIds,
        required this.senderSide,
        required this.senderUserId,
        required this.text,
    });

    factory ChatMessageCreated.fromJson(Map<String, dynamic> json) => _$ChatMessageCreatedFromJson(json);

    Map<String, dynamic> toJson() => _$ChatMessageCreatedToJson(this);
}

enum ChatMessageKind {
    @JsonValue("location")
    LOCATION,
    @JsonValue("photo")
    PHOTO,
    @JsonValue("quick_reply")
    QUICK_REPLY,
    @JsonValue("text")
    TEXT
}

final chatMessageKindValues = EnumValues({
    "location": ChatMessageKind.LOCATION,
    "photo": ChatMessageKind.PHOTO,
    "quick_reply": ChatMessageKind.QUICK_REPLY,
    "text": ChatMessageKind.TEXT
});


///The side of the haul the person writes for.
enum ChatSide {
    @JsonValue("carrier")
    CARRIER,
    @JsonValue("shipper")
    SHIPPER
}

final chatSideValues = EnumValues({
    "carrier": ChatSide.CARRIER,
    "shipper": ChatSide.SHIPPER
});

enum ChatMessageFrameType {
    @JsonValue("chat_message")
    CHAT_MESSAGE
}

final chatMessageFrameTypeValues = EnumValues({
    "chat_message": ChatMessageFrameType.CHAT_MESSAGE
});


///Frame `chat_read` goway-realtime sends to an app. Sent to the recipients. It forwards the
///internal `chat.read` event as-is.
@JsonSerializable()
class ChatReadFrame {
    @JsonKey(name: "data")
    final ChatRead data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final ChatReadFrameType type;

    ChatReadFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory ChatReadFrame.fromJson(Map<String, dynamic> json) => _$ChatReadFrameFromJson(json);

    Map<String, dynamic> toJson() => _$ChatReadFrameToJson(this);
}


///Someone read a haul's chat up to a message; the other side sees «o'qildi».
@JsonSerializable()
class ChatRead {
    @JsonKey(name: "booking_id")
    final String bookingId;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "last_read_message_id")
    final String lastReadMessageId;
    @JsonKey(name: "read_at")
    final DateTime readAt;
    @JsonKey(name: "reader_side")
    final ChatSide readerSide;
    @JsonKey(name: "reader_user_id")
    final String readerUserId;
    
    ///People on the other side who should hear it: today the driver or the person who posted
    ///the load.
    @JsonKey(name: "recipient_user_ids")
    final List<String> recipientUserIds;

    ChatRead({
        required this.bookingId,
        required this.eventId,
        required this.lastReadMessageId,
        required this.readAt,
        required this.readerSide,
        required this.readerUserId,
        required this.recipientUserIds,
    });

    factory ChatRead.fromJson(Map<String, dynamic> json) => _$ChatReadFromJson(json);

    Map<String, dynamic> toJson() => _$ChatReadToJson(this);
}

enum ChatReadFrameType {
    @JsonValue("chat_read")
    CHAT_READ
}

final chatReadFrameTypeValues = EnumValues({
    "chat_read": ChatReadFrameType.CHAT_READ
});


///Frame `chat_typing` goway-realtime sends to an app.
@JsonSerializable()
class ChatTypingFrame {
    @JsonKey(name: "data")
    final ChatTyping data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final ChatTypingFrameType type;

    ChatTypingFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory ChatTypingFrame.fromJson(Map<String, dynamic> json) => _$ChatTypingFrameFromJson(json);

    Map<String, dynamic> toJson() => _$ChatTypingFrameToJson(this);
}


///The other side is typing in a haul's chat. Not stored; show it for a few seconds.
@JsonSerializable()
class ChatTyping {
    @JsonKey(name: "booking_id")
    final String bookingId;
    @JsonKey(name: "user_id")
    final String userId;

    ChatTyping({
        required this.bookingId,
        required this.userId,
    });

    factory ChatTyping.fromJson(Map<String, dynamic> json) => _$ChatTypingFromJson(json);

    Map<String, dynamic> toJson() => _$ChatTypingToJson(this);
}

enum ChatTypingFrameType {
    @JsonValue("chat_typing")
    CHAT_TYPING
}

final chatTypingFrameTypeValues = EnumValues({
    "chat_typing": ChatTypingFrameType.CHAT_TYPING
});


///Frame `connected` goway-realtime sends to an app.
@JsonSerializable()
class ConnectedFrame {
    @JsonKey(name: "data")
    final Connected data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final ConnectedFrameType type;

    ConnectedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory ConnectedFrame.fromJson(Map<String, dynamic> json) => _$ConnectedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$ConnectedFrameToJson(this);
}


///The `auth` frame was accepted; the socket is live.
@JsonSerializable()
class Connected {
    @JsonKey(name: "config")
    final ConnectionConfig config;
    
    ///This socket's id, 32 hex characters.
    @JsonKey(name: "conn_id")
    final String connId;
    @JsonKey(name: "user_id")
    final String userId;
    
    ///The goway-realtime worker holding the socket, `<host>-<pid>-<suffix>`; for logs.
    @JsonKey(name: "worker")
    final String worker;

    Connected({
        required this.config,
        required this.connId,
        required this.userId,
        required this.worker,
    });

    factory Connected.fromJson(Map<String, dynamic> json) => _$ConnectedFromJson(json);

    Map<String, dynamic> toJson() => _$ConnectedToJson(this);
}


///Current realtime settings the app plays by.
@JsonSerializable()
class ConnectionConfig {
    
    ///Seconds between durable GPS samples.
    @JsonKey(name: "location_sample_seconds")
    final int locationSampleSeconds;
    
    ///Extra seconds after the deadline a late answer is still honoured.
    @JsonKey(name: "offer_grace_seconds")
    final int offerGraceSeconds;
    
    ///Seconds a driver has to answer an offer.
    @JsonKey(name: "offer_ttl_seconds")
    final int offerTtlSeconds;

    ConnectionConfig({
        required this.locationSampleSeconds,
        required this.offerGraceSeconds,
        required this.offerTtlSeconds,
    });

    factory ConnectionConfig.fromJson(Map<String, dynamic> json) => _$ConnectionConfigFromJson(json);

    Map<String, dynamic> toJson() => _$ConnectionConfigToJson(this);
}

enum ConnectedFrameType {
    @JsonValue("connected")
    CONNECTED
}

final connectedFrameTypeValues = EnumValues({
    "connected": ConnectedFrameType.CONNECTED
});


///Frame `driver_location` goway-realtime sends to an app.
@JsonSerializable()
class DriverLocationFrame {
    @JsonKey(name: "data")
    final DriverLocation data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final DriverLocationFrameType type;

    DriverLocationFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory DriverLocationFrame.fromJson(Map<String, dynamic> json) => _$DriverLocationFrameFromJson(json);

    Map<String, dynamic> toJson() => _$DriverLocationFrameToJson(this);
}


///The booked driver's live position, relayed to the booking's viewers as it arrives;
///dropped when a socket is slow.
@JsonSerializable()
class DriverLocation {
    @JsonKey(name: "booking_id")
    final String bookingId;
    
    ///Degrees from north; null when the driver's device did not send it.
    @JsonKey(name: "heading")
    final double? heading;
    @JsonKey(name: "lat")
    final double lat;
    @JsonKey(name: "lng")
    final double lng;
    
    ///Null when the driver's device did not send it.
    @JsonKey(name: "speed")
    final double? speed;

    DriverLocation({
        required this.bookingId,
        required this.heading,
        required this.lat,
        required this.lng,
        required this.speed,
    });

    factory DriverLocation.fromJson(Map<String, dynamic> json) => _$DriverLocationFromJson(json);

    Map<String, dynamic> toJson() => _$DriverLocationToJson(this);
}

enum DriverLocationFrameType {
    @JsonValue("driver_location")
    DRIVER_LOCATION
}

final driverLocationFrameTypeValues = EnumValues({
    "driver_location": DriverLocationFrameType.DRIVER_LOCATION
});


///Frame `error` goway-realtime sends to an app.
@JsonSerializable()
class ErrorFrame {
    @JsonKey(name: "data")
    final FrameError data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final ErrorFrameType type;

    ErrorFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory ErrorFrame.fromJson(Map<String, dynamic> json) => _$ErrorFrameFromJson(json);

    Map<String, dynamic> toJson() => _$ErrorFrameToJson(this);
}


///A frame was refused; the same problem shape as REST errors. Two forms are sent: a first
///frame that is not `auth`, an unreadable frame and a refused booking subscription send
///only `code` and `key`; every other refusal sends the full problem body with `type`,
///`title`, `status` and `code`, plus `detail`, `key` and `params` when set.
@JsonSerializable()
class FrameError {
    
    ///Stable code the app branches on.
    @JsonKey(name: "code")
    final String code;
    @JsonKey(name: "detail")
    final String? detail;
    
    ///Translation key for the message.
    @JsonKey(name: "key")
    final String? key;
    
    ///Values for the translated message.
    @JsonKey(name: "params")
    final Map<String, dynamic>? params;
    @JsonKey(name: "status")
    final int? status;
    
    ///HTTP status phrase, for example `Conflict`.
    @JsonKey(name: "title")
    final String? title;
    
    ///Problem type URI, `https://api.goway.uz/problems/<code>`.
    @JsonKey(name: "type")
    final String? type;

    FrameError({
        required this.code,
        this.detail,
        this.key,
        this.params,
        this.status,
        this.title,
        this.type,
    });

    factory FrameError.fromJson(Map<String, dynamic> json) => _$FrameErrorFromJson(json);

    Map<String, dynamic> toJson() => _$FrameErrorToJson(this);
}

enum ErrorFrameType {
    @JsonValue("error")
    ERROR
}

final errorFrameTypeValues = EnumValues({
    "error": ErrorFrameType.ERROR
});


///Frame `kicked` goway-realtime sends to an app.
@JsonSerializable()
class KickedFrame {
    @JsonKey(name: "data")
    final Kicked data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final KickedFrameType type;

    KickedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory KickedFrame.fromJson(Map<String, dynamic> json) => _$KickedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$KickedFrameToJson(this);
}


///This socket is being closed by the server; sign in again.
@JsonSerializable()
class Kicked {
    
    ///`device_removed`, `logout` (closed from another device), the `user.session_revoked`
    ///reason, or what a service passed to `/internal/kick`; null when none was given.
    @JsonKey(name: "reason")
    final String? reason;

    Kicked({
        required this.reason,
    });

    factory Kicked.fromJson(Map<String, dynamic> json) => _$KickedFromJson(json);

    Map<String, dynamic> toJson() => _$KickedToJson(this);
}

enum KickedFrameType {
    @JsonValue("kicked")
    KICKED
}

final kickedFrameTypeValues = EnumValues({
    "kicked": KickedFrameType.KICKED
});


///Frame `notification` goway-realtime sends to an app.
@JsonSerializable()
class NotificationFrame {
    @JsonKey(name: "data")
    final Notification data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final NotificationFrameType type;

    NotificationFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory NotificationFrame.fromJson(Map<String, dynamic> json) => _$NotificationFrameFromJson(json);

    Map<String, dynamic> toJson() => _$NotificationFrameToJson(this);
}


///Something landed in the user's notification feed; refresh the badge. Copied from
///`notification.created`; a field that event lacked or sent empty arrives as null.
@JsonSerializable()
class Notification {
    
    ///Text in the user's language.
    @JsonKey(name: "body")
    final String? body;
    
    ///Template kind, for example `driver_found` or `announcement`.
    @JsonKey(name: "kind")
    final String? kind;
    @JsonKey(name: "notification_id")
    final String? notificationId;
    
    ///Title in the user's language.
    @JsonKey(name: "title")
    final String? title;

    Notification({
        required this.body,
        required this.kind,
        required this.notificationId,
        required this.title,
    });

    factory Notification.fromJson(Map<String, dynamic> json) => _$NotificationFromJson(json);

    Map<String, dynamic> toJson() => _$NotificationToJson(this);
}

enum NotificationFrameType {
    @JsonValue("notification")
    NOTIFICATION
}

final notificationFrameTypeValues = EnumValues({
    "notification": NotificationFrameType.NOTIFICATION
});


///Frame `offer_proposed` goway-realtime sends to an app.
@JsonSerializable()
class OfferProposedFrame {
    @JsonKey(name: "data")
    final OfferProposed data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OfferProposedFrameType type;

    OfferProposedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OfferProposedFrame.fromJson(Map<String, dynamic> json) => _$OfferProposedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OfferProposedFrameToJson(this);
}


///An instant load offered to this driver; answer before `deadline_at`.
@JsonSerializable()
class OfferProposed {
    
    ///Price in minor units of `currency`.
    @JsonKey(name: "amount")
    final int amount;
    
    ///Cargo type code from the reference catalogue.
    @JsonKey(name: "cargo_category")
    final String cargoCategory;
    @JsonKey(name: "currency")
    final String currency;
    
    ///Answer deadline, unix milliseconds in server time.
    @JsonKey(name: "deadline_at")
    final int deadlineAt;
    
    ///Driving distance through the stops; null when routing was unavailable at posting.
    @JsonKey(name: "distance_km")
    final double? distanceKm;
    
    ///Last drop-off stop's address; null only when the load has no stops.
    @JsonKey(name: "dropoff_address")
    final String? dropoffAddress;
    
    ///Driving time for the same route in minutes; null together with the distance.
    @JsonKey(name: "duration_min")
    final int? durationMin;
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "is_urgent")
    final bool isUrgent;
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///Loaders the shipper ordered with the truck.
    @JsonKey(name: "loaders_count")
    final int loadersCount;
    @JsonKey(name: "loading_method")
    final LoadingMethod loadingMethod;
    @JsonKey(name: "needs_refrigeration")
    final bool needsRefrigeration;
    @JsonKey(name: "payment_method")
    final PaymentMethod paymentMethod;
    
    ///First pickup stop's address as the shipper wrote it; null only when the load has no stops.
    @JsonKey(name: "pickup_address")
    final String? pickupAddress;
    
    ///Straight-line distance from the driver's last position to the pickup; null while the
    ///position is unknown.
    @JsonKey(name: "pickup_distance_km")
    final double? pickupDistanceKm;
    
    ///Earliest pickup; null when not given. ISO 8601 with offset, as Python's `isoformat()`
    ///writes it.
    @JsonKey(name: "pickup_from")
    final DateTime? pickupFrom;
    
    ///Latest pickup, also the posting's expiry; null when not given. ISO 8601 with offset, as
    ///Python's `isoformat()` writes it.
    @JsonKey(name: "pickup_to")
    final DateTime? pickupTo;
    @JsonKey(name: "server_time")
    final DateTime serverTime;
    
    ///Pickup and drop-off stops together.
    @JsonKey(name: "stops_count")
    final int stopsCount;
    @JsonKey(name: "to_region_id")
    final String toRegionId;
    @JsonKey(name: "truck_class")
    final String truckClass;
    @JsonKey(name: "volume_m3")
    final double? volumeM3;
    @JsonKey(name: "weight_kg")
    final int weightKg;

    OfferProposed({
        required this.amount,
        required this.cargoCategory,
        required this.currency,
        required this.deadlineAt,
        required this.distanceKm,
        required this.dropoffAddress,
        required this.durationMin,
        required this.fromRegionId,
        required this.isUrgent,
        required this.loadId,
        required this.loadersCount,
        required this.loadingMethod,
        required this.needsRefrigeration,
        required this.paymentMethod,
        required this.pickupAddress,
        required this.pickupDistanceKm,
        required this.pickupFrom,
        required this.pickupTo,
        required this.serverTime,
        required this.stopsCount,
        required this.toRegionId,
        required this.truckClass,
        required this.volumeM3,
        required this.weightKg,
    });

    factory OfferProposed.fromJson(Map<String, dynamic> json) => _$OfferProposedFromJson(json);

    Map<String, dynamic> toJson() => _$OfferProposedToJson(this);
}


///How the cargo goes in.
enum LoadingMethod {
    @JsonValue("rear")
    REAR,
    @JsonValue("top")
    TOP
}

final loadingMethodValues = EnumValues({
    "rear": LoadingMethod.REAR,
    "top": LoadingMethod.TOP
});

enum PaymentMethod {
    @JsonValue("card")
    CARD,
    @JsonValue("cash")
    CASH,
    @JsonValue("mixed")
    MIXED,
    @JsonValue("transfer")
    TRANSFER
}

final paymentMethodValues = EnumValues({
    "card": PaymentMethod.CARD,
    "cash": PaymentMethod.CASH,
    "mixed": PaymentMethod.MIXED,
    "transfer": PaymentMethod.TRANSFER
});

enum OfferProposedFrameType {
    @JsonValue("offer_proposed")
    OFFER_PROPOSED
}

final offerProposedFrameTypeValues = EnumValues({
    "offer_proposed": OfferProposedFrameType.OFFER_PROPOSED
});


///Frame `offer_result` goway-realtime sends to an app.
@JsonSerializable()
class OfferResultFrame {
    @JsonKey(name: "data")
    final OfferResult data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OfferResultFrameType type;

    OfferResultFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OfferResultFrame.fromJson(Map<String, dynamic> json) => _$OfferResultFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OfferResultFrameToJson(this);
}


///How an offer ended for this driver.
@JsonSerializable()
class OfferResult {
    
    ///Sent only when the outcome is `won`; null when goway-loads' answer could not be read.
    @JsonKey(name: "booking_id")
    final String? bookingId;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "outcome")
    final OfferOutcome outcome;

    OfferResult({
        this.bookingId,
        required this.loadId,
        required this.outcome,
    });

    factory OfferResult.fromJson(Map<String, dynamic> json) => _$OfferResultFromJson(json);

    Map<String, dynamic> toJson() => _$OfferResultToJson(this);
}

enum OfferOutcome {
    @JsonValue("declined")
    DECLINED,
    @JsonValue("expired")
    EXPIRED,
    @JsonValue("lost")
    LOST,
    @JsonValue("withdrawn")
    WITHDRAWN,
    @JsonValue("won")
    WON
}

final offerOutcomeValues = EnumValues({
    "declined": OfferOutcome.DECLINED,
    "expired": OfferOutcome.EXPIRED,
    "lost": OfferOutcome.LOST,
    "withdrawn": OfferOutcome.WITHDRAWN,
    "won": OfferOutcome.WON
});

enum OfferResultFrameType {
    @JsonValue("offer_result")
    OFFER_RESULT
}

final offerResultFrameTypeValues = EnumValues({
    "offer_result": OfferResultFrameType.OFFER_RESULT
});


///Frame `offers_blocked` goway-realtime sends to an app.
@JsonSerializable()
class OffersBlockedFrame {
    @JsonKey(name: "data")
    final OffersBlocked data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OffersBlockedFrameType type;

    OffersBlockedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OffersBlockedFrame.fromJson(Map<String, dynamic> json) => _$OffersBlockedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OffersBlockedFrameToJson(this);
}


///The wallet stopped being eligible; no offers until it is settled.
@JsonSerializable()
class OffersBlocked {
    @JsonKey(name: "reason")
    final OffersBlockedReason reason;

    OffersBlocked({
        required this.reason,
    });

    factory OffersBlocked.fromJson(Map<String, dynamic> json) => _$OffersBlockedFromJson(json);

    Map<String, dynamic> toJson() => _$OffersBlockedToJson(this);
}

enum OffersBlockedReason {
    @JsonValue("BALANCE_INSUFFICIENT")
    BALANCE_INSUFFICIENT,
    @JsonValue("SUBSCRIPTION_REQUIRED")
    SUBSCRIPTION_REQUIRED
}

final offersBlockedReasonValues = EnumValues({
    "BALANCE_INSUFFICIENT": OffersBlockedReason.BALANCE_INSUFFICIENT,
    "SUBSCRIPTION_REQUIRED": OffersBlockedReason.SUBSCRIPTION_REQUIRED
});

enum OffersBlockedFrameType {
    @JsonValue("offers_blocked")
    OFFERS_BLOCKED
}

final offersBlockedFrameTypeValues = EnumValues({
    "offers_blocked": OffersBlockedFrameType.OFFERS_BLOCKED
});


///Frame `offers_unblocked` goway-realtime sends to an app.
@JsonSerializable()
class OffersUnblockedFrame {
    @JsonKey(name: "data")
    final Map<String, dynamic> data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OffersUnblockedFrameType type;

    OffersUnblockedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OffersUnblockedFrame.fromJson(Map<String, dynamic> json) => _$OffersUnblockedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OffersUnblockedFrameToJson(this);
}

enum OffersUnblockedFrameType {
    @JsonValue("offers_unblocked")
    OFFERS_UNBLOCKED
}

final offersUnblockedFrameTypeValues = EnumValues({
    "offers_unblocked": OffersUnblockedFrameType.OFFERS_UNBLOCKED
});


///Frame `offline` goway-realtime sends to an app.
@JsonSerializable()
class OfflineFrame {
    @JsonKey(name: "data")
    final Offline data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OfflineFrameType type;

    OfflineFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OfflineFrame.fromJson(Map<String, dynamic> json) => _$OfflineFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OfflineFrameToJson(this);
}


///The driver left the line: after `go_offline`, or taken off it with a `reason`.
@JsonSerializable()
class Offline {
    @JsonKey(name: "reason")
    final OfflineReason? reason;

    Offline({
        this.reason,
    });

    factory Offline.fromJson(Map<String, dynamic> json) => _$OfflineFromJson(json);

    Map<String, dynamic> toJson() => _$OfflineToJson(this);
}


///Why the server took the driver off the line; absent when the driver left by themselves.
///`vehicle_unavailable`: the line's truck was removed, left the fleet, or the line has none.
enum OfflineReason {
    @JsonValue("documents_expired")
    DOCUMENTS_EXPIRED,
    @JsonValue("vehicle_unavailable")
    VEHICLE_UNAVAILABLE,
    @JsonValue("verification_rejected")
    VERIFICATION_REJECTED
}

final offlineReasonValues = EnumValues({
    "documents_expired": OfflineReason.DOCUMENTS_EXPIRED,
    "vehicle_unavailable": OfflineReason.VEHICLE_UNAVAILABLE,
    "verification_rejected": OfflineReason.VERIFICATION_REJECTED
});

enum OfflineFrameType {
    @JsonValue("offline")
    OFFLINE
}

final offlineFrameTypeValues = EnumValues({
    "offline": OfflineFrameType.OFFLINE
});


///Frame `online` goway-realtime sends to an app.
@JsonSerializable()
class OnlineFrame {
    @JsonKey(name: "data")
    final Online data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final OnlineFrameType type;

    OnlineFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory OnlineFrame.fromJson(Map<String, dynamic> json) => _$OnlineFrameFromJson(json);

    Map<String, dynamic> toJson() => _$OnlineFrameToJson(this);
}


///The driver is on their line: after `go_online`, and again whenever the line is changed
///while online.
@JsonSerializable()
class Online {
    
    ///The way back is taken too.
    @JsonKey(name: "bidirectional")
    final bool bidirectional;
    
    ///Where the line starts.
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    
    ///Set when the wallet blocks offers, with the reason; null otherwise.
    @JsonKey(name: "offers_blocked")
    final OffersBlockedReason? offersBlocked;
    
    ///Place in the outbound line's queue, from 1.
    @JsonKey(name: "queue_position")
    final int queuePosition;
    
    ///Where the line ends.
    @JsonKey(name: "to_region_id")
    final String toRegionId;
    
    ///The truck the offers are matched against.
    @JsonKey(name: "vehicle_id")
    final String vehicleId;

    Online({
        required this.bidirectional,
        required this.fromRegionId,
        required this.offersBlocked,
        required this.queuePosition,
        required this.toRegionId,
        required this.vehicleId,
    });

    factory Online.fromJson(Map<String, dynamic> json) => _$OnlineFromJson(json);

    Map<String, dynamic> toJson() => _$OnlineToJson(this);
}

enum OnlineFrameType {
    @JsonValue("online")
    ONLINE
}

final onlineFrameTypeValues = EnumValues({
    "online": OnlineFrameType.ONLINE
});


///Frame `pong` goway-realtime sends to an app.
@JsonSerializable()
class PongFrame {
    @JsonKey(name: "data")
    final Map<String, dynamic> data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final PongFrameType type;

    PongFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory PongFrame.fromJson(Map<String, dynamic> json) => _$PongFrameFromJson(json);

    Map<String, dynamic> toJson() => _$PongFrameToJson(this);
}

enum PongFrameType {
    @JsonValue("pong")
    PONG
}

final pongFrameTypeValues = EnumValues({
    "pong": PongFrameType.PONG
});


///Frame `search_exhausted` goway-realtime sends to an app.
@JsonSerializable()
class SearchExhaustedFrame {
    @JsonKey(name: "data")
    final SearchExhausted data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final SearchExhaustedFrameType type;

    SearchExhaustedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory SearchExhaustedFrame.fromJson(Map<String, dynamic> json) => _$SearchExhaustedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SearchExhaustedFrameToJson(this);
}


///To the shipper: no driver took the load within the search window. The search goes on; the
///app offers to raise the price, move the pickup to a plan or cancel.
@JsonSerializable()
class SearchExhausted {
    
    ///Current price, whole so'm.
    @JsonKey(name: "amount")
    final int amount;
    @JsonKey(name: "currency")
    final String currency;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "seen")
    final int seen;
    
    ///The raised price GoWay suggests.
    @JsonKey(name: "suggested_amount")
    final int suggestedAmount;

    SearchExhausted({
        required this.amount,
        required this.currency,
        required this.loadId,
        required this.seen,
        required this.suggestedAmount,
    });

    factory SearchExhausted.fromJson(Map<String, dynamic> json) => _$SearchExhaustedFromJson(json);

    Map<String, dynamic> toJson() => _$SearchExhaustedToJson(this);
}

enum SearchExhaustedFrameType {
    @JsonValue("search_exhausted")
    SEARCH_EXHAUSTED
}

final searchExhaustedFrameTypeValues = EnumValues({
    "search_exhausted": SearchExhaustedFrameType.SEARCH_EXHAUSTED
});


///Frame `search_progress` goway-realtime sends to an app.
@JsonSerializable()
class SearchProgressFrame {
    @JsonKey(name: "data")
    final SearchProgress data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final SearchProgressFrameType type;

    SearchProgressFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory SearchProgressFrame.fromJson(Map<String, dynamic> json) => _$SearchProgressFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SearchProgressFrameToJson(this);
}


///To the shipper while their instant load is being offered: how many drivers have seen it.
@JsonSerializable()
class SearchProgress {
    @JsonKey(name: "load_id")
    final String loadId;
    
    ///Drivers the load was offered to so far.
    @JsonKey(name: "seen")
    final int seen;
    
    ///When the search began; the app counts the waiting time from it.
    @JsonKey(name: "started_at")
    final DateTime startedAt;

    SearchProgress({
        required this.loadId,
        required this.seen,
        required this.startedAt,
    });

    factory SearchProgress.fromJson(Map<String, dynamic> json) => _$SearchProgressFromJson(json);

    Map<String, dynamic> toJson() => _$SearchProgressToJson(this);
}

enum SearchProgressFrameType {
    @JsonValue("search_progress")
    SEARCH_PROGRESS
}

final searchProgressFrameTypeValues = EnumValues({
    "search_progress": SearchProgressFrameType.SEARCH_PROGRESS
});


///Frame `session_started` goway-realtime sends to an app.
@JsonSerializable()
class SessionStartedFrame {
    @JsonKey(name: "data")
    final SessionStarted data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final SessionStartedFrameType type;

    SessionStartedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory SessionStartedFrame.fromJson(Map<String, dynamic> json) => _$SessionStartedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SessionStartedFrameToJson(this);
}


///Somebody signed in to this account; ignore it when `installation_id` is this app's own.
///Copied from `user.login`; a field that event lacked or sent empty arrives as null.
@JsonSerializable()
class SessionStarted {
    
    ///When the sign-in happened, as `user.login` wrote it.
    @JsonKey(name: "at")
    final DateTime? at;
    @JsonKey(name: "device_id")
    final String? deviceId;
    @JsonKey(name: "installation_id")
    final String? installationId;
    
    ///Device model.
    @JsonKey(name: "model")
    final String? model;
    @JsonKey(name: "new_device")
    final bool newDevice;
    
    ///Device platform.
    @JsonKey(name: "platform")
    final DevicePlatform? platform;

    SessionStarted({
        required this.at,
        required this.deviceId,
        required this.installationId,
        required this.model,
        required this.newDevice,
        required this.platform,
    });

    factory SessionStarted.fromJson(Map<String, dynamic> json) => _$SessionStartedFromJson(json);

    Map<String, dynamic> toJson() => _$SessionStartedToJson(this);
}

enum DevicePlatform {
    @JsonValue("android")
    ANDROID,
    @JsonValue("ios")
    IOS,
    @JsonValue("web")
    WEB
}

final devicePlatformValues = EnumValues({
    "android": DevicePlatform.ANDROID,
    "ios": DevicePlatform.IOS,
    "web": DevicePlatform.WEB
});

enum SessionStartedFrameType {
    @JsonValue("session_started")
    SESSION_STARTED
}

final sessionStartedFrameTypeValues = EnumValues({
    "session_started": SessionStartedFrameType.SESSION_STARTED
});


///Frame `subscribed` goway-realtime sends to an app.
@JsonSerializable()
class SubscribedFrame {
    @JsonKey(name: "data")
    final Subscribed data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final SubscribedFrameType type;

    SubscribedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory SubscribedFrame.fromJson(Map<String, dynamic> json) => _$SubscribedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribedFrameToJson(this);
}


///`subscribe_booking` was accepted.
@JsonSerializable()
class Subscribed {
    @JsonKey(name: "booking_id")
    final String bookingId;

    Subscribed({
        required this.bookingId,
    });

    factory Subscribed.fromJson(Map<String, dynamic> json) => _$SubscribedFromJson(json);

    Map<String, dynamic> toJson() => _$SubscribedToJson(this);
}

enum SubscribedFrameType {
    @JsonValue("subscribed")
    SUBSCRIBED
}

final subscribedFrameTypeValues = EnumValues({
    "subscribed": SubscribedFrameType.SUBSCRIBED
});


///Frame `support_message` goway-realtime sends to an app. Sent to the person the
///conversation belongs to when an operator writes. It forwards the internal
///`support.message_created` event as-is.
@JsonSerializable()
class SupportMessageFrame {
    @JsonKey(name: "data")
    final SupportMessageCreated data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final SupportMessageFrameType type;

    SupportMessageFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory SupportMessageFrame.fromJson(Map<String, dynamic> json) => _$SupportMessageFrameFromJson(json);

    Map<String, dynamic> toJson() => _$SupportMessageFrameToJson(this);
}


///A message was added to a support conversation, by the person who opened it or by an
///operator.
@JsonSerializable()
class SupportMessageCreated {
    @JsonKey(name: "author")
    final SupportAuthor author;
    @JsonKey(name: "created_at")
    final DateTime createdAt;
    
    ///Unique id of this event, stamped by the publisher's outbox. Consumers dedupe on it.
    @JsonKey(name: "event_id")
    final String eventId;
    @JsonKey(name: "message_id")
    final String messageId;
    
    ///The ticket's short number.
    @JsonKey(name: "number")
    final String number;
    @JsonKey(name: "text")
    final String text;
    @JsonKey(name: "ticket_id")
    final String ticketId;
    
    ///The person the conversation belongs to.
    @JsonKey(name: "user_id")
    final String userId;

    SupportMessageCreated({
        required this.author,
        required this.createdAt,
        required this.eventId,
        required this.messageId,
        required this.number,
        required this.text,
        required this.ticketId,
        required this.userId,
    });

    factory SupportMessageCreated.fromJson(Map<String, dynamic> json) => _$SupportMessageCreatedFromJson(json);

    Map<String, dynamic> toJson() => _$SupportMessageCreatedToJson(this);
}

enum SupportAuthor {
    @JsonValue("operator")
    OPERATOR,
    @JsonValue("user")
    USER
}

final supportAuthorValues = EnumValues({
    "operator": SupportAuthor.OPERATOR,
    "user": SupportAuthor.USER
});

enum SupportMessageFrameType {
    @JsonValue("support_message")
    SUPPORT_MESSAGE
}

final supportMessageFrameTypeValues = EnumValues({
    "support_message": SupportMessageFrameType.SUPPORT_MESSAGE
});


///Frame `verification_changed` goway-realtime sends to an app.
@JsonSerializable()
class VerificationChangedFrame {
    @JsonKey(name: "data")
    final VerificationChanged data;
    
    ///When the server sent the frame, RFC 3339 UTC.
    @JsonKey(name: "ts")
    final DateTime ts;
    @JsonKey(name: "type")
    final VerificationChangedFrameType type;

    VerificationChangedFrame({
        required this.data,
        required this.ts,
        required this.type,
    });

    factory VerificationChangedFrame.fromJson(Map<String, dynamic> json) => _$VerificationChangedFrameFromJson(json);

    Map<String, dynamic> toJson() => _$VerificationChangedFrameToJson(this);
}


///The carrier's verification moved; the app re-reads `GET /v1/carrier/verification` for the
///note.
@JsonSerializable()
class VerificationChanged {
    @JsonKey(name: "status")
    final VerificationStatus status;

    VerificationChanged({
        required this.status,
    });

    factory VerificationChanged.fromJson(Map<String, dynamic> json) => _$VerificationChangedFromJson(json);

    Map<String, dynamic> toJson() => _$VerificationChangedToJson(this);
}


///The new verification status; `expired` when an approved document ran out.
enum VerificationStatus {
    @JsonValue("approved")
    APPROVED,
    @JsonValue("expired")
    EXPIRED,
    @JsonValue("pending")
    PENDING,
    @JsonValue("rejected")
    REJECTED
}

final verificationStatusValues = EnumValues({
    "approved": VerificationStatus.APPROVED,
    "expired": VerificationStatus.EXPIRED,
    "pending": VerificationStatus.PENDING,
    "rejected": VerificationStatus.REJECTED
});

enum VerificationChangedFrameType {
    @JsonValue("verification_changed")
    VERIFICATION_CHANGED
}

final verificationChangedFrameTypeValues = EnumValues({
    "verification_changed": VerificationChangedFrameType.VERIFICATION_CHANGED
});

class EnumValues<T> {
    Map<String, T> map;
    late Map<T, String> reverseMap;

    EnumValues(this.map);

    Map<T, String> get reverse {
            reverseMap = map.map((k, v) => MapEntry(v, k));
            return reverseMap;
    }
}
