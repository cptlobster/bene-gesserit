# Changelog

## Unreleased
- [anubis] Support adding custom rules to Anubis
  - [anubis/rules] Add optional rule to deny all requests from
    [Cloudflare `/crawl` jobs](https://developers.cloudflare.com/changelog/post/2026-03-10-br-crawl-endpoint/). This
    rule is disabled by default but can be enabled by setting `anubis.predef_rules.block_cf_bots` to `true`.
- [honeypot] Generate robots.txt at generation time from a user-defined list of endpoints.
- [labyrinth] Add configuration options for slowmode (`limit_rate`, `limit_rate_after`)
- [labyrinth] Error on unsupported `Content-Type`, to be extended for other `Content-Type`s in the future (i.e. plain
  text and Markdown via Pandoc, image and audio generation using other tools)
- [openresty] Fix `X-Real-Ip` header not being set correctly on recursive proxy passthroughs
- [openresty/lua] Log client IP addresses along with Anubis cookie
- [openresty/lua] Add IP banning functionality. Also tracks violations for IP addresses and regions, and bans them once
  they exceed the configured threshold.
  - This will be expanded later to allow for matching ASNs and countries, and to make more flexible (i.e. violations
    over a certain period of time)
  - This is disabled by default, and can be enabled with the `ipban.enabled` config option.
- [openresty/lua] Make detecting the client ID from Anubis more consistent. It will decode the JWT and read the 
  challenge ID if available, and will search for cookies based on the actual expected names rather than with a wildcard.
  After inspecting the Anubis source code, it doesn't appear to rotate cookie names like I initially thought.
  - This makes identifying clients from Anubis more consistent, as before it would sometimes use the JWT and sometimes
    use the UUID from `techaro.lol-anubis-cookie-verification`. This would mean a client would have two separate
    entries.
  - This also eliminates a potential denial-of-service attack vector that can be exploited by naming cookies with the
    same prefix with specially crafted IDs, which with 256 concurrent requests could lock the entire JSON table used for
    user identification.
- [openresty/lua] Use client IP address as a fallback client identifier if the Anubis cookie is missing
- [docker] Improve caching for Docker builds
- [docker] Update dependencies in single image build:
  - [anubis] 1.24.0 -> 1.25.0

## v0.1.7 - 2026-02-22
- [generator] Use MD5 to compute filenames for auto-downloaded corpus files. 
  This should make corpus file names deterministic
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