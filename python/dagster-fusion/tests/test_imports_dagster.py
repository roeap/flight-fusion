import dagster_fusion


def test_import_dagster_fusion():
    assert dagster_fusion.__name__ == "dagster_fusion"


def test_dagster_fusion_version():
    assert dagster_fusion.__version__ > "0.0.0"
