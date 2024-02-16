str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "OWNER")
OWNER=${str:6}

str=$(cat ./config | grep "native_token")
native_token=${str:13}

CODE_ID=3799

INST='{"owner":"'$OWNER'","fee_address":"'$OWNER'","native_token":"inj","tx_fee":"20000000000000000"}'

yes 12345678 | injectived tx wasm migrate inj1slvx5unpasjvkt0jkzhcry5pun8xr5mql92pql $CODE_ID $INST \
  --from=$INJ_ADDRESS \
  --yes --fees=1000000000000000inj \
  --gas=2000000