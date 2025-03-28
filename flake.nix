{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        name = "neuland.app-ical-service";

        binName = "neuland_app_ical_service";

        rustBuild = naersk-lib.buildPackage {
          src = self;
          buildInputs = with pkgs; [ cargo rustc ];
        };
        dockerImage = pkgs.dockerTools.buildImage
          {
            name = name;
            tag = rustBuild.version;
            copyToRoot = [ pkgs.cacert ];
            config = {
            Entrypoint = [ "${rustBuild}/bin/${binName}" ];  // Use binName instead of name
            };
          };
      in
      {
        defaultPackage = rustBuild;
        packages.image = dockerImage;

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}