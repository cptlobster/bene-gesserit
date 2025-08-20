Configuration
=============

This section details how Bene Gesserit configures the underlying services, and
how you can use the configuration for specific use cases.

The Generator Program
---------------------
Bene Gesserit uses a custom configuration generator application, which takes a
single TOML file and then rewrites its parameters into templated configurations
for other applications. More information on this is available in the
:ref:`Developer's Guide <Configuring Underlying Services>`.

In the :ref:`Docker image <Single Docker Image Deployment>`, this program is
automatically run on launch to configure services. In the
:ref:`Docker Compose Deployment`, you will need to run it yourself.

The ``config.toml``
-------------------
This section details the sections of the configuration file at a high level.

``target``
**********
The ``target`` key is the final destination for all good requests; this is the
web service that you inevitably want real people to access. This is formatted as
a URL that can be used by OpenResty's ``proxy_pass`` directive; so it can be in
one of the following formats:

- For an HTTP or HTTPS service: ``http://127.0.0.1:80``
- For a UNIX socket: ``http://unix:/run/svc.sock``

``environment``
***************
The configuration for the environment that Bene Gesserit is running in. This can
either be a manual configuration, or a default environment configuration
provided by the program. See :ref:`Deployment` for default environment options.

``labyrinth``
*************
Configuration for the Iocaine labyrinth. This includes setting up a corpus for
the Markov chain to train from, a word list for link titles, and a violation
threshold for banishing clients permanently to the labyrinth.

``honeypot``
************
Configuration for "honeypot" endpoints; Ideally you would create hidden links to
these paths on your webservice, or list them in your ``robots.txt`` file.

``metrics``
***********
Configuration for using Prometheus metrics.

.. toctree::
   :maxdepth: 2
   :caption: Additional Pages:

   ratelimit.rst
   custom_env.rst