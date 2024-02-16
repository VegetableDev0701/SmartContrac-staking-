str=$(cat ./config | grep "INJ_ADDRESS")
INJ_ADDRESS=${str:12}

str=$(cat ./config | grep "STAKER1")
STAKER1=${str:8}

str=$(cat ./config | grep "STAKER2")
STAKER2=${str:8}

str=$(cat ./config | grep "CONTRACT_ADDRESS")
CONTRACT_ADDRESS=${str:17}

echo "HARPOONTEST BALANCE"
yes 12345678 | injectived query bank balances $INJ_ADDRESS

#echo "----------------------------------"
#echo "STAKER1 BALANCE"
#yes 12345678 | injectived query bank balances $STAKER1

#echo "----------------------------------"
#echo "STAKER2 BALANCE"
#yes 12345678 | injectived query bank balances $STAKER2

echo "----------------------------------"
echo "CONTRACT BALANCE"
yes 12345678 | injectived query bank balances $CONTRACT_ADDRESS