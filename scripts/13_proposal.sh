str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "OWNER")
OWNER=${str:6}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT_ADDRESS=${str:17}

yes 12345678 | injectived tx gov submit-proposal wasm-store artifacts/staking.wasm \
--title "Aliens Staking - Upload contract" \
--description "This proposal aims to create an Aliens Collection NFT Staking contract." \
--instantiate-only-address $INJ_ADDRESS \
--deposit=50000000000000000000inj \
--run-as=$INJ_ADDRESS \
--chain-id=injective-888 \
--broadcast-mode=sync \
--yes --fees=5000000000000000inj --gas=10000000 \
--from=$INJ_ADDRESS  \
--code-source-url "https://github.com/harpoon814/contract/" \
--code-hash "a6ea103e936eafad709beff9e6d07654e92727a52f8a46a29804dbcf3e48028e" \
--builder "cosmwasm/rust-optimizer:0.12.12"