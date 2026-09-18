# Security policy

## Reporting

Report vulnerabilities privately through GitHub Security Advisories on this repository. Do not open a public issue. You will get a first response within 48 hours.

## Scope

In scope: the `furcule` binary, the MCP server (stdio and HTTP transports), the local HTTP server and the viewer it serves, the graph file parser, and the npm, AUR and Homebrew packaging.

Out of scope: the behaviour of third-party agents and language models connected over MCP, and content in graph files you author yourself.

## Design notes

- `furcule serve` binds `127.0.0.1` by default. Exposing it on another interface is a deliberate choice via `FURCULE_BIND`.
- Furcule stores no credentials and calls no network service on its own. It has no API key to leak.
- Graph directories are plain files under the user's control; the parser rejects malformed files rather than guessing.
