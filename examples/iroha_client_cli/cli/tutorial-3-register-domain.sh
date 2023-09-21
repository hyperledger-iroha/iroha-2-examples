iroha_client_cli domain register --id "looking_glass"
# returns "..." tx hash

# FIXME: would be good to have at least JSON5 here or some other CLI-friendly format
iroha_client_cli domain list filter '{ Identifiable: { Is: "looking_glass" } }'

# should return: ./tutorial-3-register-domain.output.json
