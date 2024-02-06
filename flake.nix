{
  description = "Flake to package nvim-ctrl";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, rust-overlay }: 
    let
    # Systems supported
    allSystems = [
      "x86_64-linux" # 64-bit Intel/AMD Linux
      "aarch64-linux" # 64-bit ARM Linux
      "x86_64-darwin" # 64-bit Intel macOS
      "aarch64-darwin" # 64-bit ARM macOS
    ];

    # Helper to provide system-specific attributes
    forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          # Provides Nixpkgs with a rust-bin attribute for building Rust toolchains
          rust-overlay.overlays.default
          # Uses the rust-bin attribute to select a Rust toolchain
          self.overlays.default
        ];
      };
    });
    in
    {
      overlays.default = final: prev: {
        # The Rust toolchain used for the package build
        rustToolchain = final.rust-bin.stable.latest.default;
      };

      packages = forAllSystems ({ pkgs }: {
        default =
          let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
            in
            rustPlatform.buildRustPackage {
              name = "nvim-ctrl";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            };
      });
    }; 
}
