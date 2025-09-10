# Changelog

## Unreleased

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