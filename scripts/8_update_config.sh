str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT_ADDRESS=${str:17}

str=$(cat ./config | grep "COLLECTION_ADDRESS")
COLLECTION_ADDRESS=${str:19}

UPDATE='{"update_config":{"new_owner":"'$INJ_ADDRESS'","new_fee_address":"'$INJ_ADDRESS'","new_collection_address":"'$COLLECTION_ADDRESS'","new_native_token":"inj","new_duration":86400}}'
echo $UPDATE
yes 12345678 | injectived tx wasm execute $CONTRACT_ADDRESS "$UPDATE" --from=$INJ_ADDRESS \
  --yes --fees=1000000000000000inj --gas=2000000 \
  --output json
