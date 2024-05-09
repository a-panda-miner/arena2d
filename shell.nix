let
  pkgs = import <nixpkgs> {};
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
  }
