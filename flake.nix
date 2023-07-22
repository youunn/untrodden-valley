{
  description = "acm";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = { 
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        rust-toolchain = inputs.fenix.packages.${system}.complete.withComponents [
          "rustc"
          "cargo"
          "clippy"
          "rustfmt"
          "rust-src"
          "rust-analyzer"
        ];

        rust-platform = (pkgs.makeRustPlatform {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        });

        codelldb = (pkgs.writeScriptBin "codelldb" ''
          #!${pkgs.bash}/bin/bash
          ${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/.codelldb-wrapped_ \
          --liblldb ${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/lldb/lib/liblldb.so $@
        '');

        cargo-compete = rust-platform.buildRustPackage {
          pname = "cargo-compete";
          version = "0.10.4";

          src = pkgs.fetchFromGitHub {
            owner = "qryxip";
            repo = "cargo-compete";
            rev = "3036036";
            sha256 = "sha256-2lCEk95UNP6JxDvW4SKVc2iUJ6sJMQhOPsTvTfICBNc=";
          };
          cargoPatches = [
            ./pkg/cargo-compete/cargo-compete.patch
          ];
          cargoLock = {
            lockFile = ./pkg/cargo-compete/Cargo.lock;
            outputHashes = {
              "snowchains_core-0.13.2" = "sha256-UgxJDdlzDz98tjg3KLvnv6hD2D7RpuhtBzHkZfeTujY=";
            };
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            openssl
          ];
          doCheck = false;
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            cargo-compete
            codelldb
            lldb
            taplo
          ];

          shellHook = ''
            PATH="${./.}/bin:$PATH"
          '';
        };
      }
    );
}
