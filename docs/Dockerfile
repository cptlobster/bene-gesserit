# build Sphinx docs
FROM python:3.13-alpine AS builder

WORKDIR /docs

RUN pip install sphinx sphinx-book-theme

# copy Makefile and source directory
COPY Makefile source/ /docs/

RUN make clean html

# serve docs using httpd
FROM httpd:alpine3.22

COPY --from=builder /docs/build/html/ /usr/local/apache2/htdocs/