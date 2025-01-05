let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
	overlays = [ (import rustOverlay) ];
  };
	rust = pkgs.rust-bin.stable.latest.default.override {
  	extensions = [ "llvm-tools-preview" "rust-src" "cargo" "rustc"];
		targets = [ "wasm32-unknown-unknown" ];
	};
in
  pkgs.mkShell {
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
		pkg-config
		alsa-lib
		xorg.libX11
		xorg.libXrandr
		xorg.libXcursor
		xorg.libXi
		libudev-zero
		libxkbcommon
    rust
    rust-analyzer
		cargo-pgo
	];
	RUST_BACKTRACE = 1;
	RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  }}/lib/rustlib/src/rust/library";
}
