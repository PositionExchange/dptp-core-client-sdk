rm -R lib/ios/dptp_core_sdk.xcodeproj/

cd rust || exit

# Build rust file
cargo build &&

# Generate bridge between Flutter and Rust
flutter_rust_bridge_codegen \
--rust-input src/api.rs \
--dart-output ../lib/bridge_generated_core.dart \
--llvm-path /opt/homebrew/opt/llvm/ \
--c-output ../lib/ios/bridge_generated_core.h

# Generate iOS binary
cargo lipo --release && cp ../../target/universal/release/libdptp_core_sdk.a ../lib/ios &&
# Generate Android binary
cargo ndk -o ../lib/android/jniLibs build --release &&

cd ../ &&

# Build iOS library
cargo xcode &&
mv rust/dptp_core_sdk.xcodeproj/ lib/ios

wait