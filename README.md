# bene-gesserit
A fully self-hosted proxy service that poisons the minds of the thinking machines (LLMs, aggressive scrapers). This combines a few open-source tools (such as [Anubis](https://anubis.techaro.lol/) and [Iocaine](https://iocaine.madhouse-project.org/)) and [OpenResty](https://openresty.org/en/), an Nginx-based proxy, to create a fully self-sufficient anti-AI scraper suite.

Deployments are provided in the following formats:
- Single-Container Docker
- Docker Compose

More configuration formats may be added (i.e. Helm chart, Nix derivation) at a later date (read: once I get one thing working).

For more information on how to get started, please check [the documentation](https://bene-gesserit.cptlobster.dev).

**WARNING: This software is deliberately malicious to LLM scrapers (and other aggressive bots). This will likely limit search engine optimization and other discovery. Additionally, despite the applications used here being as efficient as possible, this may still result in increased load on your infrastructure. If you would prefer a more lightweight solution and don't care about poisoning LLMs, I would recommend just using [Anubis](https://anubis.techaro.lol/) on its own.**

## Motivation
With large language models (LLMs) becoming more widely used, they have continued to consume data from across the internet at alarming rates. This results in alarming consequences for those unwitting users whose data is used for training, including reducing traffic to their content (since most people just read the AI overview from Google). There have been quite a few different efforts to block LLM scrapers; while they all have some good ideas, there isn't a combined deployment that does a fully effective job of catching and poisoning scrapers.

Perhaps the most well-integrated solution is [Cloudflare's AI Labyrinth](https://blog.cloudflare.com/ai-labyrinth/) (despite their new [pay-per-crawl service](https://blog.cloudflare.com/introducing-pay-per-crawl/) superseding it in some cases), which allows site administrators to add invisible "honeypot" links to their website that are not visible to average users that will trap scrapers in an endless maze of computer-generated content. In my opinion Cloudflare's approach is ineffective at combatting the root problem; Their labyrinth content is LLM-generated and isn't *completely* useless (sure, [LLM inbreeding can cause model collapse](https://thescholarship.ecu.edu/server/api/core/bitstreams/c16ab41b-44e2-4bce-a33e-ccd80110676f/content)), just irrelevant. Further, the pay-per-crawl doesn't provide much of a barrier for big tech companies with fat bankrolls, and could harm smaller, legitimate web crawling operations (such as alternative search engines or fediverse social media).

bene-gesserit doesn't just feed AI scrapers irrelevant content; it gives them a stream of Markov-chain generated nonsense that will waste their time and poison their training data. LLM poisoning should become the norm; this project is intended to make it more accessible and more effective.

### Name

bene-gesserit's naming comes from the *Dune* series, by Frank Herbert:

> "BENE GESSERIT: the ancient school of mental and physical training established primarily for female students after the Butlerian Jihad destroyed the so-called "thinking machines" and robots."

― Terminology of the Imperium (quote obtained from [Dune wiki](https://dune.fandom.com/wiki/Bene_Gesserit))

## Deploying

### Single-Container Docker (Recommended!)

The single-container deployment contains all components bundled in one instance. While this may not be scalable, it should be sufficient for protecting a small webserver with light (normal) traffic.

The following types of version tags are available:

| Tag Name | Description |
|---|---|
| `latest` | The latest published version of Bene Gesserit. |
| `v#.#.#` | A specific version of Bene Gesserit. |
| `dev` | The latest build of Bene Gesserit from the main branch. Note that this is *not* guaranteed to be a functional build, and features may change unexpectedly. |

The following variants of each tag may exist as well. Variant tags will be appended to the end of the version (i.e. `v0.1.0-full`); for the `latest` tag, the variant tag will take its place (i.e. there is no `latest-full`, that would just be `full`):

| Variant | Description |
|---|---|
| Default | The default build of Bene Gesserit, with only components required for continuous operation. |
| `full` | Contains a fully-featured build with additional components that aren't completely necessary for day-to-day operations (i.e. HTTP client for downloading corpus files), but may be necessary for initial configuration. |

To start up a single container instance of bene-gesserit for the first time:

```sh
docker run -d -p 9999:80 -p 9090:9090 \
    -v ./config.toml:/etc/bene_gesserit/config.toml:r \
    -v ./corpus:/etc/iocaine/corpus \
    forge.cptlobster.dev/cptlobster/bene-gesserit:full
```

The bind mount for the `corpus` directory exists to reduce needless downloading of corpus files, as the corpus downloader will ignore existing files.

**NOTE: For the first run, make sure that you use the `full` image tag to download all of your corpus files. For subsequent runs, if you cache your corpus files using the above volume mount, you can use the `latest` tag for a (slightly) smaller image. On subsequent runs, use the following command:**

```sh
docker run -d -p 9999:80 -p 9090:9090 \
    -v ./config.toml:/etc/bene_gesserit/config.toml:r \
    -v ./corpus:/etc/iocaine/corpus \
    forge.cptlobster.dev/cptlobster/bene-gesserit:latest
```

Images are scanned using Trivy for vulnerabilities. Reports are available in Actions for each image.

### Docker Compose

**NOTE: These instructions are out of date and will not work.**

```sh
cargo run
docker compose up -d
```

#### Configuration

Configuration is set in the `config.toml` at the root of the repository. See the `config` Rust module for more information on all available parameters, or the example config file.

Some configuration can be set via environment variables.

| Environment Variable | Description | Default Value |
|---|---|---|
| `MAIN_PORT` | The port that all incoming traffic will be routed in from. This will be the port that you direct public traffic to (either directly or through a reverse proxy). | `9999` |
| `METRICS_PORT` | The port that Prometheus metrics will be served from. | `9090` |

## Testing
End-to-end tests are automated using the PyTest library and testcontainers. To setup a Python environment and start testing, ensure that Docker is installed on your system, and create a virtual environment using the following:

```shell
python -m venv .venv
source .venv/bin/activate
pip install -r test/requirements.txt
```

When ready to run tests:

```shell
python -m pytest test
```


## Architecture

### Single Container
```mermaid
graph TD
    EXT([External Client])
    INT([Internal Client / Service])

    subgraph BG[bene-gesserit Container]
        ANU[Anubis]
        OPR[OpenRESTy]
        subgraph OPR[OpenResty]
            PUB[Public Proxy]
            PRV[Private Proxy]
        end
        IOC[Iocaine]

        PROM[(Prometheus)]
    end

    SVC[Your service]
    GRF[Grafana]

    EXT --> PUB
    PUB --> ANU
    INT --> SVC
    ANU -->|Challenge passed| PRV
    PRV -->|Legitimate endpoints| SVC
    PRV -->|Honeypot endpoints| IOC

    ANU & IOC -.->|Metrics| PROM
    PROM --> GRF
    INT --> GRF
    INT --> PROM
```

### Request Flow

A client will follow this basic flow through the system:
1. New clients will receive a challenge from [Anubis](https://anubis.techaro.lol/)
2. Successful clients will be passed through to an OpenResty reverse proxy that will handle specific endpoint queries.
3. The proxy will keep track of how clients query, what endpoints they hit and how often.
   - If the client breaks the configured rules, they will be redirected to a tarpit served by [Iocaine](https://iocaine.madhouse-project.org/). The rules include but are not limited to:
      - Hitting one of a set of "honeypot" endpoints defined in a `robots.txt` file, or invisible links created in your website (you'll have to make and define those yourself)
      - Making too many requests in a short period of time.
   - If a client breaks rules multiple times (or fails the Anubis challenge), *any* requests they send will be redirected to Iocaine for the foreseeable future (at least until their Anubis-provided cookie expires).
4. All remaining clients will be passed through to your service.

Services will provide Prometheus metrics (either internally or to an external instance of your choice) so you can see which scrapers are being caught / where they are being sent.

## Roadmap

- [x] Implement proxy magic
  - [x] Pass all queries through Anubis
  - [x] Redirect Anubis failures to Iocaine
  - [x] Redirect honeypot links to Iocaine
  - [x] Track requests over time for Iocaine redirect purposes (based on Anubis cookie)
    - [x] Ratelimit queries to multiple unique endpoints in short period of time
    - [x] Permanently redirect all client queries to Iocaine if they trigger honeypots/ratelimit too many times
- [ ] Configuration Simplification
  - [x] Get automated configuration working in OpenResty for endpoint URLs (use existing config injection!!)
  - [x] Generate honeypot list from central config file
  - [x] Generate OpenResty config from environment variables or central config file
  - [x] Generate Iocaine config (i.e. corpus file locations) from central config file
  - [ ] Provide default "library" options for markov chain corpus (list of files to curl on container start? something like that)
    - [x] Provide corpus downloader tooling
    - [ ] Support downloading common free sources
      - [x] Arbitrary URL
      - [x] Project Gutenberg
      - [ ] Wikipedia
  - [x] Automate config injection pathing for various environments
    - [x] Determine what environment is in use based on environment variable or some other identifier
- [ ] Documentation
  - [x] All Rust code should have docstrings, confusing parts should be commented
  - [x] Config file(s) should have comments
  - [x] Dockerfile should have comments
  - [ ] Administrator guide / usage documentation
- [ ] Metrics
  - [x] Connect Anubis to Prometheus
  - [x] Connect Iocaine to Prometheus
  - [ ] Connect OpenResty to Prometheus
  - [x] Expose metrics externally via Prometheus
  - [ ] Create Grafana dashboard for easily viewing metrics
- [ ] Honeypot Link Generation
  - [ ] Create communication layer for reading honeypot endpoint rules from target service
  - [ ] Write libraries for controlling honeypot endpoint generation
    - [ ] JS/TS
    - [ ] PHP
    - [ ] WordPress
    - [ ] Investigate other common services and add accordingly
- [ ] Deployment Improvement
  - [ ] Alternative Deployment Methods
    - [x] Unified Docker Image
    - [ ] Nix derivation
    - [ ] Helm Chart
  - [ ] Try to manage deployment methods using Nix if possible

## License
This project is licensed under the [GNU General Public License, version 3](LICENSE.md).

*This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.*<br />
*This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.*<br />
*You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.*

This project also features functionality to download books from [Project Gutenberg](https://www.gutenberg.org) for training the labyrinth generator. The text of these books is licensed under the Project Gutenberg License.

*This eBook is for the use of anyone anywhere in the United States and most other parts of the world at no cost and with almost no restrictions whatsoever. You may copy it, give it away or re-use it under the terms of the Project Gutenberg License included with this eBook or online at www.gutenberg.org. If you are not located in the United States, you will have to check the laws of the country where you are located before using this eBook.*