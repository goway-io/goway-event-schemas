// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'models.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GoOnlineFrame _$GoOnlineFrameFromJson(Map<String, dynamic> json) =>
    $checkedCreate('GoOnlineFrame', json, ($checkedConvert) {
      final val = GoOnlineFrame(
        data: $checkedConvert(
          'data',
          (v) => GoOnline.fromJson(v as Map<String, dynamic>),
        ),
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

GoOnline _$GoOnlineFromJson(Map<String, dynamic> json) => $checkedCreate(
  'GoOnline',
  json,
  ($checkedConvert) {
    final val = GoOnline(
      bidirectional: $checkedConvert('bidirectional', (v) => v as bool?),
      durationHours: $checkedConvert(
        'duration_hours',
        (v) => (v as num?)?.toInt(),
      ),
      fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
      toRegionId: $checkedConvert('to_region_id', (v) => v as String),
      vehicleId: $checkedConvert('vehicle_id', (v) => v as String),
    );
    return val;
  },
  fieldKeyMap: const {
    'durationHours': 'duration_hours',
    'fromRegionId': 'from_region_id',
    'toRegionId': 'to_region_id',
    'vehicleId': 'vehicle_id',
  },
);

Map<String, dynamic> _$GoOnlineToJson(GoOnline instance) => <String, dynamic>{
  'bidirectional': ?instance.bidirectional,
  'duration_hours': ?instance.durationHours,
  'from_region_id': instance.fromRegionId,
  'to_region_id': instance.toRegionId,
  'vehicle_id': instance.vehicleId,
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

OfferProposed _$OfferProposedFromJson(Map<String, dynamic> json) =>
    $checkedCreate(
      'OfferProposed',
      json,
      ($checkedConvert) {
        final val = OfferProposed(
          amount: $checkedConvert('amount', (v) => (v as num).toInt()),
          currency: $checkedConvert('currency', (v) => v as String),
          deadlineAt: $checkedConvert('deadline_at', (v) => (v as num).toInt()),
          fromRegionId: $checkedConvert('from_region_id', (v) => v as String),
          loadId: $checkedConvert('load_id', (v) => v as String),
          serverTime: $checkedConvert(
            'server_time',
            (v) => DateTime.parse(v as String),
          ),
          toRegionId: $checkedConvert('to_region_id', (v) => v as String),
          truckClass: $checkedConvert('truck_class', (v) => v as String),
          weightKg: $checkedConvert('weight_kg', (v) => (v as num).toInt()),
        );
        return val;
      },
      fieldKeyMap: const {
        'deadlineAt': 'deadline_at',
        'fromRegionId': 'from_region_id',
        'loadId': 'load_id',
        'serverTime': 'server_time',
        'toRegionId': 'to_region_id',
        'truckClass': 'truck_class',
        'weightKg': 'weight_kg',
      },
    );

Map<String, dynamic> _$OfferProposedToJson(OfferProposed instance) =>
    <String, dynamic>{
      'amount': instance.amount,
      'currency': instance.currency,
      'deadline_at': instance.deadlineAt,
      'from_region_id': instance.fromRegionId,
      'load_id': instance.loadId,
      'server_time': instance.serverTime.toIso8601String(),
      'to_region_id': instance.toRegionId,
      'truck_class': instance.truckClass,
      'weight_kg': instance.weightKg,
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
