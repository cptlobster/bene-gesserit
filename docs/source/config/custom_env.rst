Configuring a Custom Environment
================================
If you are manually deploying Bene Gesserit in a custom environment, you may
need to manually map which endpoints to bind to; for example if you are using
load balancing for certain parts of the Bene Gesserit stack.

.. warning::

   This method of configuration is very particular about syntax and does not
   much validation. You may run into application failures if you do not set
   parameters properly here. Good luck, friend.

The Environment
---------------

The environment consists of three sections:

- **Binds**: Ports or sockets that services should listen on
- **Endpoints**: Ports or sockets that services should connect to to access
  certain services
- **Targets**: Target directories for configuration files / associated content
  (such as corpus files for Iocaine).

Binds
*****

The following binds must be configured:

- ``external``: The external port/socket that OpenResty should listen on. Is
  either numeric (for a TCP port) or a path (for a UNIX socket).
- ``iocaine``: The location that Iocaine should listen on; is either an IP
  address followed by a port number (for a TCP port) or a path (for a UNIX
  socket).
- ``anubis``: The location that Anubis should listen on; either a number
  prefixed by a semicolon (for a TCP port), or a path (for a UNIX socket).
- ``anubis_type``: The type of listen that Anubis should use (either ``tcp``
  or ``unix``). This will be automatically inferred in a later update.
- ``internal``: The internal only port/socket that OpenResty should listen on.
  Is either numeric (for a TCP port) or a path (for a UNIX socket). It is
  strongly recommended that this port is not exposed to the public internet.
- ``prometheus``: The location that Prometheus should listen on; either a number
  prefixed by a semicolon (for a TCP port) or a path (for a UNIX socket).
- ``metrics``: The configuration for where Prometheus metrics should be served.

  - ``anubis``: The location that Anubis should listen on for Prometheus
    metrics; either a number prefixed by a semicolon (for a TCP port), or a path
    (for a UNIX socket).
  - ``anubis_type``: The type of listen that Anubis should use for Prometheus
    metrics (either ``tcp`` or ``unix``). This will be automatically inferred in
    a later update.
  - ``iocaine``: The location that Iocaine should listen on; is either an IP
    address followed by a port number (for a TCP port) or a path (for a UNIX
    socket).

.. note::

    Prometheus does not seem to support scraping from UNIX sockets (per
    `this GitHub issue <https://github.com/prometheus/prometheus/issues/12024>`_.)
    There may be workarounds for this, but implementing this is not a priority.

Endpoints
*********

For these endpoints, the following syntax must be used:

- For endpoints being accessed by OpenResty:
  - For HTTP: ``http://0.0.0.0:1234``
  - For UNIX sockets: ``http://unix:/run/svc.sock``
- For endpoints being accessed by Anubis:
  - For HTTP: ``http://0.0.0.0:1234``
  - For UNIX sockets: ``unix:/run/svc.sock``

The following endpoints must be configured:

- ``iocaine``: The location OpenResty should use to proxy to Iocaine.
- ``anubis``: The location OpenResty should use to proxy to Anubis.
- ``internal``: The location Anubis should use to pass requests to the internal
  proxy.
- ``metrics``: The configuration for where Prometheus metrics can be found.
  
  - ``anubis``: The location that Prometheus should scrape metrics for Anubis
    from. This will be a hostname or IP address and a port; no protocol needed.
  - ``iocaine``: The location that Prometheus should scrape metrics for Iocaine
    from. This will be a hostname or IP address and a port; no protocol needed.

Targets
*******

These target directories are where the config generator will place files.

Target References
*****************

These directories are where the files must be referenced internally. Note that
this may differ from the target directories; i.e. if using a bind mount in
Docker Compose, the target directory will be where the bind mount is coming from
on your local filesystem, but the target reference will be where it is mounted
inside the container.