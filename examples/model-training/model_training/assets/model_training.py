import pyarrow as pa
from dagster import asset


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
)
def ml_model(context, training_data: pa.Table):
    pass


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
)
def model_performance(context, ml_model, test_data: pa.Table):
    pass
