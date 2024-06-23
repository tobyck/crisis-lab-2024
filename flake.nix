{
	description = "WHS CRISiSLab Challenge 2024 project";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		naersk.url = "github:nix-community/naersk";
	};

	outputs = { self, nixpkgs, flake-utils, naersk }: flake-utils.lib.eachDefaultSystem (system: let
	  pkgs = import nixpkgs { inherit system; };
		naersk' = pkgs.callPackage naersk {};
	in {
		packages = {
			relay = naersk'.buildPackage {
				src = ./backend/relay;

				nativeBuildInputs = with pkgs; [
					pkg-config
					cmake
					openssl
				];

				RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
			};
		};

		devShells.default = pkgs.mkShell {
			buildInputs = with pkgs; [
				bun
				mosquitto
			];
		};
	});
}
