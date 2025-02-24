{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, ... } @ inputs: inputs.flake-utils.lib.eachDefaultSystem (system:
    let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "envide";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      devShells.default = with pkgs; mkShell {
        buildInputs = [
          just
          openssl
          pkg-config
          rust-bin.stable.latest.default
        ];
      };
    }
  );
}

