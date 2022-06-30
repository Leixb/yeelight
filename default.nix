{ lib
, rustPlatform
}:
let
  toml = lib.importTOML ./Cargo.toml;
in rustPlatform.buildRustPackage 
{
  pname = toml.package.name;
  version = toml.package.version;

  cargoLock.lockFile = ./Cargo.lock;
  checkType = "lib";
  src = ./.;

  meta = with lib; {
    description = toml.package.description;
    homepage = toml.package.repository;
    license = licenses.mit;
    maintainers = with maintainers; [ leixb ];
  };
}
