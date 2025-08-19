#!/bin/sh
set -e

function info {
    echo -e "[\e[1;36mINFO\e[0m overseer] $@"
}

function warn {
    echo -e "[\e[1;33mWARN\e[0m overseer] $@"
}

function err {
    echo -e "[\e[1;31mERR \e[0m overseer] $@"
}

function fail {
    err $@
    exit 1
}

function check_if_exists {
    CMD="$1"
    if ! command -v $1 >/dev/null 2>&1
    then
        fail "Command $CMD is not present and is required to run."
    fi
}

info "Overseer script initiated."

info "Checking prerequisites..."

# make sure all applications are present in the image before running
check_if_exists "iocaine"
check_if_exists "anubis"
check_if_exists "openresty"
check_if_exists "supervisord"
check_if_exists "generator"

info "Generating configurations..."

generator || fail "Configuration script failed. See above logs for information."

info "Preparing files..."

touch /etc/nginx/bg_conf/clients.json
chmod 666 /etc/nginx/bg_conf/clients.json

info "Starting processes using supervisord..."

supervisord -c /etc/supervisord/supervisord.conf -n || fail "Supervisord failed to start. See above logs for information."