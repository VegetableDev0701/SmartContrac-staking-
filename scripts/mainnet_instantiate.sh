CODE_ID=113
INJ_ADDRESS="inj18u7hruq746y55fz39wa8fckele77ljk5sn0l4a"
OWNER="inj1w6vuanz7u0hh4wv5zg7z8zm2fj8xjpp3xw6k5a"

INST='{"owner":"'$OWNER'","fee_address":"'$OWNER'","native_token":"inj"}'
echo $INST

yes 12345678 | injectived tx wasm instantiate $CODE_ID $INST \
--label="Injstaking by AOI" \
--from=$INJ_ADDRESS \
--yes --fees=1000000000000000inj \
--gas=2000000 \
--no-admin 