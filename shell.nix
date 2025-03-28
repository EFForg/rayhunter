with import <nixpkgs>
{
  crossSystem = {
    config = "armv7l-unknown-linux-gnueabihf";
    arch = "arm";
    bigEndian = false;
    libc = "glibc";
  };
};
  mkShell {
    buildInputs = [
      glibc.static
      stdenv
      gcc
    ];

    inputsFrom = [glibc cargo];
  }
