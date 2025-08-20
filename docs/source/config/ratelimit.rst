Custom Ratelimiting Rules
=========================
Bene Gesserit offers flexible ratelimiting tools.

A ratelimiting rule will look like this in your ``config.toml``:

.. code-block::toml

   [[ratelimit.rules]]
   rule = "any"
   amount = 30
   seconds = 5
   exclude = [
       "*.js"
   ]

Common Rule Parameters
----------------------

- ``seconds``: The amount of time before now to consider requests from.
- ``include``: If a path matches any patterns in this array, it will be
  considered for ratelimiting. If unset, all files will be included.
- ``exclude``: If a path matches any patterns in this array, it will `not`
  be considered for ratelimiting. This overrides any ``include`` parameters that
  may have been matched, so you can put in more specific exclude rules.

Rule types
----------

Any request
***********

Triggers a ratelimit if a user makes more than ``amount`` requests in
``seconds`` seconds.

Unique requests
***************
Triggers a ratelimit if a user makes requests against more than ``unique``
unique endpoints in ``seconds`` seconds. If ``total`` is set, then a user must
make that many requests overall, `and` also hit the threshold for unique
endpoints. If you set ``total`` to a lower value than ``unique``, then it will
act the same as if you did not set ``total`` at all.