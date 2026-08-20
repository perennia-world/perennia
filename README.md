# OpenWorld Protocol (OWP)

Experimental protocol for persistent decentralized game worlds.

## Run

```bash
cargo test --workspace
cargo run -p world-simulator
```

The simulator applies the same 10,000 signed transfers to three independent `WorldState` instances and asserts that all three finish with the same world hash.

## Project identity and privacy

OWP is intended to be published under a project identity rather than personal founder identities. Public source and documentation should avoid personal names, personal emails, company attribution, and unnecessary identifying metadata. See `docs/identity-and-anonymity.md`.

## CI

The repository includes a GitHub Actions workflow that runs formatting checks, workspace tests, and the deterministic simulator on public Ubuntu runners.
