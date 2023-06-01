#include "include/dptp_core_client_sdk/dptp_core_client_sdk_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "dptp_core_client_sdk_plugin.h"

void DptpCoreClientSdkPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  dptp_core_client_sdk::DptpCoreClientSdkPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
