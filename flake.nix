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
			rustToolchain = ./backend/relay/rust-toolchain.toml;
			sha256 = "sha256-i5+e77mJXAiGTaeb2zXmL19CaQRSL1oeegRwO9QHnRE=";
		}).rust;

		naersk' = pkgs.callPackage naersk {
			cargo = toolchain;
			rustc = toolchain;
		};

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
			# this is just for using on the server to keep things simple and do everything with nix
			default = pkgs.mkShell {
				buildInputs = with pkgs; [
					bun
					mosquitto
				];
			};

			relay = pkgs.mkShell {
				nativeBuildInputs = relay-deps ++ (with pkgs; [
					toolchain # use the right versions of cargo and rustc from the toolchain file
					cargo-flamegraph
				]);
			};
		};
	});
}
