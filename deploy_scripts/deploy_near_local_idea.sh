rm -rf neardev/
cp ./ft/src/lib_idea.rs ./ft/src/lib.rs
./build.sh
near dev-deploy --wasmFile res/fungible_token.wasm --helperUrl https://near-contract-helper.onrender.com
source neardev/dev-account.env
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "0", "metadata": { "spec": "ft-1.0.0", "name": "AppTestnet IDEA Token", "symbol": "IDEA", "decimals": 24 }}' --accountId $CONTRACT_NAME

# only for local developer (change for your paths in system) - only for near
cp -r ~/.near-credentials ~/ideas-back

echo "Deployed to local near (ERC20 - IDEA TOKEN) = $CONTRACT_NAME"