{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    cargo
    yarn
    go
    ruby
    elixir
  ];
}
