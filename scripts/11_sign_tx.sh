str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT=${str:17}

yes 12345678 | injectived tx sign unsigned_tx.json \
--chain-id=injective-888 \
--keyring-backend=file \
--from=$INJ_ADDRESS > sign_tx.json