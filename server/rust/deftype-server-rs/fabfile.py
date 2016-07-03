# -*- coding: utf-8 -*-
from fabric.api import abort, local


def run(mode='debug'):
    if mode not in ['debug', 'release']:
        abort("参数mode只支持debug或者release")

    mflag = '' if mode == 'debug' else '--release'
    local('cargo run {} --bin server'.format(mflag))
