import 'package:flutter_test/flutter_test.dart';
import 'package:dptp_core_client_sdk/dptp_core_client_sdk.dart';
import 'package:dptp_core_client_sdk/dptp_core_client_sdk_platform_interface.dart';
import 'package:dptp_core_client_sdk/dptp_core_client_sdk_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockDptpCoreClientSdkPlatform
    with MockPlatformInterfaceMixin
    implements DptpCoreClientSdkPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final DptpCoreClientSdkPlatform initialPlatform = DptpCoreClientSdkPlatform.instance;

  test('$MethodChannelDptpCoreClientSdk is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelDptpCoreClientSdk>());
  });

  test('getPlatformVersion', () async {
    DptpCoreClientSdk dptpCoreClientSdkPlugin = DptpCoreClientSdk();
    MockDptpCoreClientSdkPlatform fakePlatform = MockDptpCoreClientSdkPlatform();
    DptpCoreClientSdkPlatform.instance = fakePlatform;

    expect(await dptpCoreClientSdkPlugin.getPlatformVersion(), '42');
  });
}
