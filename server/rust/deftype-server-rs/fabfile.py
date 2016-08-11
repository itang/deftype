# -*- coding: utf-8 -*-
from fabric.api import abort, local


def run(mode='debug'):
    if mode not in ['debug', 'release']:
        abort("参数mode只支持debug或者release")

    mflag = '' if mode == 'debug' else '--release'
    local('cargo run {} --bin server'.format(mflag))


def postgres_pull():
    local('docker pull postgres')


def postgres_start():
    local('docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres')


def postgres_client():
    local('docker run -it --rm --link some-postgres:postgres postgres psql -h postgres -U postgres')
