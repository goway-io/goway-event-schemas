// Written by hand: no Dart generator emits a discriminated union from JSON Schema. When a frame is added to
// the schemas, add its case here; the example tests fail until it is.

import 'models.dart';

/// Any frame an app sends; `type` tells which.
sealed class ClientFrame {
  const ClientFrame();

  Map<String, dynamic> toJson();

  static ClientFrame fromJson(Map<String, dynamic> json) => switch (json['type']) {
        'go_online' => GoOnlineMessage(GoOnlineFrame.fromJson(json)),
        'offer_response' => OfferResponseMessage(OfferResponseFrame.fromJson(json)),
        final other => throw FormatException('unknown client frame type: $other'),
      };
}

final class GoOnlineMessage extends ClientFrame {
  const GoOnlineMessage(this.frame);
  final GoOnlineFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

final class OfferResponseMessage extends ClientFrame {
  const OfferResponseMessage(this.frame);
  final OfferResponseFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}

/// Any frame goway-realtime sends; `type` tells which.
sealed class ServerFrame {
  const ServerFrame();

  Map<String, dynamic> toJson();

  static ServerFrame fromJson(Map<String, dynamic> json) => switch (json['type']) {
        'offer_proposed' => OfferProposedMessage(OfferProposedFrame.fromJson(json)),
        'offer_result' => OfferResultMessage(OfferResultFrame.fromJson(json)),
        'error' => ErrorMessage(ErrorFrame.fromJson(json)),
        final other => throw FormatException('unknown server frame type: $other'),
      };
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

final class ErrorMessage extends ServerFrame {
  const ErrorMessage(this.frame);
  final ErrorFrame frame;
  @override
  Map<String, dynamic> toJson() => frame.toJson();
}
