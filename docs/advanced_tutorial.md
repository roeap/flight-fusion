# Advanced tutorial

To put the environment to the test and emulate a productive use case, we are going to
build an interactive application for interactive exploration using a machine learning
model and backed by production grade data engineering.

This apps also serves as a performance benchmark for data exploration.

## Overview

data preprocessing is build from the accompanying [notebook](https://nbviewer.org/github/vaexio/dash-117million-taxi-app/blob/master/Prepare-taxi-data.ipynb)
to the vaex taxi app

### Data preparation

### Feature generation

- https://blog.dataiku.com/predicting-taxi-fares-in-new-york-using-machine-learning-in-real-time

### Model training

### Data exploration

## Technologies

- `mlfusion`: MLOps workflow
  - `flight fusion`: data serving
  - `dagster`: orchestration
  - `mlflow`: ml experiment / model tracking
  - `mlserver`: model serving
- `VAEX`: for local caching
- `dash_antd`: UI components for Dash

## TODO

- vaex vs datafusion for data exploration
- use vaex as app-local cache
- dash-pivottable for interactive exploration
- dash_ant components
- dagster pipeline using backfills
- wrap preprocessing graph in asset to demo wrapping
- write features into separate store and merge in datafusion before training
- use fan out for parameter grid search etc ...tracking in mlflow?

### Links

- https://blog.dataiku.com/predicting-taxi-fares-in-new-york-using-machine-learning-in-real-time
- https://github.com/sdaulton/TaxiPrediction
