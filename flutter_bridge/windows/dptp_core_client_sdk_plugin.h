#ifndef FLUTTER_PLUGIN_DPTP_CORE_CLIENT_SDK_PLUGIN_H_
#define FLUTTER_PLUGIN_DPTP_CORE_CLIENT_SDK_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace dptp_core_client_sdk {

class DptpCoreClientSdkPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  DptpCoreClientSdkPlugin();

  virtual ~DptpCoreClientSdkPlugin();

  // Disallow copy and assign.
  DptpCoreClientSdkPlugin(const DptpCoreClientSdkPlugin&) = delete;
  DptpCoreClientSdkPlugin& operator=(const DptpCoreClientSdkPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace dptp_core_client_sdk

#endif  // FLUTTER_PLUGIN_DPTP_CORE_CLIENT_SDK_PLUGIN_H_
