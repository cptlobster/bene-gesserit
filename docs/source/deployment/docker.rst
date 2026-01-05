Single Docker Image Deployment
==============================

The single-container deployment contains all components bundled in one instance.
While this may not be scalable, it should be sufficient for protecting a small
webserver with light (normal) traffic.

Image Tags
----------
The following types of version tags are available:

.. list-table:: Image Tags
   :header-rows: 1

   * - Tag Name 
     - Description
   * - ``latest``
     - The latest published version of Bene Gesserit.
   * - ``v#.#.#``
     - A specific version of Bene Gesserit.
   * - ``dev``
     - The latest build of Bene Gesserit from the main branch. Note that this is
       `not` guaranteed to be a functional build, and features may change
       unexpectedly.

The following variants of each tag may exist as well. Variant tags will be
appended to the end of the version (i.e. ``v0.1.0-full``); for the ``latest``
tag, the variant tag will take its place (i.e. there is no ``latest-full``, that
would just be ``full``):

.. list-table:: Image Variants
   :header-rows: 1

   * - Variant
     - Description
   * - Default
     - The default build of Bene Gesserit, with only components required for
       continuous operation.
   * - ``full``
     - Contains a fully-featured build with additional components that aren't
       completely necessary for day-to-day operations (i.e. HTTP client for
       downloading corpus files), but may be necessary for initial
       configuration.

Running
-------

To start up a single container instance of Bene Gesserit for the first time:

.. code-block:: sh

   docker run -d -p 9999:80 -p 9090:9090 \
       -v ./config.toml:/etc/bene_gesserit/config.toml:r \
       -v ./corpus:/etc/iocaine/corpus \
       forge.cptlobster.dev/cptlobster/bene-gesserit:full


The bind mount for the ``corpus`` directory exists to reduce needless
downloading of corpus files, as the corpus downloader will ignore existing
files.

.. note::
    
   For the first run, make sure that you use the ``full`` image tag to download
   all of your corpus files. For subsequent runs, if you cache your corpus files
   using the above volume mount, you can use the ``latest`` tag for a (slightly)
   smaller image that does not include extra required packages for HTTP/S
   connections. On subsequent runs, use the following command:

   .. code-block:: sh

      docker run -d -p 9999:80 -p 9090:9090 \
          -v ./config.toml:/etc/bene_gesserit/config.toml:r \
          -v ./corpus:/etc/iocaine/corpus \
          forge.cptlobster.dev/cptlobster/bene-gesserit:latest

Metrics
-------
Prometheus metrics will be accessible on port 9090.

Using Bene Gesserit As A Base Image
-----------------------------------

If you want to use our image as a base for your own Docker image, note the
following implementation details:

- The current Docker image is based on ``openresty:alpine``.
- Anubis, Iocaine, Prometheus, and the Bene Gesserit programs are statically
  compiled and placed in ``/usr/local/bin``.
- The working directory is ``/etc/bene_gesserit``.

.. note::

   I will likely move the build process to Nix in a future release. This will
   likely mean that future base images will build very differently; therefore,
   it is NOT recommended that you base your images on ``latest`` or ``full``.
   Once this change is made, it will be a major version bump, which means that
   using versioned tags will not cause issues.