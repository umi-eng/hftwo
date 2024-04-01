# `hftwo`

For working with the [HF2 protocol](https://github.com/microsoft/uf2) both as the host and the embedded device. As such this library is `no_std` compatible.

Why the name? `hf2` was already taken and only provides utility for the host component.

Warning: whilst this library is `0.1.x` there will be no SemVer compatibility guarantees. Use at your own risk.

## Getting started

```shell
cargo add hftwo --git https://github.com/umi-eng/hftwo.git
```

## Features

- `defmt-03` enable [defmt](https://github.com/knurling-rs/defmt) `Format` on relevant types.
