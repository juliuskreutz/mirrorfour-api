# Mirror Four Api

## Installation

```nix
# flake.nix
{
  inputs = {
    mirrorfour-api = {
      url = "github:juliuskreutz/mirrorfour-api";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  ...
}
```

```nix
# configuration.nix
{
  inputs,
  ...
}:
{
  imports = [
    inputs.mirrorfour-api.nixosModules.mirrorfour-api
  ];

  services.mirrorfour-api = {
    enable = true;
    sessionKey = "YOUR_SESSION_KEY_IN_BASE64";
  };
  ...
}
```

## Developing

You need:

- [direnv](https://direnv.net)
- [devenv](https://devenv.sh)

```sh
cd mirrourfour-api
devenv up
```

And you're ready to go :D
