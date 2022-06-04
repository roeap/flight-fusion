import mlserver_fusion


def test_import_flight_fusion():
    assert mlserver_fusion.__name__ == "mlserver_fusion"


def test_flight_fusion_python_version():
    assert mlserver_fusion.__version__ > "0.0.0"
