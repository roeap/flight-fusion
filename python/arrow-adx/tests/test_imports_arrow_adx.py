import arrow_adx


def test_import_arrow_adx():
    assert arrow_adx.__name__ == "arrow_adx"


def test_arrow_adx_python_version():
    assert arrow_adx.__version__ > "0.0.0"
