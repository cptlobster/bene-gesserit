Design and Architecture
=======================
This page roughly outlines the design of Bene Gesserit.

Services
--------
Bene Gesserit uses `Anubis <https://anubis.techaro.lol/>`_ as an initial line of
defense. Anubis uses proof-of-work challenges to prove that clients are real
people, and has a set of pre-defined rules for blocking known scrapers.
Additionally, the creator of Anubis offers a paid companion software called
`Thoth <https://anubis.techaro.lol/docs/admin/thoth>`_ which provides a global
database for better identifying aggressive scrapers.

After passing Anubis, requests go through an
`OpenResty <https://openresty.org/en/>`_ proxy (based on Nginx) which handles
internal traffic. Requests deemed to be valid are sent to your target
webservice, and suspicious requests are sent to the labyrinth.

`Iocaine <https://iocaine.madhouse-project.org/>`_ is used for generating an
endless labyrinth of pages with nonsense text. It uses a Markov chain to create
a page of babble that makes `just` enough sense for a scraper to consume it, but
not enough sense that they gain anything useful from it. Each page provides a
set of links that lead further into the labyrinth, which scrapers will continue
to follow seemingly endlessly.

How A Scraper Is Caught
-----------------------
Bene Gesserit has a few tools to catch scrapers that manage to get past Anubis.

.. note::

   As of this writing, not all features are implemented yet. This note will be
   removed once they are.

- Administrators can define "honeypot" endpoints, which when requested will
  automatically be passed to the labyrinth.
- Highly configurable rate limiting rules allow for limiting requests per
  second, unique requests per second, and requests to specific domain paths
  (i.e. if you have some paths that are expected to be hit repeatedly by a
  JavaScript webapp, you can allow those).
- Violations are logged, and a violation threshold can be defined at which
  point the client will be permanently sent to the labyrinth (at least until
  their Anubis token expires).