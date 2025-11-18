{
  description = "Behavior Tree Engine";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "clippy"
            "rustfmt"
          ];
        };
      in
      {
        devShell = pkgs.mkShell {
          name = "North Slope Behaviors";

          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            vscode-extensions.vadimcn.vscode-lldb
            nixfmt-rfc-style
            codex
          ];

          shellHook = ''
            echo "Welcome to the btree dev shell"
          '';
        };
      }
    );
}
