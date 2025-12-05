{
  description = "aocoracle development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mkhelp.url = "github:apljungquist/mkhelp-rs/nix";
  };
  outputs =
    {
      self,
      mkhelp,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        config.allowUnfree = true;
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      devShells.${system}.default = pkgs.mkShellNoCC {
        buildInputs = with pkgs; [
          python310
        ];

        nativeBuildInputs = with pkgs; [
          rust
        ];

        packages = with pkgs; [
          git
          git-lfs
          mkhelp.packages.${system}.default
          nixfmt-rfc-style
        ];

        shellHook = ''
          # Prevent cargo from finding programs in the default cargo home by **appending** it to the path because
          # otherwise cargo will **prepend** it to the path e.g. when looking for clippy.
          export PATH="$PATH:$HOME/.cargo/bin"

          if [ ! -d venv ]; then
            echo "Creating venv"
            python -m venv --prompt $(basename $(pwd)) venv
            source venv/bin/activate
            PIP_CONSTRAINT="$(pwd)/constraints.txt" pip install pip setuptools
          else
            echo "Reusing venv"
            source venv/bin/activate
          fi

          export PATH="$(pwd)/bin:$PATH"
        '';
      };
    };
}
