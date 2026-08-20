# Project identity and contributor privacy

OpenWorld Protocol is designed to be published under a project identity rather than a personal founder identity.

## Public identity model

- Public-facing name: OpenWorld Protocol (OWP).
- Do not place founder names, personal email addresses, company names, physical locations, or personal social profiles in source files, documentation, package metadata, CI logs, domains, or release artifacts.
- Contributors may use pseudonyms.
- Public commits should use a project or pseudonymous Git identity.
- Use a dedicated GitHub organization/account for public publication.
- Use GitHub-provided noreply addresses where attribution to a GitHub pseudonym is desired without disclosing a personal email.

## Operational separation

Public protocol infrastructure and administrative infrastructure should be separate.

Public:
- source code
- protocol specification
- reproducible builds
- public bootstrap node addresses
- release signatures belonging to the project identity

Private:
- billing accounts
- cloud account owners
- recovery credentials
- legal records
- internal incident contacts
- infrastructure root credentials

## Metadata hygiene

Before public release, inspect:
- Git author and committer metadata
- package manifests
- README and documentation
- CI logs and environment variables
- Docker image labels
- domains and WHOIS records
- crash reports and telemetry
- release signing metadata

Do not rely on rewriting Git history after publication as the primary privacy strategy.

## Limits

Pseudonymity is not guaranteed anonymity. Hosting providers, GitHub, payment processors, registrars, network observers, legal processes, endpoint compromise, and operational mistakes can still correlate identities. The project should never promise contributors perfect anonymity.
