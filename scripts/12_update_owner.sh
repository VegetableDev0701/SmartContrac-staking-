str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "OWNER")
OWNER=${str:6}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT_ADDRESS=${str:17}

UPDATE='{"update_owner":{"owner":"'$OWNER'"}}'
echo $UPDATE
yes 12345678 | injectived tx wasm execute $CONTRACT_ADDRESS "$UPDATE" \
  --from=$INJ_ADDRESS \
  --yes --fees=1000000000000000inj --gas=2000000 \
  --output json
