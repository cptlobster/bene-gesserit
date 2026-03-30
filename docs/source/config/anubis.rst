Custom Anubis Rules
===================
Since Bene Gesserit uses Anubis as an underlying service, custom bot policies can be configured. Some policies are
pre-configured and can be toggled under the config parameter ``anubis.predef_rules``. Custom rulesets will be added at a
later date.

`The Anubis documentation <https://anubis.techaro.lol/docs/admin/configuration/expressions>`_ provides more detail on
how to write your own rules. Custom rules, when added, will use the same syntax.

Pre-Defined Rules
-----------------
These rules are pre-defined and are togglable on and off by setting the ``anubis.predef_rules.[rule name]`` key to
``true``. In a future release, when rules have additional parameters, you will need to use the
``anubis.predef_rules.[rule name].enabled`` key instead.

.. list-table::
   :header-rows: 1
   :stub-columns: 1

   * - Name
     - Description
     - Additional Parameters
   * - ``block-cf-crawl``
     - Block `Cloudflare's Browser Rendering /crawl endpoint <https://developers.cloudflare.com/changelog/post/2026-03-10-br-crawl-endpoint/>`_,
       which is likely going to be used for AI scraping purposes.
     -
