import 'dart:convert';
import 'dart:io';

import 'package:goway_events/goway_events.dart';
import 'package:test/test.dart';

final examples = Directory('../../examples');

/// The app talks to goway-realtime only, so this package carries the socket frames; NATS events stay backend-side.
const frameUnions = {'ClientFrame', 'ServerFrame'};

/// Invalid examples the Dart classes accept anyway: they do not check uuid or date formats or number ranges.
/// Safe only because the server checks every frame against the schema before sending; keep this list short.
const knownGaps = <String, String>{};

Object? parse(String title, Map<String, dynamic> json) => switch (title) {
      'ClientFrame' => ClientFrame.fromJson(json).toJson(),
      'ServerFrame' => ServerFrame.fromJson(json).toJson(),
      _ => throw StateError('no Dart type mapped for $title; add it to parse()'),
    };

Iterable<(String, File)> examplesOf(String kind) sync* {
  final folders = Directory('${examples.path}/$kind').listSync().whereType<Directory>().toList()
    ..sort((a, b) => a.path.compareTo(b.path));
  for (final folder in folders) {
    final title = folder.uri.pathSegments.where((part) => part.isNotEmpty).last;
    if (!frameUnions.contains(title)) continue;
    final files = folder.listSync().whereType<File>().toList()..sort((a, b) => a.path.compareTo(b.path));
    for (final file in files) {
      yield (title, file);
    }
  }
}

String relative(File file) => file.path.substring(examples.path.length + 1);

Map<String, dynamic> read(File file) => jsonDecode(file.readAsStringSync()) as Map<String, dynamic>;

void main() {
  for (final (title, file) in examplesOf('valid')) {
    test('${relative(file)} reads and writes back unchanged', () {
      final original = read(file);
      final written = jsonDecode(jsonEncode(parse(title, original)));
      expect(written, equals(original));
    });
  }

  for (final (title, file) in examplesOf('invalid')) {
    final gap = knownGaps[relative(file)];
    test('${relative(file)} is refused', () {
      expect(() => parse(title, read(file)), throwsA(anything));
    }, skip: gap);
  }

  test('a frame union names the frame it picked', () {
    final frame = ServerFrame.fromJson(read(File('${examples.path}/valid/ServerFrame/offer_result.won.json')));
    expect(frame, isA<OfferResultMessage>());
    final result = (frame as OfferResultMessage).frame.data;
    expect(result.outcome, OfferOutcome.WON);
    expect(result.bookingId, isNotNull);
  });

  test('an app reads a frame with a field it does not know yet', () {
    final json = read(File('${examples.path}/valid/ServerFrame/offer_result.lost.json'));
    (json['data'] as Map<String, dynamic>)['reason'] = 'someone was faster';
    expect(ServerFrame.fromJson(json), isA<OfferResultMessage>());
  });
}
