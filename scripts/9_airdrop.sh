str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT_ADDRESS=${str:17}

str=$(cat ./config | grep "COLLECTION_ADDRESS")
COLLECTION_ADDRESS=${str:19}

export AIRDROP='{"airdrop":{"airdrop_amount":"30000000000000000000"}}'
yes 12345678 | injectived tx wasm execute $CONTRACT_ADDRESS "$AIRDROP" --from=$INJ_ADDRESS \
  --yes --fees=1000000000000000inj --gas=2000000 \
  --output json
