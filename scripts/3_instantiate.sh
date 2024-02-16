str=$(cat ./config | grep "code_id")
CODE_ID=${str:8}

str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "OWNER")
OWNER=${str:6}

str=$(cat ./config | grep "native_token")
native_token=${str:13}

INST='{"owner":"'$OWNER'","fee_address":"'$OWNER'","native_token":"inj","tx_fee":"20000000000000000"}'
echo $INST

yes 12345678 | injectived tx wasm instantiate $CODE_ID $INST \
--label="NFT-Staking" \
--from=$INJ_ADDRESS \
--yes --fees=1000000000000000inj \
--gas=2000000 \
--admin='inj1gl0uf9nky7l6z280sle7x086pvgfd9e5tx932x'