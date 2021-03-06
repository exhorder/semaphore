import pytest
import semaphore

REMARKS = [["myrule", "s", 7, 17]]
META = {"": {"rem": REMARKS}}
TEXT = "Hello, [redacted]!"
CHUNKS = [
    {"type": "text", "text": "Hello, "},
    {"type": "redaction", "text": "[redacted]", "rule_id": "myrule", "remark": "s"},
    {"type": "text", "text": "!"},
]
META_WITH_CHUNKS = {"": {"rem": REMARKS, "chunks": CHUNKS}}


def test_split_chunks():
    chunks = semaphore.split_chunks(TEXT, REMARKS)
    assert chunks == CHUNKS


def test_meta_with_chunks():
    meta = semaphore.meta_with_chunks(TEXT, META)
    assert meta == META_WITH_CHUNKS


def test_meta_with_chunks_none():
    meta = semaphore.meta_with_chunks(TEXT, None)
    assert meta is None


def test_meta_with_chunks_empty():
    meta = semaphore.meta_with_chunks(TEXT, {})
    assert meta == {}


def test_meta_with_chunks_empty_remarks():
    meta = semaphore.meta_with_chunks(TEXT, {"rem": []})
    assert meta == {"rem": []}


def test_meta_with_chunks_dict():
    meta = semaphore.meta_with_chunks({"test": TEXT, "other": 1}, {"test": META})
    assert meta == {"test": META_WITH_CHUNKS}


def test_meta_with_chunks_list():
    meta = semaphore.meta_with_chunks(["other", TEXT], {"1": META})
    assert meta == {"1": META_WITH_CHUNKS}


def test_meta_with_chunks_missing_value():
    meta = semaphore.meta_with_chunks(None, META)
    assert meta == META


def test_meta_with_chunks_missing_non_string():
    meta = semaphore.meta_with_chunks(True, META)
    assert meta == META


def test_basic_store_normalization():
    normalizer = semaphore.StoreNormalizer(project_id=1)
    event = normalizer.normalize_event({"tags": []})
    assert event["project"] == 1
    assert event["type"] == "default"
    assert event["platform"] == "other"
    assert "tags" not in event
    assert "received" in event


def test_legacy_json():
    normalizer = semaphore.StoreNormalizer(project_id=1)
    event = normalizer.normalize_event(
        raw_event='{"extra":{"x":NaN}}', legacy_python_json=True
    )
    assert event["extra"] == {"x": 0.0}

    with pytest.raises(semaphore.SemaphoreError):
        normalizer.normalize_event(raw_event='{"extra":{"x":NaN}}')
