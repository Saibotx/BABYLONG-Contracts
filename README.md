# How to deploy these contracts and have them work:
cd contract/cw20-bonding

# prereqs
## Wallet setup 
https://docs.satlayer.xyz/~/changes/z0grj9ZdLyT0QOOYHm1R/bvs-developers/developer-toolbox/wallet-guide

## Babylon CLI
https://docs.satlayer.xyz/~/changes/z0grj9ZdLyT0QOOYHm1R/bvs-developers/developer-toolbox/babylon-cli

### Build wasm for the factory token
cargo clean
cargo build
docker run --rm \
    -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.16.0

### deploy wasm for  token
babylond tx wasm store artifacts/cw20_bonding.wasm --from=deployer --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id=sat-bbn-testnet1 -b=sync --yes --log_format=json --node https://rpc.sat-bbn-testnet1.satlayer.net

### check the tx on block explorer and find the code id --> Towards the end of the output json just before "gas used", there's and attribute key. Under this attribute key is 3 subobjects. in here ther will be a code_id

### code id is 76 for deployed token (currently for mine)

### now build wasm for the factory.
cd contract/token-factory

cargo clean
cargo build

docker run --rm --platform linux/amd64 \
  -v "$(pwd)":/code \
  -v "$(pwd)/../cw20-bonding:/cw20-bonding" \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0


### deploy wasm for factory
babylond tx wasm store artifacts/token_factory.wasm --from=deployer --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id=sat-bbn-testnet1 -b=sync --yes --log_format=json --node https://rpc.sat-bbn-testnet1.satlayer.net

#### check the tx on block explorer and find the code id --> Towards the end of the output json just before "gas used", there's and attribute key. Under this attribute key is 3 subobjects. in here ther will be a code_id

### instanciate factory contract (our factory code id is 78 based on latest deploy and recall our token code ID was 76)
babylond tx wasm instantiate 78 \
  '{"stable_denom": "ubbn", "token_contract_code_id": 76}' \
  --from=deployer \
  --admin=deployer \
  --label "Token Factory" \
  --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 \
  --chain-id=sat-bbn-testnet1 -b=sync --yes \
  --log_format=json --node https://rpc.sat-bbn-testnet1.satlayer.net

Notes: 

Bonding Curve tokens deployed here:
https://devnet.satlayer.xyz/satlayer-babylon-testnet/tx/6823E87E9306CBDAB2CE80461B6BC0B75F78E3D00F9265F1D79B4DAB1AF19212


Factory Deployed here: 
https://devnet.satlayer.xyz/satlayer-babylon-testnet/tx/9D573E057A2F33B73E87DDFAACE2FF08F5A1561B19112C36BC1A5EC88C44088D

Factory instanciated here:
https://devnet.satlayer.xyz/satlayer-babylon-testnet/tx/65B6BCD0F4B8D4C0B6EDBF9186237B8CFFEA83E59E76CD598D9F462F47B0B0B8