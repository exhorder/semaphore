import uuid
import semaphore
import pytest


def test_basic_key_functions():
    sk, pk = semaphore.generate_key_pair()
    signature = sk.sign(b"some secret data")
    assert pk.verify(b"some secret data", signature)
    assert not pk.verify(b"some other data", signature)


def test_challenge_response():
    resp = semaphore.create_register_challenge(
        b'{"relay_id":"95dc7c80-6db7-4505-8969-3a0927bfb85d","public_key":"KXxwPvbhadLYTglsiGnQe2lxKLCT4VB2qEDd-OQVLbQ"}',
        "EQXKqDYLei5XhDucMDIR3n1khdcOqGWmUWDYhcnvi-OBkW92qfcAMSjSn8xPYDmkB2kLnNNsaFeBx1VifD3TCw.eyJ0IjoiMjAxOC0wMy0wMVQwOTo0NjowNS41NDA0NzdaIn0",
        max_age=0xFFFFFFFF,
    )
    assert str(resp["public_key"]) == "KXxwPvbhadLYTglsiGnQe2lxKLCT4VB2qEDd-OQVLbQ"
    assert resp["relay_id"] == uuid.UUID("95dc7c80-6db7-4505-8969-3a0927bfb85d")
    assert len(resp["token"]) > 40


def test_challenge_response_validation_errors():
    with pytest.raises(semaphore.UnpackErrorSignatureExpired):
        semaphore.create_register_challenge(
            b'{"relay_id":"95dc7c80-6db7-4505-8969-3a0927bfb85d","public_key":"KXxwPvbhadLYTglsiGnQe2lxKLCT4VB2qEDd-OQVLbQ"}',
            "EQXKqDYLei5XhDucMDIR3n1khdcOqGWmUWDYhcnvi-OBkW92qfcAMSjSn8xPYDmkB2kLnNNsaFeBx1VifD3TCw.eyJ0IjoiMjAxOC0wMy0wMVQwOTo0NjowNS41NDA0NzdaIn0",
            max_age=1,
        )
    with pytest.raises(semaphore.UnpackErrorBadPayload):
        semaphore.create_register_challenge(
            b'{"relay_id":"95dc7c80-6db7-4505-8969-3a0927bfb85d","public_key":"KXxwPvbhadLYTglsiGnQe2lxKLCT4VB2qEDd-OQVLbQ"}glumpat',
            "EQXKqDYLei5XhDucMDIR3n1khdcOqGWmUWDYhcnvi-OBkW92qfcAMSjSn8xPYDmkB2kLnNNsaFeBx1VifD3TCw.eyJ0IjoiMjAxOC0wMy0wMVQwOTo0NjowNS41NDA0NzdaIn0",
            max_age=1,
        )


def test_register_response():
    pk = semaphore.PublicKey.parse("sFTtnMGut3xR_EqP1hSmyfBc6590wDQzHFGWj5nEG18")
    resp = semaphore.validate_register_response(
        pk,
        b'{"relay_id":"2ffe6ba6-3a27-4936-b30f-d6944a4f1216","token":"iiWGyrgBZDOOclHjnQILU6zHL1Mjl-yXUpjHOIaArowhrZ2djSUkzPuH_l7UF6sKYpbKD4C2nZWCBhuULLJE-w"}',
        "uLvKHrTtFohGVMLDxlMZythEXmTJTx8DCT2VwZ_x5Aw0UzTzoastrn2tFy4I8jeTYzS-N8D-PZ_khfVzfFZHBg.eyJ0IjoiMjAxOC0wMy0wMVQwOTo0ODo1OC41ODMzNTRaIn0",
        max_age=0xFFFFFFFF,
    )
    assert (
        resp["token"]
        == "iiWGyrgBZDOOclHjnQILU6zHL1Mjl-yXUpjHOIaArowhrZ2djSUkzPuH_l7UF6sKYpbKD4C2nZWCBhuULLJE-w"
    )
    assert resp["relay_id"] == uuid.UUID("2ffe6ba6-3a27-4936-b30f-d6944a4f1216")
