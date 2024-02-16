grpcurl -plaintext localhost:9090 list

# Service we want to inspect
grpcurl \
  localhost:9090 \
  describe cosmos.bank.v1beta1.Query

# execute an RPC call to query the node for information:
grpcurl \
  -plaintext
  -d '{"address":"$MY_VALIDATOR"}' \
  localhost:9090 \
  cosmos.bank.v1beta1.Query/AllBalances

#Query for historical state using grpcurl
grpcurl \
  -plaintext \
  -H "x-cosmos-block-height: 279256" \
  -d '{"address":"$MY_VALIDATOR"}' \
  localhost:9090 \
  cosmos.bank.v1beta1.Query/AllBalances

  - commission:
    commission_rates:
      max_change_rate: "0.100000000000000000"
      max_rate: "0.100000000000000000"
      rate: "0.100000000000000000"
    update_time: "2023-01-29T10:45:36.577627523Z"
  consensus_pubkey:
    '@type': /cosmos.crypto.ed25519.PubKey
    key: /mafCN/bZohid8C+JGWM0Vfu1yQjj/9nZVaflfpORvQ=
  delegator_shares: "1000000000000000000.000000000000000000"
  description:
    details: ""
    identity: ""
    moniker: metaONE
    security_contact: ""
    website: ""
  jailed: true
  min_self_delegation: "500000000000000000"
  operator_address: injvaloper1leajejdkl3zk0yt3tg4tth733lveyyst757s00
  status: BOND_STATUS_UNBONDED
  tokens: "999900000000000000"
  unbonding_height: "7606990"
  unbonding_time: "2023-02-20T12:33:56.494405681Z"
  
200003092616672308483638126472640
2027366000048987700835
  50000000000000000000
    999900000000000000
   1000000000000000000
    500000000000000000