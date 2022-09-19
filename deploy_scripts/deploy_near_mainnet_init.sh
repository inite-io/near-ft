rm -rf neardev/
cp ./ft/src/lib_init.rs ./ft/src/lib.rs
./build.sh
near deploy --wasmFile res/fungible_token.wasm --helperUrl https://near-contract-helper.onrender.com --accountId $CONTRACT_NAME
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "1000000000", "metadata": { "spec": "ft-1.0.0", "name": "INIT Token", "symbol": "INIT", "decimals": 24 }}' --accountId $CONTRACT_NAME

echo "Deployed to near (INIT TOKEN) = $CONTRACT_NAME"
