import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:dptp_core_client_sdk/dptp_core_client_sdk_method_channel.dart';

void main() {
  MethodChannelDptpCoreClientSdk platform = MethodChannelDptpCoreClientSdk();
  const MethodChannel channel = MethodChannel('dptp_core_client_sdk');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await platform.getPlatformVersion(), '42');
  });
}
