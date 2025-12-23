{
  pkgs ? import <nixpkgs> { },
}:
let
  libs = with pkgs; [
    libGL
    libxkbcommon
    wayland
  ];
in
pkgs.mkShell {
  name = "dropfunk";
  buildInputs =
    with pkgs;
    [
      rustc
      rustfmt
      clippy
      cargo
      rust-analyzer
      pre-commit
    ]
    ++ libs;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
  RUST_BACKTRACE = 1;
}
