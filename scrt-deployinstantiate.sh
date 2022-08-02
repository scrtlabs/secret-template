#!/bin/bash

#       val gas = (contractFile.length() * 6).coerceAtMost(6_000_000)
gas=$(cat contract.wasm.gz | wc -c | python -c "import sys; print(max(6000000, int(sys.stdin.read())*6))")
# gas=777462

secretcli config chain-id pulsar-2
secretcli config node https://rpc.pulsar.scrttestnet.com
secretcli config output json
secretcli config keyring-backend test
secretcli config broadcast-mode block
secretcli keys delete SecretIDE-Deployment -y
cat ~/.Secret-IDE-seed | secretcli keys add SecretIDE-Deployment --recover || exit 1
codeId=$(secretcli tx compute store contract.wasm.gz --from SecretIDE-Deployment --gas $gas -y | jq -r '.logs[0].events[0].attributes[3].value')
codeId=$(printf '%s' $codeId)
echo "Contract stored successfully! Code ID: $codeId"
label=${RANDOM}${RANDOM}${RANDOM}
set -x
secretcli tx compute instantiate $codeId '{"count":55}' --label "${label}" --from 'SecretIDE-Deployment' -y
