{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.cargo-workspace.url = "github:Maix0/cargo-ws-flake";
  inputs.cargo-semver-checks.url = "github:Maix0/cargo-semver-checks-flake";
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    cargo-workspace,
    cargo-semver-checks,
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
        cargo-sc = cargo-semver-checks.packages.${system}.default;
      in
        mkShell {
          nativeBuildInputs = [
            pkgs.bashInteractive
            pkgs.jdk
          ];
          buildInputs = [
            # Rust
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];

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
            cargo-sc
            #cargo-semver-checks
            cargo-ws
            freetype
            mold
            clang
          ];

          LIB_PATH = lib.makeLibraryPath [
            wayland
            wayland-protocols
            #xorg.libX11
            #xorg.libXcursor
            #xorg.libXrandr
            #xorg.libXi
            libxkbcommon
            glslang # or shaderc
            vulkan-headers
            vulkan-validation-layers
            vulkan-loader
            #libGL
            androidComposition.androidsdk
            fontconfig
            freetype
          ];
          packages = [
            pkg-config
            libxkbcommon
            freetype
            #wayland-utils
            vulkan-headers
            vulkan-loader
            vulkan-validation-layers
            vulkan-tools
            androidComposition.androidsdk
            mold
            fontconfig
            clang
          ];

          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";

          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidComposition.androidsdk}/libexec/android-sdk/build-tools/30.0.3/aapt2";
          shellHook = ''
            export LD_LIBRARY_PATH=$LIB_PATH:$LD_LIBRARY_PATH
          '';

          VULKAN_SDK = "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
        };
    });
}
