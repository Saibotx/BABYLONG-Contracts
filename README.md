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

### instanciate factory contract (our factory code id is 90 based on latest deploy and recall our token code ID was 82)
babylond tx wasm instantiate 90 \
  '{"stable_denom": "ubbn", "token_contract_code_id": 82}' \
  --from=deployer \
  --admin=deployer \
  --label "Token Factory" \
  --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 \
  --chain-id=sat-bbn-testnet1 -b=sync --yes \
  --log_format=json --node https://rpc.sat-bbn-testnet1.satlayer.net


## Create a token:
babylond tx wasm execute bbn1yrsteeuwsajpspshpf09tax3rsnmdnn283dz73pz2qdz6ke0rmeqplcvkx '{"create_bonding_token": {"name": "NewBondToken", "symbol": "NBT"}}'   --from=deployer   --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id=sat-bbn-testnet1 -b=sync --yes   --log_format=json --node https://rpc.sat-bbn-testnet1.satlayer.net

Deployed FACTORY: 
bbn1yrsteeuwsajpspshpf09tax3rsnmdnn283dz73pz2qdz6ke0rmeqplcvkx

Deployed TOKEN (created from this factory): 
bbn1asnx4t0j2espauakdpw459t7f8wujtef2ymtdda9cl88fx8fkumst72y6l


To buy:
```
babylond tx wasm execute bbn1asnx4t0j2espauakdpw459t7f8wujtef2ymtdda9cl88fx8fkumst72y6l \
  '{"buy": {}}' \
  --from=deployer \
  --amount=100ubbn \
  --gas=auto \
  --gas-prices=1ubbn \
  --gas-adjustment=1.3 \
  --chain-id=sat-bbn-testnet1 \
  -b=sync \
  --yes \
  --log_format=json \
  --node https://rpc.sat-bbn-testnet1.satlayer.net
  ```

  To sell:
  ```
    babylond tx wasm execute bbn1asnx4t0j2espauakdpw459t7f8wujtef2ymtdda9cl88fx8fkumst72y6l \
  '{"burn": {"amount": "100"}}' \
  --from=deployer \
  --gas=auto \
  --gas-prices=1ubbn \
  --gas-adjustment=1.3 \
  --chain-id=sat-bbn-testnet1 \
  -b=sync \
  --yes \
  --log_format=json \
  --node https://rpc.sat-bbn-testnet1.satlayer.net
  ```

  To Query price: 
  ```
    babylond query wasm contract-state smart bbn1asnx4t0j2espauakdpw459t7f8wujtef2ymtdda9cl88fx8fkumst72y6l \
  '{"balance": {"address": "bbn1wc57avcll2g4n24fs0r0nwruvt56ggahkfg34u"}}' \
  --output=json \
  --node https://rpc.sat-bbn-testnet1.satlayer.net
  ```