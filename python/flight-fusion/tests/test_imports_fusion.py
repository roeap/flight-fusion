import flight_fusion


def test_import_flight_fusion():
    assert flight_fusion.__name__ == "flight_fusion"


def test_flight_fusion_python_version():
    assert flight_fusion.__version__ > "0.0.0"
