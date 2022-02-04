{
  description = "Flake for my portfolio";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        });
        crateName = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.name;
        getBin = package: "${package}/bin/${crateName}";
        defaultPackage =
          (pkgs.callPackage ./package.nix { inherit crateName; });
      in {
        inherit defaultPackage;
        packages = { default = defaultPackage; };
        defaultApp = {
          type = "app";
          program = "${defaultPackage}";
        };
        apps = {
          default = self.defaultApp.${system};
        };
        devShell = (({ pkgs, ... }:
          pkgs.mkShell {
            buildInputs = with pkgs; [ cargo
	    nodejs
            wasm-pack
            pkg-config
            openssl
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            }) ];
            shellHook = "";
          }) { inherit pkgs; });
      });

}
