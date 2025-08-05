#!/bin/sh
function info {
    echo -e "[\e[1;36mINFO\e[0m overseer] $@"
}

function warn {
    echo -e "[\e[1;33mWARN\e[0m overseer] $@"
}

function err {
    echo -e "[\e[1;31mERR \e[0m overseer] $@"
}

info "Overseer program initiated."
info "Generating configurations..."

bene_gesserit

info "Starting processes using supervisord..."

# TODO: start openresty/Anubis/Iocaine
supervisord -c /etc/supervisord/supervisord.conf -n