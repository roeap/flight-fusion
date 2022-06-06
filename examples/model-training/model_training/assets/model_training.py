import numpy as np
import pyarrow as pa
from dagster import asset
from mlflow.models.signature import infer_signature
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_absolute_error, mean_squared_error, r2_score


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
    required_resource_keys={"mlflow"},
    io_manager_key="model_artifact_io",
    metadata={"file_name": "model.pkl", "artifact_path": "model"},
)
def ml_model(context, training_data: pa.Table) -> LinearRegression:
    mlflow = context.resources.mlflow
    mlflow.sklearn.autolog()

    features = [col for col in training_data.column_names if col != "target"]
    df_train = training_data.select(features).to_pandas()
    target = training_data.column("target").to_pandas()  # type: ignore

    model = LinearRegression()
    model.fit(df_train, target)

    model_signature = infer_signature(df_train, target)
    mlflow.sklearn.log_model(model, "model", signature=model_signature)

    return model


@asset(
    namespace=["demo", "model_training"],
    description="Model trained on data",
    required_resource_keys={"mlflow"},
)
def model_performance(context, ml_model, test_data: pa.Table):
    mlflow = context.resources.mlflow

    features = [col for col in test_data.column_names if col != "target"]
    df_test = test_data.select(features).to_pandas()
    target = test_data.column("target").to_pandas()  # type: ignore

    predicted_qualities = ml_model.predict(df_test)

    (rmse, mae, r2) = eval_metrics(target, predicted_qualities)
    mlflow.log_metric("rmse", rmse)
    mlflow.log_metric("r2", r2)
    mlflow.log_metric("mae", mae)


def eval_metrics(actual, pred):
    rmse = np.sqrt(mean_squared_error(actual, pred))
    mae = mean_absolute_error(actual, pred)
    r2 = r2_score(actual, pred)
    return rmse, mae, r2
