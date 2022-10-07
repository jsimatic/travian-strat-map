# Travian strat map

A small project to build a map of alliances on the browser game Travian.

It is an on-going project but I don't have much time to add all the features that I would want:

- Show un-accessible squares: oases, mountains...
- Identify villages: vdefs, voffs, treasure chambers, WV...
- Add cropper information to adjust threat level...
- Compute likely off operations (distances) to adjust defense plans

## Dependencies

Fontconfig (for plotters)

- Arch Linux: fontconfig
- Debian-based systems: libfontconfig1-dev
- FreeBSD: fontconfig

## Configuring the tool

The config is stored in a `conf.yaml` file.
See the [Travian forum](https://forum.kingdoms.com/index.php?thread/21440-official-api-for-external-tools/&postID=149814#post149814) for API key generation.

The config format is

```
key: <API PRIVATE KEY>
kingdoms:
  us: <KINGDOM NAME>
  allies:
    - <ALLY1 NAME>
    - <ALLY2 NAME>
    ...
  friends:
  hostiles:
  ennemies:
  neutrals:
```

## Running

```
cargo run
```

This will produce a `map.svg` file with the map.