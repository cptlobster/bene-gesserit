# build Sphinx docs
FROM python:3.13-alpine AS builder

WORKDIR /docs

RUN apk add make

RUN pip install sphinx sphinx-book-theme

# copy Makefile and source directory
COPY . /docs/

RUN make clean html

# serve docs using httpd
FROM httpd:alpine3.22

COPY --from=builder /docs/build/html/ /usr/local/apache2/htdocs/