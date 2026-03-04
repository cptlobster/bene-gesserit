Configuring Underlying Services
===============================

Bene Gesserit is a combination of multiple services; as such, they have
differing syntax and structures for their configuration files. Therefore, Bene
Gesserit provides a configuration generator that provides the important parameters
in one configuration file, and then maps parameters to their respective
application configurations.

The Generator
-------------
The config generator program configures services automatically on launch. It
performs the following tasks in this order:

#. Reads the user-provided configuration file.
#. Copies static files into their respective configuration directories.
#. Generates other configuration files from the template directory and places
   the outputs into their respective configuration directories.
#. Downloads corpus files that are not locally available and places them into
   Iocaine's configuration directory.

Once all these steps are completed, the configuration program exits, and the
Bene Gesserit stack can be fully launched.

Generating Configuration files
------------------------------

Configuration files are either statically inserted into the configuration
directories for each service or are generated using templates. The generator
program uses `Tera <https://keats.github.io/tera/>`_, a Rust based templating
engine with similar syntax to Jinja2/Django templating. If you expect to alter
configuration files using user-provided parameters, then you should insert the
file in the ``templates`` directory; If you do not expect to change it, then
insert it in the ``static`` directory.

Available Context
*****************
The following context variables are made available in the configuration files:

- ``config``: The raw configuration object.
- ``env``: Environment-specific configuration parameters. If the user puts in a
  default environment config, this will supply the actual environment
  parameters.
- ``corpus``: Normalized paths to the corpus files for the expected environment.
  These are how the paths will appear inside of the container (if using Docker)
  and should be used for locating corpus files.
- ``words``: Normalized path to the words file for the expected environment.
  This is how the path will appear inside of the container (if using Docker)
  and should be used for locating the words file.

Example Configuration Snippet
*****************************
The following is an example of an nginx server block using Tera templating:

.. code-block:: django

   {# This is an example of an nginx server configuration block. This is a
   comment block! #}
   server {
       {# This value is provided by the env context, since the result may not
       exist in the config block #}
       listen {{ env.binds.external }};
       server_name  localhost;

       {# This is a conditional! You can setup blocks like this to render 
       content if a specific variable is set or a condition is met #}
       {%- if env.endpoints.use_docker_resolver %}
       resolver 127.0.0.11 valid=30s;
       {%- endif %}

       proxy_set_header X-Real-IP $remote_addr;
       proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;

       location / {
           proxy_pass {{ env.endpoints.anubis }};
           proxy_intercept_errors on;
           error_page 421 @ioc;
       }

       location @ioc {
           proxy_pass {{ env.endpoints.iocaine }};
       }
   }
