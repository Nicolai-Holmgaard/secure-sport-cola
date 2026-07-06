# secure-sport-cola (SSC)

SSC is a command line interface for [stregsystemet](https://github.com/f-klubben/stregsystemet).

## Installation

### Cargo

You can use Cargo to install SSC by running the following command:

```bash
cargo install --git https://github.com/Nicolai-Holmgaard/secure-sport-cola.git
```

### Nix (flakes)

To install SSC on NixOS via flakes add the following to your flake.nix:

```nix
inputs = {
  nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  ssc = {
    url = "github:Nicolai-Holmgaard/secure-sport-cola";
    inputs.nixpkgs.follows = "nixpkgs";
  };
}
```

Then in your configuration.nix add it to `systemPackages`:

```nix
{ inputs, pkgs, ... }:
{
  environment.systemPackages = [
    inputs.ssc.packages.${pkgs.stdenv.hostPlatform.system}.default;
  ]
}
```

## Usage

To use SSC, you can run the following command in your terminal:

```bash
ssc sc
```

It works the same as the normal stregsystemet, but just in the CLI and without the username.
To buy for another user, you can use the `-u` flag followed by the username. For example:

```bash
ssc -u Nicolai sc
```

To list all products, you can use the `-l` flag. For example:

```bash
ssc -l
```

To show the balance of a user, you can use the `-b` flag followed by the username. For example:

```bash
ssc -b
```

You can also chain the `-u` flag with the `-b` flag to show the balance of a specific user. For example:

```bash
ssc -u Nicolai -b
```

## Config

SSC uses a configuration file to store the username and other information for stregsystemet. The configuration file is located at `~/.config/secure-sport-cola/config.toml`. This will be created automatically when you run SSC for the first time. You can edit this file to change the username and other settings. The configuration file has the following format:

```toml
username = "your_username" # The username for stregsystemet
room_id = 10 # The room id for stregsystemet, default is 10
url = "https://stregsystemet.fklub.dk" # The url for stregsystemet useful for testing with a local instance of stregsystemet
```
