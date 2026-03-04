Considerations for Distributed Deployments
==========================================
The Bene Gesserit docker image was originally designed for a single container to protect a single website. As such,
deploying the container in a way that produces multiple replicas (i.e. with Kubernetes) will result in unusual behaviors
between containers unless configured properly.

Synchronizing Data Between Replicas
-----------------------------------
In order to facilitate synchronization between all services, Bene Gesserit will support Redis (or compatible
alternatives such as Valkey) as an in-memory key/value store for the following reasons:

- `Anubis has native Redis support <https://anubis.techaro.lol/docs/admin/policies/#storage-backends>`_
- `OpenResty has a package for Redis <https://opm.openresty.org/package/openresty/lua-resty-redis/>`_

This will be implemented in a future update.