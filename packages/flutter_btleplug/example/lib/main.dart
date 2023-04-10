import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:logger/logger.dart';
import 'package:flutter_btleplug/flutter_btleplug.dart';

final log = Logger();

final streamProvider = StreamProvider.autoDispose<String>((ref) async* {
  await for (final s in btl.createLogStream()) {
    log.i(s.msg);
  }
});

class ScanNotifier extends StateNotifier<List<String>> {
  ScanNotifier() : super([]);

  void add(String s) {
    state = [...state, s];
  }

  void start() {
    final scan = btl.bleScan(filter: []);
    scan.listen((s) {
      log.i(s.id);
      add(s.id);
    });
  }
}

final scanProvider =
    StateNotifierProvider<ScanNotifier, List<String>>((ref) => ScanNotifier());

class Log extends ConsumerWidget {
  const Log({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    AsyncValue<String> message = ref.watch(streamProvider);

    return message.when(
      loading: () => const CircularProgressIndicator(),
      error: (err, stack) => Text('Error: $err'),
      data: (message) {
        return Text(message);
      },
    );
  }
}

void main() async {
  final init = btl.init();
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends ConsumerWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final scans = ref.watch(scanProvider);
    return MaterialApp(
      home: Scaffold(
          appBar: AppBar(
            title: const Text('Plugin example app'),
          ),
          body: Column(children: [
            ElevatedButton(
                onPressed: () {
                  log.i('scanning...');
                  ref.read(scanProvider.notifier).start();
                },
                child: const Text('scan')),
            const Log(),
            ListView(
              shrinkWrap: true,
              children: [for (final s in scans) Text(s)],
            ),
          ])),
    );
  }
}