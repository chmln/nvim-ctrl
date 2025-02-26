{ lib
, rustPlatform
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
in

rustPlatform.buildRustPackage {
  pname = cargoToml.name;
  version = cargoToml.version;

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-tNCSGL0xelWtSZnSHV0l27vh8iYFeFoiFCAD/BPCNnI=";

  meta = {
    description = "Control neovim instances using the command line";
    homepage = "https://github.com/Samasaur1/nvim-ctrl";
    mainProgram = "nvim-ctrl";
  };
}
