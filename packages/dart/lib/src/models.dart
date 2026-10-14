// ignore_for_file: constant_identifier_names
// To parse this JSON data, do
//
//     final goOnlineFrame = goOnlineFrameFromJson(jsonString);
//     final offerResponseFrame = offerResponseFrameFromJson(jsonString);
//     final errorFrame = errorFrameFromJson(jsonString);
//     final offerProposedFrame = offerProposedFrameFromJson(jsonString);
//     final offerResultFrame = offerResultFrameFromJson(jsonString);

import 'package:json_annotation/json_annotation.dart';
import 'dart:convert';

part 'models.g.dart';

GoOnlineFrame goOnlineFrameFromJson(String str) => GoOnlineFrame.fromJson(json.decode(str));

String goOnlineFrameToJson(GoOnlineFrame data) => json.encode(data.toJson());

OfferResponseFrame offerResponseFrameFromJson(String str) => OfferResponseFrame.fromJson(json.decode(str));

String offerResponseFrameToJson(OfferResponseFrame data) => json.encode(data.toJson());

ErrorFrame errorFrameFromJson(String str) => ErrorFrame.fromJson(json.decode(str));

String errorFrameToJson(ErrorFrame data) => json.encode(data.toJson());

OfferProposedFrame offerProposedFrameFromJson(String str) => OfferProposedFrame.fromJson(json.decode(str));

String offerProposedFrameToJson(OfferProposedFrame data) => json.encode(data.toJson());

OfferResultFrame offerResultFrameFromJson(String str) => OfferResultFrame.fromJson(json.decode(str));

String offerResultFrameToJson(OfferResultFrame data) => json.encode(data.toJson());


///Frame `go_online` an app sends to goway-realtime.
@JsonSerializable()
class GoOnlineFrame {
    @JsonKey(name: "data")
    final GoOnline data;
    @JsonKey(name: "type")
    final GoOnlineFrameType type;

    GoOnlineFrame({
        required this.data,
        required this.type,
    });

    factory GoOnlineFrame.fromJson(Map<String, dynamic> json) => _$GoOnlineFrameFromJson(json);

    Map<String, dynamic> toJson() => _$GoOnlineFrameToJson(this);
}


///Start receiving instant offers on a line with one vehicle.
@JsonSerializable()
class GoOnline {
    
    ///Also take loads on the way back.
    @JsonKey(name: "bidirectional")
    final bool? bidirectional;
    
    ///How long to stay online; must be one of the allowed durations.
    @JsonKey(name: "duration_hours")
    final int? durationHours;
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "to_region_id")
    final String toRegionId;
    @JsonKey(name: "vehicle_id")
    final String vehicleId;

    GoOnline({
        this.bidirectional,
        this.durationHours,
        required this.fromRegionId,
        required this.toRegionId,
        required this.vehicleId,
    });

    factory GoOnline.fromJson(Map<String, dynamic> json) => _$GoOnlineFromJson(json);

    Map<String, dynamic> toJson() => _$GoOnlineToJson(this);
}

enum GoOnlineFrameType {
    @JsonValue("go_online")
    GO_ONLINE
}

final goOnlineFrameTypeValues = EnumValues({
    "go_online": GoOnlineFrameType.GO_ONLINE
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


///A frame was refused; the same problem shape as REST errors.
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

    FrameError({
        required this.code,
        this.detail,
        this.key,
        this.params,
        this.status,
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
    @JsonKey(name: "currency")
    final String currency;
    
    ///Answer deadline, unix milliseconds in server time.
    @JsonKey(name: "deadline_at")
    final int deadlineAt;
    @JsonKey(name: "from_region_id")
    final String fromRegionId;
    @JsonKey(name: "load_id")
    final String loadId;
    @JsonKey(name: "server_time")
    final DateTime serverTime;
    @JsonKey(name: "to_region_id")
    final String toRegionId;
    @JsonKey(name: "truck_class")
    final String truckClass;
    @JsonKey(name: "weight_kg")
    final int weightKg;

    OfferProposed({
        required this.amount,
        required this.currency,
        required this.deadlineAt,
        required this.fromRegionId,
        required this.loadId,
        required this.serverTime,
        required this.toRegionId,
        required this.truckClass,
        required this.weightKg,
    });

    factory OfferProposed.fromJson(Map<String, dynamic> json) => _$OfferProposedFromJson(json);

    Map<String, dynamic> toJson() => _$OfferProposedToJson(this);
}

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
    
    ///Set when the outcome is `won`.
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

class EnumValues<T> {
    Map<String, T> map;
    late Map<T, String> reverseMap;

    EnumValues(this.map);

    Map<T, String> get reverse {
            reverseMap = map.map((k, v) => MapEntry(v, k));
            return reverseMap;
    }
}
