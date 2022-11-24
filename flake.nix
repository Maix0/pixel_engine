{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.cargo-workspace.url = "github:Maix0/cargo-ws-flake";
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    cargo-workspace,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
        config.android_sdk.accept_license = true;
      };
    in {
      devShell = with pkgs; let
        androidComposition = androidenv.composeAndroidPackages {
          platformVersions = ["30"];
          includeNDK = true;
          ndkVersions = ["22.1.7171670"];
          buildToolsVersions = ["30.0.3"];
        };
        cargo-ws = cargo-workspace.defaultPackage.${system}; #.packages.${system}.default;
      in
        mkShell {
          nativeBuildInputs = [
            pkgs.bashInteractive
            pkgs.jdk
          ];
          buildInputs = [
            # Rust
            (rust-bin.stable.latest.default.override {
              targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu" "aarch64-linux-android"];
            })
			cmake
			fontconfig
            pkgconfig
			# Web
            trunk
            wasm-bindgen-cli
            # android
            androidComposition.androidsdk
            cargo-ws
          ];

          LIB_PATH = lib.makeLibraryPath [wayland wayland-protocols libxkbcommon vulkan-loader libGL androidComposition.androidsdk];
          packages = [pkg-config libxkbcommon wayland-utils vulkan-headers vulkan-loader vulkan-validation-layers vulkan-tools androidComposition.androidsdk];

          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";

          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidComposition.androidsdk}/libexec/android-sdk/build-tools/30.0.3/aapt2";
          shellHook = ''
            export LD_LIBRARY_PATH=$LIB_PATH:$LD_LIBRARY_PATH
          '';
        };
    });
}
