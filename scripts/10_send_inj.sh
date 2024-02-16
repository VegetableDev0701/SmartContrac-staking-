FROM="inj18u7hruq746y55fz39wa8fckele77ljk5sn0l4a"
TO="inj1w6vuanz7u0hh4wv5zg7z8zm2fj8xjpp3xw6k5a"

yes 12345678 | injectived tx bank send $FROM $TO 500000000000000000inj \
--yes --fees=1000000000000000inj --gas=2000000 \
--generate-only > unsigned_tx.json

yes 12345678 | injectived tx sign unsigned_tx.json \
--chain-id=injective-1 \
--keyring-backend=file \
--from=$FROM > sign_tx.json

yes 12345678 | injectived tx broadcast sign_tx.json \
--from=$FROM \