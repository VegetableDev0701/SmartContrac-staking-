INJ_ADDRESS=inj1gl0uf9nky7l6z280sle7x086pvgfd9e5tx932x
CONTRACT_ADDRESS=inj1slvx5unpasjvkt0jkzhcry5pun8xr5mql92pql

QUERY='{"get_total_locked":{"collection_address":"inj1tnptr6pc9g0npxf06es8ha7yadr8ek6ddhjr3u"}}'
injectived query wasm contract-state smart $CONTRACT_ADDRESS "$QUERY" --output text

#QUERY='{"get_block_time":{}}'
#injectived query wasm contract-state smart $CONTRACT_ADDRESS "$QUERY" --output text

#GET_USER_STAKED_NFTS='{"get_staked_nfts":{"address":"inj1qhjg7s604ywn57gjgpn2s23kl8v6pymr492a64","collection_address":"inj1krph8w9d9ep289yh7pcuk7gd587a693g340vyj"}}'
#injectived query wasm contract-state smart $CONTRACT_ADDRESS "$GET_USER_STAKED_NFTS" --output text


#yes 12345678 | injectived tx wasm execute $CONTRACT_ADDRESS "$GET_USER_LIST" \
#  --from=$INJ_ADDRESS \
#  --yes --fees=1000000000000000inj --gas=2000000 \
#  --output json
