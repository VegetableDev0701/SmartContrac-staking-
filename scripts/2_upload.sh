str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

yes 12345678 | injectived tx wasm store artifacts/staking.wasm \
--from=$INJ_ADDRESS \
--yes --gas-prices=500000000inj --gas=10000000