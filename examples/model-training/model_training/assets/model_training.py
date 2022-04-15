import pyarrow as pa
from dagster import asset
from sklearn.linear_model import LinearRegression


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
    required_resource_keys={"mlflow"},
)
def ml_model(context, training_data: pa.Table) -> LinearRegression:
    mlflow = context.resources.mlflow
    mlflow.sklearn.autolog()

    features = [col for col in training_data.column_names if col != "target"]
    df_train = training_data.select(features).to_pandas()
    target = training_data.column("target").to_pandas()

    model = LinearRegression()
    model.fit(df_train, target)

    mlflow.end_run()

    return model


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
    required_resource_keys={"mlflow"},
)
def model_performance(context, ml_model, test_data: pa.Table):
    pass
