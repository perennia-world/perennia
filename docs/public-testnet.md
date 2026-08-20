# OWP public testnet plan

The first public network should be intentionally small and disposable.

## Phase A - CI simulation

GitHub Actions runs protocol tests and the deterministic simulator on every push and pull request.

Success condition:
- all tests pass
- simulator exits successfully
- independent state instances produce the same final world hash

## Phase B - three public nodes

Deploy three independently addressable nodes in different failure domains after world-node/libp2p is implemented.

Recommended topology:
- bootstrap-1: provider A / region A
- validator-1: provider B / region B
- validator-2: provider C / region C

No node should possess a special database containing authoritative world state.

## Phase C - community node

Publish reproducible node deployment instructions and allow an external operator to join without receiving a secret database dump or privileged application credential.

## Security rules

- Never embed provider credentials in the repository.
- Bootstrap nodes are discovery infrastructure, not authority.
- Validators use separate protocol keys from cloud/API credentials.
- Testnet keys must never be reused for a later production network.
- Assume all public node IPs will be scanned and attacked.
