# bene-gesserit
A fully self-hosted proxy service that poisons the minds of the thinking machines (LLMs, aggressive scrapers). This combines a few open-source tools (such as [Anubis](https://anubis.techaro.lol/) and [Iocaine](https://iocaine.madhouse-project.org/)) and [OpenResty](https://openresty.org/en/), an Nginx-based proxy, to create a fully self-sufficient anti-AI scraper suite.

Configurations are provided in the following formats:
- Docker Compose

More configuration formats may be added (i.e. Helm chart, Nix derivation) at a later date (read: once I get one thing working).

## Deploying

### Docker Compose

```sh
docker compose up -d
```

#### Configuration

| Environment Variable | Description | Default Value |
|---|---|---|
| `TARGET_URL` | The final service endpoint you intend to access. This is a local IP address within your Compose network, or an external address (preferably one that isn't exposed to the greater internet!). | `http://example:8080` |
| `MAIN_PORT` | The port that all incoming traffic will be routed in from. This will be the port that you direct public traffic to (either directly or through a reverse proxy). | `9999` |
| `METRICS_PORT` | The port that Prometheus metrics will be served from. | `9090` |

## Architecture

```mermaid
graph TD
    EXT([External Client])
    INT([Internal Client / Service])

    subgraph AS[antiscraper collection]
        ANU[Anubis]
        OPR[OpenRESTy]
        subgraph OPR[OpenRESTy]
            PUB[Public Proxy]
            PRV[Private Proxy]
        end
        IOC[Iocaine]

        PROM[(Prometheus)]
        GRF[Grafana]
    end

    SVC[Your service]

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

A client will follow this basic flow through the system:
1. New clients will receive a challenge from [Anubis](https://anubis.techaro.lol/)
2. Successful clients will be passed through to an OpenResty reverse proxy that will handle specific endpoint queries.
3. The proxy will keep track of how clients query, what endpoints they hit and how often.
   - If the client breaks the configured rules, they will be redirected to a tarpit served by [Iocaine](https://iocaine.madhouse-project.org/). The rules include but are not limited to:
      - Hitting one of a set of "honeypot" endpoints defined in a `robots.txt` file, or invisible links created in your website (you'll have to make and define those yourself)
      - Making too many requests in a short period of time.
   - If a client breaks rules multiple times (or fails the Anubis challenge), *any* requests they send will be redirected to Iocaine for the foreseeable future (at least until their Anubis-provided cookie expires).
4. All remaining clients will be passed through to your service.

Services will provide Prometheus metrics (either internally or to an external instance of your choice) so you can see which scrapers are being caught / where they are being sent. You can also include an internal Grafana dashboard for viewing metrics, or use an existing Grafana instance.

## License
This project is licensed under the [GNU General Public License, version 3](LICENSE.md).

*This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.*<br />
*This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.*<br />
*You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.*