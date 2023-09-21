iroha_client_cli account register \
    --id 'white_rabbit@looking_glass' \
    --key 'ed0120E7BF9FD343AF8C112AE4AF12FEE1D21EDD3403D97562758C9AA12074A793E95B'
# tx hash

iroha_client_cli account list filter '{ Identifiable: { Is: "white_rabbit@looking_glass" } }'