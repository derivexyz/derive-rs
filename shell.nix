let
  pkgs = import <nixpkgs> { };
  openapi-generator-cli-7_13_0 =
    pkgs.openapi-generator-cli.overrideAttrs (old: rec {
      version = "7.13.0";
      src = pkgs.fetchurl {
        url = "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${version}/openapi-generator-cli-${version}.jar";
        sha256 = "sha256-0G2kaAm2L96cp6ism9OZv7omUWYbF+JMqlMDQtBoH+I=";
      };
      dontUnpack = true;
      dontBuild = true;

      patches = [];
      installPhase = ''
        runHook preInstall
        install -D $src $out/share/java/openapi-generator-cli.jar
        makeWrapper ${pkgs.jre}/bin/java $out/bin/openapi-generator-cli \
          --add-flags "-jar $out/share/java/openapi-generator-cli.jar"
        runHook postInstall
      '';
    });
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cargo
    nodejs_22
    corepack_22
    openapi-generator-cli-7_13_0
    python312
    redocly
    sccache
    mold
    clang
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
    ++ lib.optionals stdenv.hostPlatform.isDarwin [ darwin.apple_sdk.frameworks.WebKit ];
  shellHook = ''
    rustup default 1.93
    rustup component add rust-src
    rustup component add clippy
    export PATH=/home/$(whoami)/.cargo/bin:$PATH
  '';
}
