let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cargo-audit
    cargo
    redocly
  ];
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    rustup
    clang
    pkg-config
    makeWrapper
  ];
  buildInputs =
    with pkgs;
    [ openssl ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [ glib-networking ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      # Frameworks (WebKit et al.) ship with the SDK; darwin.apple_sdk.frameworks
      # was removed from nixpkgs. Pin the SDK version explicitly instead.
      apple-sdk_15
    ];
  shellHook = ''
    # Toolchain version and components come from rust-toolchain.toml;
    # `rustup show` provisions them. Don't hardcode a version here.
    rustup show > /dev/null
    export PATH=$HOME/.cargo/bin:$PATH
  '';
}
