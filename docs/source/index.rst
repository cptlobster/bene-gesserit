.. bene-gesserit documentation master file, created by
   sphinx-quickstart on Mon Aug 18 11:10:52 2025.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

bene-gesserit
=============

A fully self-hosted proxy service that poisons the minds of the thinking
machines (LLMs, aggressive scrapers). This combines a few open-source tools
(such as `Anubis <https://anubis.techaro.lol/>`_ and
`Iocaine <https://iocaine.madhouse-project.org/>`_) and
`OpenResty <https://openresty.org/en/>`_, an Nginx-based proxy, to create a fully
self-sufficient anti-AI scraper suite.

.. warning:: This software is deliberately malicious to LLM scrapers (and other
   aggressive bots). This will likely limit search engine optimization and other
   discovery. Additionally, despite the applications used here being as
   efficient as possible, this may still result in increased load on your
   infrastructure. If you would prefer a more lightweight solution and don't
   care about poisoning LLMs, I would recommend just using
   `Anubis <https://anubis.techaro.lol/>`_ on its own.

.. toctree::
   :maxdepth: 3
   :caption: Contents

   motivation.rst
   design.rst
   deployment/index.rst
   config/index.rst
   development/index.rst

