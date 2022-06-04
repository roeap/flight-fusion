import mlflow_fusion


def test_import_mlflow_fusion():
    assert mlflow_fusion.__name__ == "mlflow_fusion"


def test_mlflow_fusion_python_version():
    assert mlflow_fusion.__version__ > "0.0.0"
