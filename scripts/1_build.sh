docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=regi.stry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.12