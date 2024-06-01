{
	description = "WHS CRISiSLab Challenge 2024 project";

	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.flake-utils.url = "github:numtide/flake-utils";

	outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
	  pkgs = import nixpkgs { inherit system; };
	in {
		devShells = {
			relay = pkgs.mkShell {
				nativeBuildInputs = with pkgs; [
					pkg-config
					cmake
					openssl
				];
			};
		};
	});
}
