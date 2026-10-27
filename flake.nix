{
  description = "CLI for calculating chemical formula masses";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem(system:
    let 
      pkgs = nixpkgs.legacyPackages.${system};

      rofitab = pkgs.rustPlatform.buildRustPackage {
        pname = "rofitabs";
        version = "0.1.0";

        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      install = pkgs.writeShellApplication {
        name = "install-rofitabs";
        runtimeInputs = [pkgs.coreutils];
        text = ''
          dest=$HOME/.local/bin
          mkdir -p "$dest"
          for bin in ${rofitab}/bin/*; do
            install -m755 "$bin" "$dest/$(basename "$bin")"
          done

          #"$dest/${rofitab}" install
          '';
      };
    in
    {
      packages.default = rofitab;

      apps.install =  {
        type = "app";
        program = "${install}/bin/install-rofitabs";
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = [ rofitab ];
        packages = [ pkgs.rust-analyzer pkgs.clippy ];
      };
    });
}
