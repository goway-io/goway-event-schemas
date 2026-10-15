// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'models.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AuthFrame _$AuthFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('AuthFrame', json, ($checkedConvert) {
      final val = AuthFrame(
        data: $checkedConvert(
          'data',
          (v) => Auth.fromJson(v as Map<String, dynamic>),
        ),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$AuthFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$AuthFrameToJson(AuthFrame instance) => <String, dynamic>{
  'data': instance.data,
  'type': _$AuthFrameTypeEnumMap[instance.type]!,
};

const _$AuthFrameTypeEnumMap = {AuthFrameType.AUTH: 'auth'};

Auth _$AuthFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Auth', json, ($checkedConvert) {
      final val = Auth(
        installationId: $checkedConvert('installation_id', (v) => v as String?),
        token: $checkedConvert('token', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'installationId': 'installation_id'});

Map<String, dynamic> _$AuthToJson(Auth instance) => <String, dynamic>{
  'installation_id': ?instance.installationId,
  'token': instance.token,
};

GoOfflineFrame _$GoOfflineFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('GoOfflineFrame', json, ($checkedConvert) {
      final val = GoOfflineFrame(
        data: $checkedConvert('data', (v) => v as Map<String, dynamic>?),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$GoOfflineFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$GoOfflineFrameToJson(GoOfflineFrame instance) =>
    <String, dynamic>{
      'data': ?instance.data,
      'type': _$GoOfflineFrameTypeEnumMap[instance.type]!,
    };

const _$GoOfflineFrameTypeEnumMap = {
  GoOfflineFrameType.GO_OFFLINE: 'go_offline',
};

GoOnlineFrame _$GoOnlineFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('GoOnlineFrame', json, ($checkedConvert) {
      final val = GoOnlineFrame(
        data: $checkedConvert('data', (v) => v as Map<String, dynamic>),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$GoOnlineFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$GoOnlineFrameToJson(GoOnlineFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'type': _$GoOnlineFrameTypeEnumMap[instance.type]!,
    };

const _$GoOnlineFrameTypeEnumMap = {GoOnlineFrameType.GO_ONLINE: 'go_online'};

LocationFrame _$LocationFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('LocationFrame', json, ($checkedConvert) {
      final val = LocationFrame(
        data: $checkedConvert(
          'data',
          (v) => Location.fromJson(v as Map<String, dynamic>),
        ),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$LocationFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$LocationFrameToJson(LocationFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'type': _$LocationFrameTypeEnumMap[instance.type]!,
    };

const _$LocationFrameTypeEnumMap = {LocationFrameType.LOCATION: 'location'};

Location _$LocationFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Location', json, ($checkedConvert) {
      final val = Location(
        heading: $checkedConvert('heading', (v) => (v as num?)?.toDouble()),
        lat: $checkedConvert('lat', (v) => (v as num).toDouble()),
        lng: $checkedConvert('lng', (v) => (v as num).toDouble()),
        speed: $checkedConvert('speed', (v) => (v as num?)?.toDouble()),
      );
      return val;
    });

Map<String, dynamic> _$LocationToJson(Location instance) => <String, dynamic>{
  'heading': ?instance.heading,
  'lat': instance.lat,
  'lng': instance.lng,
  'speed': ?instance.speed,
};

OfferResponseFrame _$OfferResponseFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfferResponseFrame', json, ($checkedConvert) {
      final val = OfferResponseFrame(
        data: $checkedConvert(
          'data',
          (v) => OfferResponse.fromJson(v as Map<String, dynamic>),
        ),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OfferResponseFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OfferResponseFrameToJson(OfferResponseFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'type': _$OfferResponseFrameTypeEnumMap[instance.type]!,
    };

const _$OfferResponseFrameTypeEnumMap = {
  OfferResponseFrameType.OFFER_RESPONSE: 'offer_response',
};

OfferResponse _$OfferResponseFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfferResponse', json, ($checkedConvert) {
      final val = OfferResponse(
        action: $checkedConvert(
          'action',
          (v) => $enumDecode(_$OfferActionEnumMap, v),
        ),
        loadId: $checkedConvert('load_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'loadId': 'load_id'});

Map<String, dynamic> _$OfferResponseToJson(OfferResponse instance) =>
    <String, dynamic>{
      'action': _$OfferActionEnumMap[instance.action]!,
      'load_id': instance.loadId,
    };

const _$OfferActionEnumMap = {
  OfferAction.ACCEPT: 'accept',
  OfferAction.DECLINE: 'decline',
};

PingFrame _$PingFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('PingFrame', json, ($checkedConvert) {
      final val = PingFrame(
        data: $checkedConvert('data', (v) => v as Map<String, dynamic>?),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$PingFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$PingFrameToJson(PingFrame instance) => <String, dynamic>{
  'data': ?instance.data,
  'type': _$PingFrameTypeEnumMap[instance.type]!,
};

const _$PingFrameTypeEnumMap = {PingFrameType.PING: 'ping'};

SubscribeBoardFrame _$SubscribeBoardFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SubscribeBoardFrame', json, ($checkedConvert) {
      final val = SubscribeBoardFrame(
        data: $checkedConvert(
          'data',
          (v) => SubscribeBoard.fromJson(v as Map<String, dynamic>),
        ),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$SubscribeBoardFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$SubscribeBoardFrameToJson(
  SubscribeBoardFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'type': _$SubscribeBoardFrameTypeEnumMap[instance.type]!,
};

const _$SubscribeBoardFrameTypeEnumMap = {
  SubscribeBoardFrameType.SUBSCRIBE_BOARD: 'subscribe_board',
};

SubscribeBoard _$SubscribeBoardFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'SubscribeBoard',
      json,
      ($checkedConvert) {
        final val = SubscribeBoard(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$SubscribeBoardToJson(SubscribeBoard instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'to_region_id': instance.toRegionId,
    };

SubscribeBookingFrame _$SubscribeBookingFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('SubscribeBookingFrame', json, ($checkedConvert) {
  final val = SubscribeBookingFrame(
    data: $checkedConvert(
      'data',
      (v) => SubscribeBooking.fromJson(v as Map<String, dynamic>),
    ),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$SubscribeBookingFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$SubscribeBookingFrameToJson(
  SubscribeBookingFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'type': _$SubscribeBookingFrameTypeEnumMap[instance.type]!,
};

const _$SubscribeBookingFrameTypeEnumMap = {
  SubscribeBookingFrameType.SUBSCRIBE_BOOKING: 'subscribe_booking',
};

SubscribeBooking _$SubscribeBookingFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SubscribeBooking', json, ($checkedConvert) {
      final val = SubscribeBooking(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id'});

Map<String, dynamic> _$SubscribeBookingToJson(SubscribeBooking instance) =>
    <String, dynamic>{'booking_id': instance.bookingId};

TypingFrame _$TypingFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('TypingFrame', json, ($checkedConvert) {
      final val = TypingFrame(
        data: $checkedConvert(
          'data',
          (v) => Typing.fromJson(v as Map<String, dynamic>),
        ),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$TypingFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$TypingFrameToJson(TypingFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'type': _$TypingFrameTypeEnumMap[instance.type]!,
    };

const _$TypingFrameTypeEnumMap = {TypingFrameType.TYPING: 'typing'};

Typing _$TypingFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Typing', json, ($checkedConvert) {
      final val = Typing(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id'});

Map<String, dynamic> _$TypingToJson(Typing instance) => <String, dynamic>{
  'booking_id': instance.bookingId,
};

UnsubscribeBoardFrame _$UnsubscribeBoardFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('UnsubscribeBoardFrame', json, ($checkedConvert) {
  final val = UnsubscribeBoardFrame(
    data: $checkedConvert(
      'data',
      (v) => UnsubscribeBoard.fromJson(v as Map<String, dynamic>),
    ),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$UnsubscribeBoardFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$UnsubscribeBoardFrameToJson(
  UnsubscribeBoardFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'type': _$UnsubscribeBoardFrameTypeEnumMap[instance.type]!,
};

const _$UnsubscribeBoardFrameTypeEnumMap = {
  UnsubscribeBoardFrameType.UNSUBSCRIBE_BOARD: 'unsubscribe_board',
};

UnsubscribeBoard _$UnsubscribeBoardFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'UnsubscribeBoard',
      json,
      ($checkedConvert) {
        final val = UnsubscribeBoard(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$UnsubscribeBoardToJson(UnsubscribeBoard instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'to_region_id': instance.toRegionId,
    };

UnsubscribeBookingFrame _$UnsubscribeBookingFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('UnsubscribeBookingFrame', json, ($checkedConvert) {
  final val = UnsubscribeBookingFrame(
    data: $checkedConvert(
      'data',
      (v) => UnsubscribeBooking.fromJson(v as Map<String, dynamic>),
    ),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$UnsubscribeBookingFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$UnsubscribeBookingFrameToJson(
  UnsubscribeBookingFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'type': _$UnsubscribeBookingFrameTypeEnumMap[instance.type]!,
};

const _$UnsubscribeBookingFrameTypeEnumMap = {
  UnsubscribeBookingFrameType.UNSUBSCRIBE_BOOKING: 'unsubscribe_booking',
};

UnsubscribeBooking _$UnsubscribeBookingFromJson(Map<String, dynamic> json) =>
    $checkedCreate('UnsubscribeBooking', json, ($checkedConvert) {
      final val = UnsubscribeBooking(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id'});

Map<String, dynamic> _$UnsubscribeBookingToJson(UnsubscribeBooking instance) =>
    <String, dynamic>{'booking_id': instance.bookingId};

BoardLoadAddedFrame _$BoardLoadAddedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BoardLoadAddedFrame', json, ($checkedConvert) {
      final val = BoardLoadAddedFrame(
        data: $checkedConvert(
          'data',
          (v) => BoardLoadAdded.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$BoardLoadAddedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BoardLoadAddedFrameToJson(
  BoardLoadAddedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BoardLoadAddedFrameTypeEnumMap[instance.type]!,
};

const _$BoardLoadAddedFrameTypeEnumMap = {
  BoardLoadAddedFrameType.BOARD_LOAD_ADDED: 'board_load_added',
};

BoardLoadAdded _$BoardLoadAddedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BoardLoadAdded',
      json,
      ($checkedConvert) {
        final val = BoardLoadAdded(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          plan: $checkedConvert('plan', (v) => v as bool),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'loadId': 'load_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$BoardLoadAddedToJson(BoardLoadAdded instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'load_id': instance.loadId,
      'plan': instance.plan,
      'to_region_id': instance.toRegionId,
    };

BoardLoadRemovedFrame _$BoardLoadRemovedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('BoardLoadRemovedFrame', json, ($checkedConvert) {
  final val = BoardLoadRemovedFrame(
    data: $checkedConvert(
      'data',
      (v) => BoardLoadRemoved.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$BoardLoadRemovedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$BoardLoadRemovedFrameToJson(
  BoardLoadRemovedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BoardLoadRemovedFrameTypeEnumMap[instance.type]!,
};

const _$BoardLoadRemovedFrameTypeEnumMap = {
  BoardLoadRemovedFrameType.BOARD_LOAD_REMOVED: 'board_load_removed',
};

BoardLoadRemoved _$BoardLoadRemovedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BoardLoadRemoved',
      json,
      ($checkedConvert) {
        final val = BoardLoadRemoved(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          reason: $checkedConvert(
            'reason',
            (v) => $enumDecode(_$BoardRemovalReasonEnumMap, v),
          ),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'loadId': 'load_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$BoardLoadRemovedToJson(BoardLoadRemoved instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'load_id': instance.loadId,
      'reason': _$BoardRemovalReasonEnumMap[instance.reason]!,
      'to_region_id': instance.toRegionId,
    };

const _$BoardRemovalReasonEnumMap = {
  BoardRemovalReason.BOOKED: 'booked',
  BoardRemovalReason.CANCELLED: 'cancelled',
  BoardRemovalReason.EXPIRED: 'expired',
};

BoardLoadUpdatedFrame _$BoardLoadUpdatedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('BoardLoadUpdatedFrame', json, ($checkedConvert) {
  final val = BoardLoadUpdatedFrame(
    data: $checkedConvert(
      'data',
      (v) => BoardLoadUpdated.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$BoardLoadUpdatedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$BoardLoadUpdatedFrameToJson(
  BoardLoadUpdatedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BoardLoadUpdatedFrameTypeEnumMap[instance.type]!,
};

const _$BoardLoadUpdatedFrameTypeEnumMap = {
  BoardLoadUpdatedFrameType.BOARD_LOAD_UPDATED: 'board_load_updated',
};

BoardLoadUpdated _$BoardLoadUpdatedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BoardLoadUpdated',
      json,
      ($checkedConvert) {
        final val = BoardLoadUpdated(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          plan: $checkedConvert('plan', (v) => v as bool),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'loadId': 'load_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$BoardLoadUpdatedToJson(BoardLoadUpdated instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'load_id': instance.loadId,
      'plan': instance.plan,
      'to_region_id': instance.toRegionId,
    };

BoardSubscribedFrame _$BoardSubscribedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('BoardSubscribedFrame', json, ($checkedConvert) {
  final val = BoardSubscribedFrame(
    data: $checkedConvert(
      'data',
      (v) => BoardSubscribed.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$BoardSubscribedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$BoardSubscribedFrameToJson(
  BoardSubscribedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BoardSubscribedFrameTypeEnumMap[instance.type]!,
};

const _$BoardSubscribedFrameTypeEnumMap = {
  BoardSubscribedFrameType.BOARD_SUBSCRIBED: 'board_subscribed',
};

BoardSubscribed _$BoardSubscribedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BoardSubscribed',
      json,
      ($checkedConvert) {
        final val = BoardSubscribed(
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'fromRegionId': 'from_region_id',
        'toRegionId': 'to_region_id',
      },
    );

Map<String, dynamic> _$BoardSubscribedToJson(BoardSubscribed instance) =>
    <String, dynamic>{
      'from_region_id': instance.fromRegionId,
      'to_region_id': instance.toRegionId,
    };

BookingCancelledFrame _$BookingCancelledFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('BookingCancelledFrame', json, ($checkedConvert) {
  final val = BookingCancelledFrame(
    data: $checkedConvert(
      'data',
      (v) => BookingCancelled.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$BookingCancelledFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$BookingCancelledFrameToJson(
  BookingCancelledFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BookingCancelledFrameTypeEnumMap[instance.type]!,
};

const _$BookingCancelledFrameTypeEnumMap = {
  BookingCancelledFrameType.BOOKING_CANCELLED: 'booking_cancelled',
};

BookingCancelled _$BookingCancelledFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BookingCancelled',
      json,
      ($checkedConvert) {
        final val = BookingCancelled(
          bookingId: $checkedConvert('booking_id', (v) => v as String),
          by: $checkedConvert(
            'by',
            (v) => $enumDecode(_$CancelledByEnumMap, v),
          ),
          carrierOrgId: $checkedConvert('carrier_org_id', (v) => v as String),
          currency: $checkedConvert('currency', (v) => v as String),
          driverUserId: $checkedConvert('driver_user_id', (v) => v as String?),
          eventId: $checkedConvert('event_id', (v) => v as String),
          feeAmount: $checkedConvert('fee_amount', (v) => (v as num?)?.toInt()),
          loadCode: $checkedConvert('load_code', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          penalized: $checkedConvert('penalized', (v) => v as bool),
          plan: $checkedConvert('plan', (v) => v as bool),
          reason: $checkedConvert('reason', (v) => v as String),
          reasonCode: $checkedConvert('reason_code', (v) => v as String?),
          shipperOrgId: $checkedConvert('shipper_org_id', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'bookingId': 'booking_id',
        'carrierOrgId': 'carrier_org_id',
        'driverUserId': 'driver_user_id',
        'eventId': 'event_id',
        'feeAmount': 'fee_amount',
        'loadCode': 'load_code',
        'loadId': 'load_id',
        'reasonCode': 'reason_code',
        'shipperOrgId': 'shipper_org_id',
      },
    );

Map<String, dynamic> _$BookingCancelledToJson(BookingCancelled instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'by': _$CancelledByEnumMap[instance.by]!,
      'carrier_org_id': instance.carrierOrgId,
      'currency': instance.currency,
      'driver_user_id': ?instance.driverUserId,
      'event_id': instance.eventId,
      'fee_amount': ?instance.feeAmount,
      'load_code': instance.loadCode,
      'load_id': instance.loadId,
      'penalized': instance.penalized,
      'plan': instance.plan,
      'reason': instance.reason,
      'reason_code': ?instance.reasonCode,
      'shipper_org_id': instance.shipperOrgId,
    };

const _$CancelledByEnumMap = {
  CancelledBy.CARRIER: 'carrier',
  CancelledBy.OPERATOR: 'operator',
  CancelledBy.SHIPPER: 'shipper',
};

BookingCompletedFrame _$BookingCompletedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('BookingCompletedFrame', json, ($checkedConvert) {
  final val = BookingCompletedFrame(
    data: $checkedConvert(
      'data',
      (v) => BookingCompleted.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$BookingCompletedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$BookingCompletedFrameToJson(
  BookingCompletedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BookingCompletedFrameTypeEnumMap[instance.type]!,
};

const _$BookingCompletedFrameTypeEnumMap = {
  BookingCompletedFrameType.BOOKING_COMPLETED: 'booking_completed',
};

BookingCompleted _$BookingCompletedFromJson(
  Map<String, dynamic> json,
) => $checkedCreate(
  'BookingCompleted',
  json,
  ($checkedConvert) {
    final val = BookingCompleted(
      agreedAmount: $checkedConvert('agreed_amount', (v) => (v as num).toInt()),
      bookingId: $checkedConvert('booking_id', (v) => v as String),
      carrierOrgId: $checkedConvert('carrier_org_id', (v) => v as String),
      charges: $checkedConvert(
        'charges',
        (v) => (v as List<dynamic>)
            .map((e) => ChargeLine.fromJson(e as Map<String, dynamic>))
            .toList(),
      ),
      completedAt: $checkedConvert(
        'completed_at',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      completedBy: $checkedConvert(
        'completed_by',
        (v) => $enumDecode(_$CompletedByEnumMap, v),
      ),
      currency: $checkedConvert('currency', (v) => v as String),
      driverUserId: $checkedConvert('driver_user_id', (v) => v as String),
      eventId: $checkedConvert('event_id', (v) => v as String),
      loadCode: $checkedConvert('load_code', (v) => v as String),
      loadId: $checkedConvert('load_id', (v) => v as String),
      settledAmount: $checkedConvert(
        'settled_amount',
        (v) => (v as num).toInt(),
      ),
      shipperOrgId: $checkedConvert('shipper_org_id', (v) => v as String),
      totalAmount: $checkedConvert('total_amount', (v) => (v as num?)?.toInt()),
      waitingAmount: $checkedConvert(
        'waiting_amount',
        (v) => (v as num).toInt(),
      ),
      waitingBillable: $checkedConvert('waiting_billable', (v) => v as bool),
      waitingMinutes: $checkedConvert(
        'waiting_minutes',
        (v) => (v as num).toInt(),
      ),
    );
    return val;
  },
  fieldKeyMap: const {
    'agreedAmount': 'agreed_amount',
    'bookingId': 'booking_id',
    'carrierOrgId': 'carrier_org_id',
    'completedAt': 'completed_at',
    'completedBy': 'completed_by',
    'driverUserId': 'driver_user_id',
    'eventId': 'event_id',
    'loadCode': 'load_code',
    'loadId': 'load_id',
    'settledAmount': 'settled_amount',
    'shipperOrgId': 'shipper_org_id',
    'totalAmount': 'total_amount',
    'waitingAmount': 'waiting_amount',
    'waitingBillable': 'waiting_billable',
    'waitingMinutes': 'waiting_minutes',
  },
);

Map<String, dynamic> _$BookingCompletedToJson(BookingCompleted instance) =>
    <String, dynamic>{
      'agreed_amount': instance.agreedAmount,
      'booking_id': instance.bookingId,
      'carrier_org_id': instance.carrierOrgId,
      'charges': instance.charges,
      'completed_at': ?instance.completedAt?.toIso8601String(),
      'completed_by': _$CompletedByEnumMap[instance.completedBy]!,
      'currency': instance.currency,
      'driver_user_id': instance.driverUserId,
      'event_id': instance.eventId,
      'load_code': instance.loadCode,
      'load_id': instance.loadId,
      'settled_amount': instance.settledAmount,
      'shipper_org_id': instance.shipperOrgId,
      'total_amount': ?instance.totalAmount,
      'waiting_amount': instance.waitingAmount,
      'waiting_billable': instance.waitingBillable,
      'waiting_minutes': instance.waitingMinutes,
    };

const _$CompletedByEnumMap = {
  CompletedBy.CARRIER: 'carrier',
  CompletedBy.OPERATOR: 'operator',
  CompletedBy.SHIPPER: 'shipper',
  CompletedBy.SYSTEM: 'system',
};

ChargeLine _$ChargeLineFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ChargeLine', json, ($checkedConvert) {
      final val = ChargeLine(
        amount: $checkedConvert('amount', (v) => (v as num).toInt()),
        code: $checkedConvert('code', (v) => v as String),
        qty: $checkedConvert('qty', (v) => (v as num).toInt()),
      );
      return val;
    });

Map<String, dynamic> _$ChargeLineToJson(ChargeLine instance) =>
    <String, dynamic>{
      'amount': instance.amount,
      'code': instance.code,
      'qty': instance.qty,
    };

BookingCreatedFrame _$BookingCreatedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookingCreatedFrame', json, ($checkedConvert) {
      final val = BookingCreatedFrame(
        data: $checkedConvert(
          'data',
          (v) => BookingCreated.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$BookingCreatedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BookingCreatedFrameToJson(
  BookingCreatedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BookingCreatedFrameTypeEnumMap[instance.type]!,
};

const _$BookingCreatedFrameTypeEnumMap = {
  BookingCreatedFrameType.BOOKING_CREATED: 'booking_created',
};

BookingCreated _$BookingCreatedFromJson(
  Map<String, dynamic> json,
) => $checkedCreate(
  'BookingCreated',
  json,
  ($checkedConvert) {
    final val = BookingCreated(
      agreedAmount: $checkedConvert('agreed_amount', (v) => (v as num).toInt()),
      bookingId: $checkedConvert('booking_id', (v) => v as String),
      busyFrom: $checkedConvert(
        'busy_from',
        (v) => DateTime.parse(v as String),
      ),
      busyUntil: $checkedConvert(
        'busy_until',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      carrierOrgId: $checkedConvert('carrier_org_id', (v) => v as String),
      currency: $checkedConvert('currency', (v) => v as String),
      driverUserId: $checkedConvert('driver_user_id', (v) => v as String?),
      durationMin: $checkedConvert('duration_min', (v) => (v as num?)?.toInt()),
      eventId: $checkedConvert('event_id', (v) => v as String),
      loadId: $checkedConvert('load_id', (v) => v as String),
      pickupFrom: $checkedConvert(
        'pickup_from',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      pickupTo: $checkedConvert(
        'pickup_to',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      plan: $checkedConvert('plan', (v) => v as bool),
      shipperOrgId: $checkedConvert('shipper_org_id', (v) => v as String),
      shipperUserId: $checkedConvert('shipper_user_id', (v) => v as String),
      source: $checkedConvert(
        'source',
        (v) => $enumDecodeNullable(_$BookingSourceEnumMap, v),
      ),
    );
    return val;
  },
  fieldKeyMap: const {
    'agreedAmount': 'agreed_amount',
    'bookingId': 'booking_id',
    'busyFrom': 'busy_from',
    'busyUntil': 'busy_until',
    'carrierOrgId': 'carrier_org_id',
    'driverUserId': 'driver_user_id',
    'durationMin': 'duration_min',
    'eventId': 'event_id',
    'loadId': 'load_id',
    'pickupFrom': 'pickup_from',
    'pickupTo': 'pickup_to',
    'shipperOrgId': 'shipper_org_id',
    'shipperUserId': 'shipper_user_id',
  },
);

Map<String, dynamic> _$BookingCreatedToJson(BookingCreated instance) =>
    <String, dynamic>{
      'agreed_amount': instance.agreedAmount,
      'booking_id': instance.bookingId,
      'busy_from': instance.busyFrom.toIso8601String(),
      'busy_until': ?instance.busyUntil?.toIso8601String(),
      'carrier_org_id': instance.carrierOrgId,
      'currency': instance.currency,
      'driver_user_id': ?instance.driverUserId,
      'duration_min': ?instance.durationMin,
      'event_id': instance.eventId,
      'load_id': instance.loadId,
      'pickup_from': ?instance.pickupFrom?.toIso8601String(),
      'pickup_to': ?instance.pickupTo?.toIso8601String(),
      'plan': instance.plan,
      'shipper_org_id': instance.shipperOrgId,
      'shipper_user_id': instance.shipperUserId,
      'source': ?_$BookingSourceEnumMap[instance.source],
    };

const _$BookingSourceEnumMap = {BookingSource.INSTANT: 'instant'};

BookingEtaFrame _$BookingEtaFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookingEtaFrame', json, ($checkedConvert) {
      final val = BookingEtaFrame(
        data: $checkedConvert(
          'data',
          (v) => BookingEtaUpdated.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$BookingEtaFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BookingEtaFrameToJson(BookingEtaFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$BookingEtaFrameTypeEnumMap[instance.type]!,
    };

const _$BookingEtaFrameTypeEnumMap = {
  BookingEtaFrameType.BOOKING_ETA: 'booking_eta',
};

BookingEtaUpdated _$BookingEtaUpdatedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'BookingEtaUpdated',
      json,
      ($checkedConvert) {
        final val = BookingEtaUpdated(
          bookingId: $checkedConvert('booking_id', (v) => v as String),
          carrierOrgId: $checkedConvert('carrier_org_id', (v) => v as String),
          driverUserId: $checkedConvert('driver_user_id', (v) => v as String),
          etaFrom: $checkedConvert(
            'eta_from',
            (v) => v == null ? null : DateTime.parse(v as String),
          ),
          etaStop: $checkedConvert(
            'eta_stop',
            (v) => $enumDecodeNullable(_$EtaStopEnumMap, v),
          ),
          etaTo: $checkedConvert(
            'eta_to',
            (v) => v == null ? null : DateTime.parse(v as String),
          ),
          eventId: $checkedConvert('event_id', (v) => v as String),
          freeAt: $checkedConvert(
            'free_at',
            (v) => v == null ? null : DateTime.parse(v as String),
          ),
          freeRegionId: $checkedConvert('free_region_id', (v) => v as String?),
          lastWaypoint: $checkedConvert(
            'last_waypoint',
            (v) => v == null
                ? null
                : PassedWaypoint.fromJson(v as Map<String, dynamic>),
          ),
          loadId: $checkedConvert('load_id', (v) => v as String),
          remainingKm: $checkedConvert(
            'remaining_km',
            (v) => (v as num?)?.toDouble(),
          ),
          shipperOrgId: $checkedConvert('shipper_org_id', (v) => v as String),
          status: $checkedConvert(
            'status',
            (v) => $enumDecode(_$BookingStatusEnumMap, v),
          ),
          truckClass: $checkedConvert('truck_class', (v) => v as String),
        );
        return val;
      },
      fieldKeyMap: const {
        'bookingId': 'booking_id',
        'carrierOrgId': 'carrier_org_id',
        'driverUserId': 'driver_user_id',
        'etaFrom': 'eta_from',
        'etaStop': 'eta_stop',
        'etaTo': 'eta_to',
        'eventId': 'event_id',
        'freeAt': 'free_at',
        'freeRegionId': 'free_region_id',
        'lastWaypoint': 'last_waypoint',
        'loadId': 'load_id',
        'remainingKm': 'remaining_km',
        'shipperOrgId': 'shipper_org_id',
        'truckClass': 'truck_class',
      },
    );

Map<String, dynamic> _$BookingEtaUpdatedToJson(BookingEtaUpdated instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'carrier_org_id': instance.carrierOrgId,
      'driver_user_id': instance.driverUserId,
      'eta_from': ?instance.etaFrom?.toIso8601String(),
      'eta_stop': ?_$EtaStopEnumMap[instance.etaStop],
      'eta_to': ?instance.etaTo?.toIso8601String(),
      'event_id': instance.eventId,
      'free_at': ?instance.freeAt?.toIso8601String(),
      'free_region_id': ?instance.freeRegionId,
      'last_waypoint': ?instance.lastWaypoint,
      'load_id': instance.loadId,
      'remaining_km': ?instance.remainingKm,
      'shipper_org_id': instance.shipperOrgId,
      'status': _$BookingStatusEnumMap[instance.status]!,
      'truck_class': instance.truckClass,
    };

const _$EtaStopEnumMap = {EtaStop.DROPOFF: 'dropoff', EtaStop.PICKUP: 'pickup'};

const _$BookingStatusEnumMap = {
  BookingStatus.AT_PICKUP: 'at_pickup',
  BookingStatus.CANCELLED: 'cancelled',
  BookingStatus.COMPLETED: 'completed',
  BookingStatus.CONFIRMED: 'confirmed',
  BookingStatus.DELIVERED: 'delivered',
  BookingStatus.EN_ROUTE: 'en_route',
  BookingStatus.IN_TRANSIT: 'in_transit',
  BookingStatus.LOADING: 'loading',
};

PassedWaypoint _$PassedWaypointFromJson(Map<String, dynamic> json) =>
    $checkedCreate('PassedWaypoint', json, ($checkedConvert) {
      final val = PassedWaypoint(
        passedAt: $checkedConvert(
          'passed_at',
          (v) => DateTime.parse(v as String),
        ),
        regionId: $checkedConvert('region_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'passedAt': 'passed_at', 'regionId': 'region_id'});

Map<String, dynamic> _$PassedWaypointToJson(PassedWaypoint instance) =>
    <String, dynamic>{
      'passed_at': instance.passedAt.toIso8601String(),
      'region_id': instance.regionId,
    };

BookingStatusFrame _$BookingStatusFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookingStatusFrame', json, ($checkedConvert) {
      final val = BookingStatusFrame(
        data: $checkedConvert(
          'data',
          (v) => BookingStatusChanged.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$BookingStatusFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BookingStatusFrameToJson(BookingStatusFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$BookingStatusFrameTypeEnumMap[instance.type]!,
    };

const _$BookingStatusFrameTypeEnumMap = {
  BookingStatusFrameType.BOOKING_STATUS: 'booking_status',
};

BookingStatusChanged _$BookingStatusChangedFromJson(
  Map<String, dynamic> json,
) => $checkedCreate(
  'BookingStatusChanged',
  json,
  ($checkedConvert) {
    final val = BookingStatusChanged(
      bookingId: $checkedConvert('booking_id', (v) => v as String),
      carrierOrgId: $checkedConvert('carrier_org_id', (v) => v as String),
      eventId: $checkedConvert('event_id', (v) => v as String),
      loadId: $checkedConvert('load_id', (v) => v as String),
      occurredAt: $checkedConvert(
        'occurred_at',
        (v) => DateTime.parse(v as String),
      ),
      shipperOrgId: $checkedConvert('shipper_org_id', (v) => v as String),
      status: $checkedConvert(
        'status',
        (v) => $enumDecode(_$BookingStatusEnumMap, v),
      ),
    );
    return val;
  },
  fieldKeyMap: const {
    'bookingId': 'booking_id',
    'carrierOrgId': 'carrier_org_id',
    'eventId': 'event_id',
    'loadId': 'load_id',
    'occurredAt': 'occurred_at',
    'shipperOrgId': 'shipper_org_id',
  },
);

Map<String, dynamic> _$BookingStatusChangedToJson(
  BookingStatusChanged instance,
) => <String, dynamic>{
  'booking_id': instance.bookingId,
  'carrier_org_id': instance.carrierOrgId,
  'event_id': instance.eventId,
  'load_id': instance.loadId,
  'occurred_at': instance.occurredAt.toIso8601String(),
  'shipper_org_id': instance.shipperOrgId,
  'status': _$BookingStatusEnumMap[instance.status]!,
};

BookingUpdatedFrame _$BookingUpdatedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookingUpdatedFrame', json, ($checkedConvert) {
      final val = BookingUpdatedFrame(
        data: $checkedConvert(
          'data',
          (v) => BookingUpdated.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$BookingUpdatedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$BookingUpdatedFrameToJson(
  BookingUpdatedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$BookingUpdatedFrameTypeEnumMap[instance.type]!,
};

const _$BookingUpdatedFrameTypeEnumMap = {
  BookingUpdatedFrameType.BOOKING_UPDATED: 'booking_updated',
};

BookingUpdated _$BookingUpdatedFromJson(Map<String, dynamic> json) =>
    $checkedCreate('BookingUpdated', json, ($checkedConvert) {
      final val = BookingUpdated(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
        change: $checkedConvert(
          'change',
          (v) => $enumDecode(_$BookingChangeEnumMap, v),
        ),
        issueId: $checkedConvert('issue_id', (v) => v as String?),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id', 'issueId': 'issue_id'});

Map<String, dynamic> _$BookingUpdatedToJson(BookingUpdated instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'change': _$BookingChangeEnumMap[instance.change]!,
      'issue_id': ?instance.issueId,
    };

const _$BookingChangeEnumMap = {
  BookingChange.ISSUE_OPENED: 'issue_opened',
  BookingChange.ISSUE_RESOLVED: 'issue_resolved',
  BookingChange.WAITING_STARTED: 'waiting_started',
};

ChatMessageFrame _$ChatMessageFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ChatMessageFrame', json, ($checkedConvert) {
      final val = ChatMessageFrame(
        data: $checkedConvert(
          'data',
          (v) => ChatMessageCreated.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$ChatMessageFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$ChatMessageFrameToJson(ChatMessageFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$ChatMessageFrameTypeEnumMap[instance.type]!,
    };

const _$ChatMessageFrameTypeEnumMap = {
  ChatMessageFrameType.CHAT_MESSAGE: 'chat_message',
};

ChatMessageCreated _$ChatMessageCreatedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'ChatMessageCreated',
      json,
      ($checkedConvert) {
        final val = ChatMessageCreated(
          bookingId: $checkedConvert('booking_id', (v) => v as String),
          clientMessageId: $checkedConvert(
            'client_message_id',
            (v) => v as String?,
          ),
          createdAt: $checkedConvert(
            'created_at',
            (v) => DateTime.parse(v as String),
          ),
          eventId: $checkedConvert('event_id', (v) => v as String),
          kind: $checkedConvert(
            'kind',
            (v) => $enumDecode(_$ChatMessageKindEnumMap, v),
          ),
          lat: $checkedConvert('lat', (v) => (v as num?)?.toDouble()),
          lng: $checkedConvert('lng', (v) => (v as num?)?.toDouble()),
          loadId: $checkedConvert('load_id', (v) => v as String),
          messageId: $checkedConvert('message_id', (v) => v as String),
          photoFileId: $checkedConvert('photo_file_id', (v) => v as String?),
          quickReplyCode: $checkedConvert(
            'quick_reply_code',
            (v) => v as String?,
          ),
          recipientUserIds: $checkedConvert(
            'recipient_user_ids',
            (v) => (v as List<dynamic>).map((e) => e as String).toList(),
          ),
          senderSide: $checkedConvert(
            'sender_side',
            (v) => $enumDecode(_$ChatSideEnumMap, v),
          ),
          senderUserId: $checkedConvert('sender_user_id', (v) => v as String),
          text: $checkedConvert('text', (v) => v as String?),
        );
        return val;
      },
      fieldKeyMap: const {
        'bookingId': 'booking_id',
        'clientMessageId': 'client_message_id',
        'createdAt': 'created_at',
        'eventId': 'event_id',
        'loadId': 'load_id',
        'messageId': 'message_id',
        'photoFileId': 'photo_file_id',
        'quickReplyCode': 'quick_reply_code',
        'recipientUserIds': 'recipient_user_ids',
        'senderSide': 'sender_side',
        'senderUserId': 'sender_user_id',
      },
    );

Map<String, dynamic> _$ChatMessageCreatedToJson(ChatMessageCreated instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'client_message_id': ?instance.clientMessageId,
      'created_at': instance.createdAt.toIso8601String(),
      'event_id': instance.eventId,
      'kind': _$ChatMessageKindEnumMap[instance.kind]!,
      'lat': ?instance.lat,
      'lng': ?instance.lng,
      'load_id': instance.loadId,
      'message_id': instance.messageId,
      'photo_file_id': ?instance.photoFileId,
      'quick_reply_code': ?instance.quickReplyCode,
      'recipient_user_ids': instance.recipientUserIds,
      'sender_side': _$ChatSideEnumMap[instance.senderSide]!,
      'sender_user_id': instance.senderUserId,
      'text': ?instance.text,
    };

const _$ChatMessageKindEnumMap = {
  ChatMessageKind.LOCATION: 'location',
  ChatMessageKind.PHOTO: 'photo',
  ChatMessageKind.QUICK_REPLY: 'quick_reply',
  ChatMessageKind.TEXT: 'text',
};

const _$ChatSideEnumMap = {
  ChatSide.CARRIER: 'carrier',
  ChatSide.SHIPPER: 'shipper',
};

ChatReadFrame _$ChatReadFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ChatReadFrame', json, ($checkedConvert) {
      final val = ChatReadFrame(
        data: $checkedConvert(
          'data',
          (v) => ChatRead.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$ChatReadFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$ChatReadFrameToJson(ChatReadFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$ChatReadFrameTypeEnumMap[instance.type]!,
    };

const _$ChatReadFrameTypeEnumMap = {ChatReadFrameType.CHAT_READ: 'chat_read'};

ChatRead _$ChatReadFromJson(Map<String, dynamic> json) => $checkedCreate(
  'ChatRead',
  json,
  ($checkedConvert) {
    final val = ChatRead(
      bookingId: $checkedConvert('booking_id', (v) => v as String),
      eventId: $checkedConvert('event_id', (v) => v as String),
      lastReadMessageId: $checkedConvert(
        'last_read_message_id',
        (v) => v as String,
      ),
      readAt: $checkedConvert('read_at', (v) => DateTime.parse(v as String)),
      readerSide: $checkedConvert(
        'reader_side',
        (v) => $enumDecode(_$ChatSideEnumMap, v),
      ),
      readerUserId: $checkedConvert('reader_user_id', (v) => v as String),
      recipientUserIds: $checkedConvert(
        'recipient_user_ids',
        (v) => (v as List<dynamic>).map((e) => e as String).toList(),
      ),
    );
    return val;
  },
  fieldKeyMap: const {
    'bookingId': 'booking_id',
    'eventId': 'event_id',
    'lastReadMessageId': 'last_read_message_id',
    'readAt': 'read_at',
    'readerSide': 'reader_side',
    'readerUserId': 'reader_user_id',
    'recipientUserIds': 'recipient_user_ids',
  },
);

Map<String, dynamic> _$ChatReadToJson(ChatRead instance) => <String, dynamic>{
  'booking_id': instance.bookingId,
  'event_id': instance.eventId,
  'last_read_message_id': instance.lastReadMessageId,
  'read_at': instance.readAt.toIso8601String(),
  'reader_side': _$ChatSideEnumMap[instance.readerSide]!,
  'reader_user_id': instance.readerUserId,
  'recipient_user_ids': instance.recipientUserIds,
};

ChatTypingFrame _$ChatTypingFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ChatTypingFrame', json, ($checkedConvert) {
      final val = ChatTypingFrame(
        data: $checkedConvert(
          'data',
          (v) => ChatTyping.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$ChatTypingFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$ChatTypingFrameToJson(ChatTypingFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$ChatTypingFrameTypeEnumMap[instance.type]!,
    };

const _$ChatTypingFrameTypeEnumMap = {
  ChatTypingFrameType.CHAT_TYPING: 'chat_typing',
};

ChatTyping _$ChatTypingFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ChatTyping', json, ($checkedConvert) {
      final val = ChatTyping(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
        userId: $checkedConvert('user_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id', 'userId': 'user_id'});

Map<String, dynamic> _$ChatTypingToJson(ChatTyping instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'user_id': instance.userId,
    };

ConnectedFrame _$ConnectedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ConnectedFrame', json, ($checkedConvert) {
      final val = ConnectedFrame(
        data: $checkedConvert(
          'data',
          (v) => Connected.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$ConnectedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$ConnectedFrameToJson(ConnectedFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$ConnectedFrameTypeEnumMap[instance.type]!,
    };

const _$ConnectedFrameTypeEnumMap = {ConnectedFrameType.CONNECTED: 'connected'};

Connected _$ConnectedFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Connected', json, ($checkedConvert) {
      final val = Connected(
        config: $checkedConvert(
          'config',
          (v) => ConnectionConfig.fromJson(v as Map<String, dynamic>),
        ),
        connId: $checkedConvert('conn_id', (v) => v as String),
        userId: $checkedConvert('user_id', (v) => v as String),
        worker: $checkedConvert('worker', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'connId': 'conn_id', 'userId': 'user_id'});

Map<String, dynamic> _$ConnectedToJson(Connected instance) => <String, dynamic>{
  'config': instance.config,
  'conn_id': instance.connId,
  'user_id': instance.userId,
  'worker': instance.worker,
};

ConnectionConfig _$ConnectionConfigFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'ConnectionConfig',
      json,
      ($checkedConvert) {
        final val = ConnectionConfig(
          locationSampleSeconds: $checkedConvert(
            'location_sample_seconds',
            (v) => (v as num).toInt(),
          ),
          offerGraceSeconds: $checkedConvert(
            'offer_grace_seconds',
            (v) => (v as num).toInt(),
          ),
          offerTtlSeconds: $checkedConvert(
            'offer_ttl_seconds',
            (v) => (v as num).toInt(),
          ),
        );
        return val;
      },
      fieldKeyMap: const {
        'locationSampleSeconds': 'location_sample_seconds',
        'offerGraceSeconds': 'offer_grace_seconds',
        'offerTtlSeconds': 'offer_ttl_seconds',
      },
    );

Map<String, dynamic> _$ConnectionConfigToJson(ConnectionConfig instance) =>
    <String, dynamic>{
      'location_sample_seconds': instance.locationSampleSeconds,
      'offer_grace_seconds': instance.offerGraceSeconds,
      'offer_ttl_seconds': instance.offerTtlSeconds,
    };

DriverLocationFrame _$DriverLocationFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('DriverLocationFrame', json, ($checkedConvert) {
      final val = DriverLocationFrame(
        data: $checkedConvert(
          'data',
          (v) => DriverLocation.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$DriverLocationFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$DriverLocationFrameToJson(
  DriverLocationFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$DriverLocationFrameTypeEnumMap[instance.type]!,
};

const _$DriverLocationFrameTypeEnumMap = {
  DriverLocationFrameType.DRIVER_LOCATION: 'driver_location',
};

DriverLocation _$DriverLocationFromJson(Map<String, dynamic> json) =>
    $checkedCreate('DriverLocation', json, ($checkedConvert) {
      final val = DriverLocation(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
        heading: $checkedConvert('heading', (v) => (v as num?)?.toDouble()),
        lat: $checkedConvert('lat', (v) => (v as num).toDouble()),
        lng: $checkedConvert('lng', (v) => (v as num).toDouble()),
        speed: $checkedConvert('speed', (v) => (v as num?)?.toDouble()),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id'});

Map<String, dynamic> _$DriverLocationToJson(DriverLocation instance) =>
    <String, dynamic>{
      'booking_id': instance.bookingId,
      'heading': ?instance.heading,
      'lat': instance.lat,
      'lng': instance.lng,
      'speed': ?instance.speed,
    };

ErrorFrame _$ErrorFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('ErrorFrame', json, ($checkedConvert) {
      final val = ErrorFrame(
        data: $checkedConvert(
          'data',
          (v) => FrameError.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$ErrorFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$ErrorFrameToJson(ErrorFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$ErrorFrameTypeEnumMap[instance.type]!,
    };

const _$ErrorFrameTypeEnumMap = {ErrorFrameType.ERROR: 'error'};

FrameError _$FrameErrorFromJson(Map<String, dynamic> json) =>
    $checkedCreate('FrameError', json, ($checkedConvert) {
      final val = FrameError(
        code: $checkedConvert('code', (v) => v as String),
        detail: $checkedConvert('detail', (v) => v as String?),
        key: $checkedConvert('key', (v) => v as String?),
        params: $checkedConvert('params', (v) => v as Map<String, dynamic>?),
        status: $checkedConvert('status', (v) => (v as num?)?.toInt()),
        title: $checkedConvert('title', (v) => v as String?),
        type: $checkedConvert('type', (v) => v as String?),
      );
      return val;
    });

Map<String, dynamic> _$FrameErrorToJson(FrameError instance) =>
    <String, dynamic>{
      'code': instance.code,
      'detail': ?instance.detail,
      'key': ?instance.key,
      'params': ?instance.params,
      'status': ?instance.status,
      'title': ?instance.title,
      'type': ?instance.type,
    };

KickedFrame _$KickedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('KickedFrame', json, ($checkedConvert) {
      final val = KickedFrame(
        data: $checkedConvert(
          'data',
          (v) => Kicked.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$KickedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$KickedFrameToJson(KickedFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$KickedFrameTypeEnumMap[instance.type]!,
    };

const _$KickedFrameTypeEnumMap = {KickedFrameType.KICKED: 'kicked'};

Kicked _$KickedFromJson(Map<String, dynamic> json) => $checkedCreate(
  'Kicked',
  json,
  ($checkedConvert) {
    final val = Kicked(reason: $checkedConvert('reason', (v) => v as String?));
    return val;
  },
);

Map<String, dynamic> _$KickedToJson(Kicked instance) => <String, dynamic>{
  'reason': ?instance.reason,
};

NotificationFrame _$NotificationFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('NotificationFrame', json, ($checkedConvert) {
      final val = NotificationFrame(
        data: $checkedConvert(
          'data',
          (v) => Notification.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$NotificationFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$NotificationFrameToJson(NotificationFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$NotificationFrameTypeEnumMap[instance.type]!,
    };

const _$NotificationFrameTypeEnumMap = {
  NotificationFrameType.NOTIFICATION: 'notification',
};

Notification _$NotificationFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Notification', json, ($checkedConvert) {
      final val = Notification(
        body: $checkedConvert('body', (v) => v as String?),
        kind: $checkedConvert('kind', (v) => v as String?),
        notificationId: $checkedConvert('notification_id', (v) => v as String?),
        title: $checkedConvert('title', (v) => v as String?),
      );
      return val;
    }, fieldKeyMap: const {'notificationId': 'notification_id'});

Map<String, dynamic> _$NotificationToJson(Notification instance) =>
    <String, dynamic>{
      'body': ?instance.body,
      'kind': ?instance.kind,
      'notification_id': ?instance.notificationId,
      'title': ?instance.title,
    };

OfferProposedFrame _$OfferProposedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfferProposedFrame', json, ($checkedConvert) {
      final val = OfferProposedFrame(
        data: $checkedConvert(
          'data',
          (v) => OfferProposed.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OfferProposedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OfferProposedFrameToJson(OfferProposedFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$OfferProposedFrameTypeEnumMap[instance.type]!,
    };

const _$OfferProposedFrameTypeEnumMap = {
  OfferProposedFrameType.OFFER_PROPOSED: 'offer_proposed',
};

OfferProposed _$OfferProposedFromJson(
  Map<String, dynamic> json,
) => $checkedCreate(
  'OfferProposed',
  json,
  ($checkedConvert) {
    final val = OfferProposed(
      amount: $checkedConvert('amount', (v) => (v as num).toInt()),
      cargoCategory: $checkedConvert('cargo_category', (v) => v as String),
      currency: $checkedConvert('currency', (v) => v as String),
      deadlineAt: $checkedConvert('deadline_at', (v) => (v as num).toInt()),
      distanceKm: $checkedConvert(
        'distance_km',
        (v) => (v as num?)?.toDouble(),
      ),
      dropoffAddress: $checkedConvert('dropoff_address', (v) => v as String?),
      durationMin: $checkedConvert('duration_min', (v) => (v as num?)?.toInt()),
      fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
      isUrgent: $checkedConvert('is_urgent', (v) => v as bool),
      loadId: $checkedConvert('load_id', (v) => v as String),
      loadersCount: $checkedConvert('loaders_count', (v) => (v as num).toInt()),
      loadingMethod: $checkedConvert(
        'loading_method',
        (v) => $enumDecode(_$LoadingMethodEnumMap, v),
      ),
      needsRefrigeration: $checkedConvert(
        'needs_refrigeration',
        (v) => v as bool,
      ),
      paymentMethod: $checkedConvert(
        'payment_method',
        (v) => $enumDecode(_$PaymentMethodEnumMap, v),
      ),
      pickupAddress: $checkedConvert('pickup_address', (v) => v as String?),
      pickupDistanceKm: $checkedConvert(
        'pickup_distance_km',
        (v) => (v as num?)?.toDouble(),
      ),
      pickupFrom: $checkedConvert(
        'pickup_from',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      pickupTo: $checkedConvert(
        'pickup_to',
        (v) => v == null ? null : DateTime.parse(v as String),
      ),
      serverTime: $checkedConvert(
        'server_time',
        (v) => DateTime.parse(v as String),
      ),
      stopsCount: $checkedConvert('stops_count', (v) => (v as num).toInt()),
      toRegionId: $checkedConvert('to_region_id', (v) => v as String),
      truckClass: $checkedConvert('truck_class', (v) => v as String),
      volumeM3: $checkedConvert('volume_m3', (v) => (v as num?)?.toDouble()),
      weightKg: $checkedConvert('weight_kg', (v) => (v as num).toInt()),
    );
    return val;
  },
  fieldKeyMap: const {
    'cargoCategory': 'cargo_category',
    'deadlineAt': 'deadline_at',
    'distanceKm': 'distance_km',
    'dropoffAddress': 'dropoff_address',
    'durationMin': 'duration_min',
    'fromRegionId': 'from_region_id',
    'isUrgent': 'is_urgent',
    'loadId': 'load_id',
    'loadersCount': 'loaders_count',
    'loadingMethod': 'loading_method',
    'needsRefrigeration': 'needs_refrigeration',
    'paymentMethod': 'payment_method',
    'pickupAddress': 'pickup_address',
    'pickupDistanceKm': 'pickup_distance_km',
    'pickupFrom': 'pickup_from',
    'pickupTo': 'pickup_to',
    'serverTime': 'server_time',
    'stopsCount': 'stops_count',
    'toRegionId': 'to_region_id',
    'truckClass': 'truck_class',
    'volumeM3': 'volume_m3',
    'weightKg': 'weight_kg',
  },
);

Map<String, dynamic> _$OfferProposedToJson(OfferProposed instance) =>
    <String, dynamic>{
      'amount': instance.amount,
      'cargo_category': instance.cargoCategory,
      'currency': instance.currency,
      'deadline_at': instance.deadlineAt,
      'distance_km': ?instance.distanceKm,
      'dropoff_address': ?instance.dropoffAddress,
      'duration_min': ?instance.durationMin,
      'from_region_id': instance.fromRegionId,
      'is_urgent': instance.isUrgent,
      'load_id': instance.loadId,
      'loaders_count': instance.loadersCount,
      'loading_method': _$LoadingMethodEnumMap[instance.loadingMethod]!,
      'needs_refrigeration': instance.needsRefrigeration,
      'payment_method': _$PaymentMethodEnumMap[instance.paymentMethod]!,
      'pickup_address': ?instance.pickupAddress,
      'pickup_distance_km': ?instance.pickupDistanceKm,
      'pickup_from': ?instance.pickupFrom?.toIso8601String(),
      'pickup_to': ?instance.pickupTo?.toIso8601String(),
      'server_time': instance.serverTime.toIso8601String(),
      'stops_count': instance.stopsCount,
      'to_region_id': instance.toRegionId,
      'truck_class': instance.truckClass,
      'volume_m3': ?instance.volumeM3,
      'weight_kg': instance.weightKg,
    };

const _$LoadingMethodEnumMap = {
  LoadingMethod.REAR: 'rear',
  LoadingMethod.TOP: 'top',
};

const _$PaymentMethodEnumMap = {
  PaymentMethod.CARD: 'card',
  PaymentMethod.CASH: 'cash',
  PaymentMethod.MIXED: 'mixed',
  PaymentMethod.TRANSFER: 'transfer',
};

OfferResultFrame _$OfferResultFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfferResultFrame', json, ($checkedConvert) {
      final val = OfferResultFrame(
        data: $checkedConvert(
          'data',
          (v) => OfferResult.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OfferResultFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OfferResultFrameToJson(OfferResultFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$OfferResultFrameTypeEnumMap[instance.type]!,
    };

const _$OfferResultFrameTypeEnumMap = {
  OfferResultFrameType.OFFER_RESULT: 'offer_result',
};

OfferResult _$OfferResultFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfferResult', json, ($checkedConvert) {
      final val = OfferResult(
        bookingId: $checkedConvert('booking_id', (v) => v as String?),
        loadId: $checkedConvert('load_id', (v) => v as String),
        outcome: $checkedConvert(
          'outcome',
          (v) => $enumDecode(_$OfferOutcomeEnumMap, v),
        ),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id', 'loadId': 'load_id'});

Map<String, dynamic> _$OfferResultToJson(OfferResult instance) =>
    <String, dynamic>{
      'booking_id': ?instance.bookingId,
      'load_id': instance.loadId,
      'outcome': _$OfferOutcomeEnumMap[instance.outcome]!,
    };

const _$OfferOutcomeEnumMap = {
  OfferOutcome.DECLINED: 'declined',
  OfferOutcome.EXPIRED: 'expired',
  OfferOutcome.LOST: 'lost',
  OfferOutcome.WITHDRAWN: 'withdrawn',
  OfferOutcome.WON: 'won',
};

OffersBlockedFrame _$OffersBlockedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OffersBlockedFrame', json, ($checkedConvert) {
      final val = OffersBlockedFrame(
        data: $checkedConvert(
          'data',
          (v) => OffersBlocked.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OffersBlockedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OffersBlockedFrameToJson(OffersBlockedFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$OffersBlockedFrameTypeEnumMap[instance.type]!,
    };

const _$OffersBlockedFrameTypeEnumMap = {
  OffersBlockedFrameType.OFFERS_BLOCKED: 'offers_blocked',
};

OffersBlocked _$OffersBlockedFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OffersBlocked', json, ($checkedConvert) {
      final val = OffersBlocked(
        reason: $checkedConvert(
          'reason',
          (v) => $enumDecode(_$OffersBlockedReasonEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OffersBlockedToJson(OffersBlocked instance) =>
    <String, dynamic>{'reason': _$OffersBlockedReasonEnumMap[instance.reason]!};

const _$OffersBlockedReasonEnumMap = {
  OffersBlockedReason.BALANCE_INSUFFICIENT: 'BALANCE_INSUFFICIENT',
  OffersBlockedReason.SUBSCRIPTION_REQUIRED: 'SUBSCRIPTION_REQUIRED',
};

OffersUnblockedFrame _$OffersUnblockedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('OffersUnblockedFrame', json, ($checkedConvert) {
  final val = OffersUnblockedFrame(
    data: $checkedConvert('data', (v) => v as Map<String, dynamic>),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$OffersUnblockedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$OffersUnblockedFrameToJson(
  OffersUnblockedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$OffersUnblockedFrameTypeEnumMap[instance.type]!,
};

const _$OffersUnblockedFrameTypeEnumMap = {
  OffersUnblockedFrameType.OFFERS_UNBLOCKED: 'offers_unblocked',
};

OfflineFrame _$OfflineFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OfflineFrame', json, ($checkedConvert) {
      final val = OfflineFrame(
        data: $checkedConvert(
          'data',
          (v) => Offline.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OfflineFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OfflineFrameToJson(OfflineFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$OfflineFrameTypeEnumMap[instance.type]!,
    };

const _$OfflineFrameTypeEnumMap = {OfflineFrameType.OFFLINE: 'offline'};

Offline _$OfflineFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Offline', json, ($checkedConvert) {
      final val = Offline(
        reason: $checkedConvert(
          'reason',
          (v) => $enumDecodeNullable(_$OfflineReasonEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OfflineToJson(Offline instance) => <String, dynamic>{
  'reason': ?_$OfflineReasonEnumMap[instance.reason],
};

const _$OfflineReasonEnumMap = {
  OfflineReason.DOCUMENTS_EXPIRED: 'documents_expired',
  OfflineReason.VEHICLE_UNAVAILABLE: 'vehicle_unavailable',
  OfflineReason.VERIFICATION_REJECTED: 'verification_rejected',
};

OnlineFrame _$OnlineFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('OnlineFrame', json, ($checkedConvert) {
      final val = OnlineFrame(
        data: $checkedConvert(
          'data',
          (v) => Online.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$OnlineFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$OnlineFrameToJson(OnlineFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$OnlineFrameTypeEnumMap[instance.type]!,
    };

const _$OnlineFrameTypeEnumMap = {OnlineFrameType.ONLINE: 'online'};

Online _$OnlineFromJson(Map<String, dynamic> json) => $checkedCreate(
  'Online',
  json,
  ($checkedConvert) {
    final val = Online(
      bidirectional: $checkedConvert('bidirectional', (v) => v as bool),
      fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
      offersBlocked: $checkedConvert(
        'offers_blocked',
        (v) => $enumDecodeNullable(_$OffersBlockedReasonEnumMap, v),
      ),
      queuePosition: $checkedConvert(
        'queue_position',
        (v) => (v as num).toInt(),
      ),
      toRegionId: $checkedConvert('to_region_id', (v) => v as String),
      vehicleId: $checkedConvert('vehicle_id', (v) => v as String),
    );
    return val;
  },
  fieldKeyMap: const {
    'fromRegionId': 'from_region_id',
    'offersBlocked': 'offers_blocked',
    'queuePosition': 'queue_position',
    'toRegionId': 'to_region_id',
    'vehicleId': 'vehicle_id',
  },
);

Map<String, dynamic> _$OnlineToJson(Online instance) => <String, dynamic>{
  'bidirectional': instance.bidirectional,
  'from_region_id': instance.fromRegionId,
  'offers_blocked': ?_$OffersBlockedReasonEnumMap[instance.offersBlocked],
  'queue_position': instance.queuePosition,
  'to_region_id': instance.toRegionId,
  'vehicle_id': instance.vehicleId,
};

PongFrame _$PongFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('PongFrame', json, ($checkedConvert) {
      final val = PongFrame(
        data: $checkedConvert('data', (v) => v as Map<String, dynamic>),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$PongFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$PongFrameToJson(PongFrame instance) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$PongFrameTypeEnumMap[instance.type]!,
};

const _$PongFrameTypeEnumMap = {PongFrameType.PONG: 'pong'};

SearchExhaustedFrame _$SearchExhaustedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('SearchExhaustedFrame', json, ($checkedConvert) {
  final val = SearchExhaustedFrame(
    data: $checkedConvert(
      'data',
      (v) => SearchExhausted.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$SearchExhaustedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$SearchExhaustedFrameToJson(
  SearchExhaustedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$SearchExhaustedFrameTypeEnumMap[instance.type]!,
};

const _$SearchExhaustedFrameTypeEnumMap = {
  SearchExhaustedFrameType.SEARCH_EXHAUSTED: 'search_exhausted',
};

SearchExhausted _$SearchExhaustedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'SearchExhausted',
      json,
      ($checkedConvert) {
        final val = SearchExhausted(
          amount: $checkedConvert('amount', (v) => (v as num).toInt()),
          currency: $checkedConvert('currency', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          seen: $checkedConvert('seen', (v) => (v as num).toInt()),
          suggestedAmount: $checkedConvert(
            'suggested_amount',
            (v) => (v as num).toInt(),
          ),
        );
        return val;
      },
      fieldKeyMap: const {
        'loadId': 'load_id',
        'suggestedAmount': 'suggested_amount',
      },
    );

Map<String, dynamic> _$SearchExhaustedToJson(SearchExhausted instance) =>
    <String, dynamic>{
      'amount': instance.amount,
      'currency': instance.currency,
      'load_id': instance.loadId,
      'seen': instance.seen,
      'suggested_amount': instance.suggestedAmount,
    };

SearchProgressFrame _$SearchProgressFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SearchProgressFrame', json, ($checkedConvert) {
      final val = SearchProgressFrame(
        data: $checkedConvert(
          'data',
          (v) => SearchProgress.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$SearchProgressFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$SearchProgressFrameToJson(
  SearchProgressFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$SearchProgressFrameTypeEnumMap[instance.type]!,
};

const _$SearchProgressFrameTypeEnumMap = {
  SearchProgressFrameType.SEARCH_PROGRESS: 'search_progress',
};

SearchProgress _$SearchProgressFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SearchProgress', json, ($checkedConvert) {
      final val = SearchProgress(
        loadId: $checkedConvert('load_id', (v) => v as String),
        seen: $checkedConvert('seen', (v) => (v as num).toInt()),
        startedAt: $checkedConvert(
          'started_at',
          (v) => DateTime.parse(v as String),
        ),
      );
      return val;
    }, fieldKeyMap: const {'loadId': 'load_id', 'startedAt': 'started_at'});

Map<String, dynamic> _$SearchProgressToJson(SearchProgress instance) =>
    <String, dynamic>{
      'load_id': instance.loadId,
      'seen': instance.seen,
      'started_at': instance.startedAt.toIso8601String(),
    };

SessionStartedFrame _$SessionStartedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SessionStartedFrame', json, ($checkedConvert) {
      final val = SessionStartedFrame(
        data: $checkedConvert(
          'data',
          (v) => SessionStarted.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$SessionStartedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$SessionStartedFrameToJson(
  SessionStartedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$SessionStartedFrameTypeEnumMap[instance.type]!,
};

const _$SessionStartedFrameTypeEnumMap = {
  SessionStartedFrameType.SESSION_STARTED: 'session_started',
};

SessionStarted _$SessionStartedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'SessionStarted',
      json,
      ($checkedConvert) {
        final val = SessionStarted(
          at: $checkedConvert(
            'at',
            (v) => v == null ? null : DateTime.parse(v as String),
          ),
          deviceId: $checkedConvert('device_id', (v) => v as String?),
          installationId: $checkedConvert(
            'installation_id',
            (v) => v as String?,
          ),
          model: $checkedConvert('model', (v) => v as String?),
          newDevice: $checkedConvert('new_device', (v) => v as bool),
          platform: $checkedConvert(
            'platform',
            (v) => $enumDecodeNullable(_$DevicePlatformEnumMap, v),
          ),
        );
        return val;
      },
      fieldKeyMap: const {
        'deviceId': 'device_id',
        'installationId': 'installation_id',
        'newDevice': 'new_device',
      },
    );

Map<String, dynamic> _$SessionStartedToJson(SessionStarted instance) =>
    <String, dynamic>{
      'at': ?instance.at?.toIso8601String(),
      'device_id': ?instance.deviceId,
      'installation_id': ?instance.installationId,
      'model': ?instance.model,
      'new_device': instance.newDevice,
      'platform': ?_$DevicePlatformEnumMap[instance.platform],
    };

const _$DevicePlatformEnumMap = {
  DevicePlatform.ANDROID: 'android',
  DevicePlatform.IOS: 'ios',
  DevicePlatform.WEB: 'web',
};

SubscribedFrame _$SubscribedFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SubscribedFrame', json, ($checkedConvert) {
      final val = SubscribedFrame(
        data: $checkedConvert(
          'data',
          (v) => Subscribed.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$SubscribedFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$SubscribedFrameToJson(SubscribedFrame instance) =>
    <String, dynamic>{
      'data': instance.data,
      'ts': instance.ts.toIso8601String(),
      'type': _$SubscribedFrameTypeEnumMap[instance.type]!,
    };

const _$SubscribedFrameTypeEnumMap = {
  SubscribedFrameType.SUBSCRIBED: 'subscribed',
};

Subscribed _$SubscribedFromJson(Map<String, dynamic> json) =>
    $checkedCreate('Subscribed', json, ($checkedConvert) {
      final val = Subscribed(
        bookingId: $checkedConvert('booking_id', (v) => v as String),
      );
      return val;
    }, fieldKeyMap: const {'bookingId': 'booking_id'});

Map<String, dynamic> _$SubscribedToJson(Subscribed instance) =>
    <String, dynamic>{'booking_id': instance.bookingId};

SupportMessageFrame _$SupportMessageFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('SupportMessageFrame', json, ($checkedConvert) {
      final val = SupportMessageFrame(
        data: $checkedConvert(
          'data',
          (v) => SupportMessageCreated.fromJson(v as Map<String, dynamic>),
        ),
        ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
        type: $checkedConvert(
          'type',
          (v) => $enumDecode(_$SupportMessageFrameTypeEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$SupportMessageFrameToJson(
  SupportMessageFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$SupportMessageFrameTypeEnumMap[instance.type]!,
};

const _$SupportMessageFrameTypeEnumMap = {
  SupportMessageFrameType.SUPPORT_MESSAGE: 'support_message',
};

SupportMessageCreated _$SupportMessageCreatedFromJson(
  Map<String, dynamic> json,
) => $checkedCreate(
  'SupportMessageCreated',
  json,
  ($checkedConvert) {
    final val = SupportMessageCreated(
      author: $checkedConvert(
        'author',
        (v) => $enumDecode(_$SupportAuthorEnumMap, v),
      ),
      createdAt: $checkedConvert(
        'created_at',
        (v) => DateTime.parse(v as String),
      ),
      eventId: $checkedConvert('event_id', (v) => v as String),
      messageId: $checkedConvert('message_id', (v) => v as String),
      number: $checkedConvert('number', (v) => v as String),
      text: $checkedConvert('text', (v) => v as String),
      ticketId: $checkedConvert('ticket_id', (v) => v as String),
      userId: $checkedConvert('user_id', (v) => v as String),
    );
    return val;
  },
  fieldKeyMap: const {
    'createdAt': 'created_at',
    'eventId': 'event_id',
    'messageId': 'message_id',
    'ticketId': 'ticket_id',
    'userId': 'user_id',
  },
);

Map<String, dynamic> _$SupportMessageCreatedToJson(
  SupportMessageCreated instance,
) => <String, dynamic>{
  'author': _$SupportAuthorEnumMap[instance.author]!,
  'created_at': instance.createdAt.toIso8601String(),
  'event_id': instance.eventId,
  'message_id': instance.messageId,
  'number': instance.number,
  'text': instance.text,
  'ticket_id': instance.ticketId,
  'user_id': instance.userId,
};

const _$SupportAuthorEnumMap = {
  SupportAuthor.OPERATOR: 'operator',
  SupportAuthor.USER: 'user',
};

VerificationChangedFrame _$VerificationChangedFrameFromJson(
  Map<String, dynamic> json,
) => $checkedCreate('VerificationChangedFrame', json, ($checkedConvert) {
  final val = VerificationChangedFrame(
    data: $checkedConvert(
      'data',
      (v) => VerificationChanged.fromJson(v as Map<String, dynamic>),
    ),
    ts: $checkedConvert('ts', (v) => DateTime.parse(v as String)),
    type: $checkedConvert(
      'type',
      (v) => $enumDecode(_$VerificationChangedFrameTypeEnumMap, v),
    ),
  );
  return val;
});

Map<String, dynamic> _$VerificationChangedFrameToJson(
  VerificationChangedFrame instance,
) => <String, dynamic>{
  'data': instance.data,
  'ts': instance.ts.toIso8601String(),
  'type': _$VerificationChangedFrameTypeEnumMap[instance.type]!,
};

const _$VerificationChangedFrameTypeEnumMap = {
  VerificationChangedFrameType.VERIFICATION_CHANGED: 'verification_changed',
};

VerificationChanged _$VerificationChangedFromJson(Map<String, dynamic> json) =>
    $checkedCreate('VerificationChanged', json, ($checkedConvert) {
      final val = VerificationChanged(
        status: $checkedConvert(
          'status',
          (v) => $enumDecode(_$VerificationStatusEnumMap, v),
        ),
      );
      return val;
    });

Map<String, dynamic> _$VerificationChangedToJson(
  VerificationChanged instance,
) => <String, dynamic>{'status': _$VerificationStatusEnumMap[instance.status]!};

const _$VerificationStatusEnumMap = {
  VerificationStatus.APPROVED: 'approved',
  VerificationStatus.EXPIRED: 'expired',
  VerificationStatus.PENDING: 'pending',
  VerificationStatus.REJECTED: 'rejected',
};
