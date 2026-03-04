Roadmap and Planned Features
============================
The following features are planned for development.

Proxy Protection Features
-------------------------
.. list-table::
    :header-rows: 1
    :stub-columns: 1

    * - Name
      - Description
      - Status
      - Planned Release
    * - IP Banning
      - Add fail2ban-style IP banning. Allow banning a certain client / block of IPs after clients from those IPs make a
        certain number of violations.

        This would ideally also permit for IP-level or block-level ratelimiting.

        2026-03-04: IPs are now recorded alongside client ID in client logs, this is the first step in logging
        offenders.
      - In Progress
      -
    * - ASN / Country Lookup
      - Lookup IP address in a BGP Autonomous System to gather ASN / country information in order to identify specific
        providers. This will allow automatically banning specific providers for repeated violations, as well as for
        reporting abuse.
      - Not Started
      -
    * - Automated robots.txt Generation
      - The automated robots.txt generation feature was removed because of Lua's matching rules. Reimplement this in a
        way that allows for generating specific paths from pattern strings to include in the robots.txt.
      - Not Started
      -
    * - Target-side Libraries
      - Write libraries for target services that allow for automatic honeypot endpoint configuration / invisible link
        generation. The following languages/CMSes should be prioritized but targeting more would be useful:

        - NodeJS
        - Hugo
        - WordPress
      - Not Started
      -

Build, Execution, and Deployment
--------------------------------
.. list-table::
    :header-rows: 1
    :stub-columns: 1

    * - Name
      - Description
      - Status
      - Planned Release
    * - Distributed Deployment Support
      - Add support for synchronizing state between multiple replicas of the service.

        2026-03-04: Redis is a supported backend for Anubis, and OpenResty has a library for this. If Redis has
        sufficient namespacing support, we can use a Redis instance for synchronizing all services.
      - Not Started
      -
    * - Multi-Target Deployment Support
      - Add support for serving multiple target applications behind a single instance. Allows for common configuration
        and more synchronization of data.
      - Not Started
      -
    * - Build Dockers Using Nix
      - Build the Docker images using Nix. This may provide lighter Docker images and other build optimizations.
      - Not Started
      -
    * - Nix Deployment
      - Setup a Nix derivation for deploying the stack.
      - Not Started
      -
    * - Kubernetes Deployment
      - Write a Helm chart for deploying the stack, either as an ingress controller or sidecar.
      - Not Started
      -
    * - Unit Testing
      - Add unit testing to Rust code where applicable.
      - Not Started
      -
    * - E2E Testing
      - PyTest, Testcontainers, and Selenium will be used to orchestrate testing the entire Bene Gesserit application.
        This will be crucial to ensure that the service is fully coordinated and behaves as intended. Ideally this could
        be run in CI, but forge.cptlobster.dev still has issues with Testcontainers I think
      - Not Started
      -

Observability & Metrics
-----------------------
.. list-table::
    :header-rows: 1
    :stub-columns: 1

    * - Name
      - Description
      - Status
      - Planned Release
    * - Prometheus Metrics from Everything
      - Output Prometheus metrics from all services from a common endpoint.
      - In Progress
      -
    * - Migrate to Birdcage
      - Currently Bene Gesserit bundles a Prometheus instance in the Docker image. Even with as much stripped down from
        the build as possible, it still doubles the image size. I have started developing
        `birdcage <https://forge.cptlobster.dev/cptlobster/birdcage>`_ as a lightweight Prometheus aggregator.

        2026-03-04: Birdcage development has started, but not released yet.
      - In Progress
      -
    * - Example Grafana Dashboard
      - Create an example Grafana dashboard for displaying important metrics.
      - Not Started
      -

Stability & Speed
-----------------
.. list-table::
    :header-rows: 1
    :stub-columns: 1

    * - Name
      - Description
      - Status
      - Planned Release
    * - Rewrite It In Rust (or C, or literally anything else)
      - OpenResty custom functionality is written in Lua currently. I despise this language. Rewrite all the Lua code in
        a different language (C? Rust?). This could follow one of two approaches:

        - Write a native library with Lua bindings and import it into OpenResty. This could be done in Rust with the
          `mlua crate <https://github.com/mlua-rs/mlua>`_, where a majority of the business logic could be written in
          Rust and Lua would only be used to scrape data from OpenResty and manage outputs.
        - Write an Nginx module that handles all the custom functionality.
      - Not Started
      -

Documentation
-------------
.. list-table::
    :header-rows: 1
    :stub-columns: 1

    * - Name
      - Description
      - Status
      - Planned Release
    * - Fully Document the Configuration Structure
      - The configuration structure is not fully documented, there should be more detailed information and tutorials in
        this site.
      - In Progress
      -
    * - Deploy Cargo Docs Somewhere
      - Build and deploy the Cargo docs as supplemental material.
      - Not Started
      -
    * - Add a Privacy Policy (and similar legal requirements)
      - To comply with international regulations, a privacy policy page should be made available without needing to
        traverse the proxy. Write a sample policy and allow for users to replace it with their own. Might need to
        consult a lawyer or other legal expert on the exact wording of this policy.
      - Not Started
      -