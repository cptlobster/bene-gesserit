Docker Compose Deployment
=========================

A sample Docker Compose file is provided, and can be integrated into an existing
Docker Compose deployment (or run on its own).

Configuration
-------------
Please make sure that you have the following set in your configuration file:

.. code-block:: toml

   environment = "compose"

Running
-------

First, make sure you have compiled the generator application (preferably with
the http feature enabled for your first run):

.. code-block:: shell

   cargo build --features http .

Then, run the generator:

.. code-block:: shell

   ./target/debug/generator

All the config files should be placed in the ``./docker-include`` directory.
You should now be able to start up the stack using:

.. code-block:: shell

   docker compose up -d

Metrics
-------
Prometheus metrics will be accessible on port 9090.
