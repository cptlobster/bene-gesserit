Observability and Metrics
=========================
Bene Gesserit and its component services output Prometheus metrics. These are funneled by an internal Prometheus
instance (to be replaced by `birdcage <https://forge.cptlobster.dev/cptlobster/birdcage>`_ once stable), which you can
then read using another Prometheus instance or something like Grafana.

Metrics are not fully implemented yet. Notably, OpenResty does not output any metrics. This will be fixed in a later
update.

Configuring Metrics
-------------------
Metrics can be enabled by setting ``metrics.enabled`` to ``true`` in your configuration file.