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

  cargoHash = "sha256-gEgSb2DkiGBVmSMt3whVLbCEOrPQKhtWvIhnbUDBYOk=";

  meta = {
    description = "Control neovim instances using the command line";
    homepage = "https://github.com/Samasaur1/nvim-ctrl";
    mainProgram = "nvim-ctrl";
  };
}
