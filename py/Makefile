venv:
	@$(MAKE) -C .. venv/bin/python
.PHONY: venv

install: venv
	SEMAPHORE_DEBUG=1 ../venv/bin/pip install -v --editable .
.PHONY: install

test: install
	../venv/bin/pip install -U pytest
	@../venv/bin/pytest tests
.PHONY: test

manylinux:
	@$(MAKE) -C .. manylinux
.PHONY: manylinux

wheel: venv
	@../venv/bin/python setup.py bdist_wheel
.PHONY: wheel

sdist: venv
	@../venv/bin/python setup.py sdist --format=zip
.PHONY: sdist

clean:
	rm -rf .eggs build dist semaphore.egg-info MANIFEST rustsrc.zip version.txt semaphore/_lowlevel*
.PHONY: clean
