import socket
import subprocess

import pytest

from .fixtures.gobetween import gobetween  # noqa
from .fixtures.haproxy import haproxy  # noqa
from .fixtures.mini_sentry import mini_sentry  # noqa
from .fixtures.relay import relay  # noqa


@pytest.fixture
def random_port():
    def inner():
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind(("127.0.0.1", 0))
        s.listen(1)
        port = s.getsockname()[1]
        s.close()
        return port

    return inner


@pytest.fixture
def background_process(request):
    def inner(*args, **kwargs):
        p = subprocess.Popen(*args, **kwargs)
        request.addfinalizer(p.kill)
        return p

    return inner


@pytest.fixture
def config_dir(tmpdir):
    counters = {}

    def inner(name):
        counters.setdefault(name, 0)
        counters[name] += 1
        return tmpdir.mkdir("{}-{}".format(name, counters[name]))

    return inner


@pytest.fixture(  # noqa
    params=[
        lambda s, r, g, h: r(s),
        lambda s, r, g, h: r(r(s)),
        lambda s, r, g, h: r(h(r(g(s)))),
        lambda s, r, g, h: r(g(r(h(s)))),
    ]
)
def relay_chain(request, mini_sentry, relay, gobetween, haproxy):
    return lambda: request.param(mini_sentry, relay, gobetween, haproxy)
