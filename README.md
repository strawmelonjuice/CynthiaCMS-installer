# Cyninst

Cynthia-CMS installation manager.

## Not ready.

Just like CynthiaCMS, not ready for production usage.

## About

Cynthia is written in Node, which has `NPM` as default package manager, however, installing Cynthia through `NPM` will not work, because `NPM` uses a structure that is simply incompatible with how Cynthia works. `cyninst` aims to use `NPM`'s ways where possible, but also structure Cynthia folders to be a perfect fit for the Cynthia instance living in them.

## Usage (for now)

```bash
cyninst <[version]>
```

And, for installing [Cynthia plugins](https://github.com/strawmelonjuice/CynthiaCMS-installer/blob/main/plugins.md):

```bash
cyninst -p <plugin_name> <[plugin_version]>
```
