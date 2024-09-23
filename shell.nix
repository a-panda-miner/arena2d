let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
	overlays = [ (import rustOverlay) ];
  };
	rust = pkgs.rust-bin.stable.latest.default.override {
  	extensions = [ "llvm-tools-preview" ];
		targets = [ "wasm32-unknown-unknown" ];
	};
in
  pkgs.mkShell {
    packages = [ 
		pkgs.pkg-config
		pkgs.alsa-lib
		pkgs.libudev-zero
		pkgs.xorg.libX11
		pkgs.xorg.libXrandr
		pkgs.xorg.libXcursor
		pkgs.xorg.libXi
		pkgs.libxkbcommon
	];
    nativeBuildInputs = [ pkgs.pkg-config ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
		pkgs.pkg-config
		pkgs.libudev-zero
		pkgs.alsa-lib
		pkgs.xorg.libX11
		pkgs.xorg.libXrandr
		pkgs.xorg.libXcursor
		pkgs.xorg.libXi
		pkgs.libxkbcommon
        pkgs.libglvnd	
	];
	buildInputs = with pkgs; [
      rust
      rust-analyzer
			cargo-pgo
	];
	RUST_BACKTRACE = 1;
}
