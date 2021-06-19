let
  nixpkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/fa0326ce5233f7d592271df52c9d0812bec47b84.tar.gz") {};
in
nixpkgs.mkShell {
  buildInputs = with nixpkgs; [ cargo gcc ];
}
