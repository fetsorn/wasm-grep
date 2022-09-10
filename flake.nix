{
  description = "Flake for my portfolio";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-22.05"; };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      eachSystem = systems: f:
        let
          op = attrs: system:
            let
              ret = f system;
              op = attrs: key:
                let
                  appendSystem = key: system: ret: { ${system} = ret.${key}; };
                in attrs // {
                  ${key} = (attrs.${key} or { })
                    // (appendSystem key system ret);
                };
            in builtins.foldl' op attrs (builtins.attrNames ret);
        in builtins.foldl' op { } systems;
      defaultSystems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in eachSystem defaultSystems (system:
      let
        pkgs = (import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        });
        crateName =
          (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.name;
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
        apps = { default = self.defaultApp.${system}; };
        devShell = (({ pkgs, ... }:
          pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              nodejs
              wasm-pack
              pkg-config
              openssl
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })
            ];
            shellHook = "";
          }) { inherit pkgs; });
      });
}
