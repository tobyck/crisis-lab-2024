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

		relay-deps = with pkgs; [
			pkg-config
			cmake
			openssl
		];
	in {
		packages = {
			relay = naersk'.buildPackage {
				src = ./backend/relay;
				nativeBuildInputs = relay-deps;

				RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
			};
		};

		devShells = {
			# this is just for using on the server
			default = pkgs.mkShell {
				buildInputs = with pkgs; [
					bun
					mosquitto
				];
			};

			relay = pkgs.mkShell {
				buildInputs = relay-deps ++ (with pkgs; [
					cargo-flamegraph
					gnuplot
				]);
			};
		};
	});
}
