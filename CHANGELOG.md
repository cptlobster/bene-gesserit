# Changelog

## v0.1.7 - 2026-02-22
- [openresty/lua] Fix bug where Anubis cookie name rotation breaks service

## v0.1.6 - 2026-02-20
- [openresty/lua] Fix bug where trailing slash is missing from iocaine URLs

## v0.1.5 - 2026-02-20
- [docker] Add version labels for dependent programs to image
- [generator] Fix error caused by missing Anubis parameter
- [anubis] Fix inconsistency in Anubis cookie name
- [labyrinth] Add slowmode option to labyrinth configuration

## v0.1.4 - 2026-01-05
- [docker] Update dependencies in single image build:
  - [anubis] 1.22.0 -> 1.24.0
- [anubis] Update bot policy to use default configuration and enable OpenGraph
  passthrough

## v0.1.3 - 2025-10-15
- [generator] Add ability to use environment variables for configuration.
  Parameters can be set using environment variables with the config key name 
  prefixed by `BG_`.
- [docker] It is no longer necessary to set the `environment` parameter in the
  Docker image; this is configured automatically using the `BG_ENVIRONMENT`
  variable.
- [docker] Update dependencies in single image build:
  - [iocaine] 2.5.0 -> 2.5.1
  - [anubis] 1.21.3 -> 1.22.0
- [compose] Update dependencies in example Compose stack:
  - [iocaine] 2.5.0 -> 2.5.1
- [generator] Update Rust crates to latest versions.
- [docker/prometheus] Minimized extra modules used in Prometheus build.
- [docker] Add Trivy scanning for images built in Actions.

## v0.1.2 - 2025-09-09
- [labyrinth] Add delay for incrementing user violations.
- [openresty/lua] Change client data storage to use a JSON file table; splits
  client data between files based on a "hash" of the client ID.
- [openresty/lua] Combine request filters in ratelimit checks to reduce time
  spent iterating through requests.
- [openresty/lua] Lock files while reading/writing data to avoid race conditions
  caused by multiple OpenResty workers accessing the files at the same time.

## v0.1.1 - 2025-08-19

- [generator] Fixed a bug where honeypot configurations would not be applied due
  to a bad variable reference in configuration templates

## v0.1.0 - 2025-08-19

- Initial versioned release.