{
  description = "Neuland App iCal Service";

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
        name = "neuland-app-ical-service";

        rustBuild = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [ openssl pkg-config perl ];
          nativeBuildInputs = with pkgs; [ pkg-config perl ];
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = name;
          tag = "latest";
          copyToRoot = [ pkgs.cacert ];
          config = {
            Entrypoint = [ "${rustBuild}/bin/${name}" ];
            ExposedPorts = { "8000/tcp" = {}; };
            Env = [
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
          };
        };
      in
      {
        defaultPackage = rustBuild;
        packages = {
          default = rustBuild;
          neuland-app-ical-service = rustBuild;
          dockerImage = dockerImage;
        };

        defaultApp = {
          type = "app";
          program = "${rustBuild}/bin/${name}";
        };
      });
}