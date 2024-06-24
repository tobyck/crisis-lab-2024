{
	description = "WHS CRISiSLab Challenge 2024 project";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

		flake-utils.url = "github:numtide/flake-utils";
		naersk.url = "github:nix-community/naersk";

		nixpkgs-mozilla = {
			url = "github:mozilla/nixpkgs-mozilla";
			flake = false;
		};
	};

	outputs = {
		self,
		nixpkgs,
		flake-utils,
		naersk,
		nixpkgs-mozilla
	}: flake-utils.lib.eachDefaultSystem (system: let
		pkgs = import nixpkgs {
			inherit system;
			overlays = [ (import nixpkgs-mozilla) ];
		};

		toolchain = (pkgs.rustChannelOf {
			channel = "nightly";
			date = "2024-06-13";
			sha256 = "sha256-s5nlYcYG9EuO2HK2BU3PkI928DZBKCTJ4U9bz3RX1t4=";
		}).rust;

		naersk' = pkgs.callPackage naersk {
			cargo = toolchain;
			rustc = toolchain;
		};

		relayNativeBuildInputs = with pkgs; [ pkg-config cmake makeWrapper ];
		relayBuildInputs = [ pkgs.openssl ];
		libsslPath = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
	in {
		packages = {
			relay = naersk'.buildPackage {
				src = ./backend/relay;

				nativeBuildInputs = relayNativeBuildInputs;
				buildInputs = relayBuildInputs;

				postInstall = ''
					wrapProgram "$out/bin/relay" \
						--suffix LD_LIBRARY_PATH : ${libsslPath}
				'';
			};
		};

		devShells = {
			default = pkgs.mkShell {
				buildInputs = with pkgs; [
					bun
					mosquitto
				];
			};

			relay = pkgs.mkShell {
				buildInputs = 
					relayNativeBuildInputs ++ 
					relayBuildInputs ++ 
					[
						toolchain
						pkgs.cargo-flamegraph
					];

				RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
			};
		};
	});
}
