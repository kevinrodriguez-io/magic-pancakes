# ✨🥞 Magic Pancakes

The magic pancakes cli currently supports one command:

```bash
$ pancakes generate
```

## Installing

```bash
$ cargo build --release
$ cargo install --path ./
```

## Notes

With Magic Pancakes you can generate NFT arte. It uses parallelization in rust and a very LightWeight ThreadPool,
that runs 1 thread per core. It expects several things:

1. Layers should be `.png` files, even the background ones.
2. You must provide a JSON Template in the format of the Metaplex NFT Standard (For Solana NFTs).
3. You must provide a Layers Configuration file.

See examples.
