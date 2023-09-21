iroha_client_cli asset register \
  --id 'time#looking_glass' \
  --value-type Quantity \
  --unmintable # mintable once

# iroha_client_cli asset list filter '{ "Identifiable": { "Is": "time#looking_glass" } }'
# FIXME: lol `asset register` creates an asset DEFINITION, but `asset list` doesn't return a definition

iroha_client_cli asset mint \
  --account 'white_rabbit@looking_glass' \
  --asset 'time#looking_glass' \
  --quantity 42
  # FIXME: Fixed-type quantity is not supported
  # --quantity=12.34

iroha_client_cli asset list filter '{ Identifiable: { StartsWith: "time" } }'

iroha_client_cli asset burn \
  --account 'white_rabbit@looking_glass' \
  --asset 'time#looking_glass' \
  --quantity 7

iroha_client_cli asset list filter '{ Identifiable: { StartsWith: "time" } }'
