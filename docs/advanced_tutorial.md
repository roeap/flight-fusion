# Advanced tutorial

To put the environment to the test and emulate a productive use case, we are going to
build an interactive application for interactive exploration using a machine learning
model and backed by production grade data engineering.

This apps also serves as a performance benchmark for data exploration.

## Overview

data preprocessing is build from the accompanying [notebook](https://nbviewer.org/github/vaexio/dash-117million-taxi-app/blob/master/Prepare-taxi-data.ipynb)
to the vaex taxi app

### Data preparation

**Assets**: :material-database-eye: `raw-data`, :material-database-eye: `refined-data`

In the data preparation step we will be lading the relevant data from external sources
and store them in a raw data table. To get data into a usable state, some pre-processing
is required to generate the refined dataset.

### Feature generation

- https://blog.dataiku.com/predicting-taxi-fares-in-new-york-using-machine-learning-in-real-time

### Model training

### Data exploration

## Suggested naming conventions

sensors: name of the triggered job followed by `<job name>_on_<expressive event name>`
assuming there is a job called `process_data` and a tabled called `taxi`, a sensor
triggering `process_data` when the `taxi` table updates could be called `process_data_on_taxi_updated`.

## Technologies

- `mlfusion`: MLOps workflow
  - `flight fusion`: data serving
  - `dagster`: orchestration
  - `mlflow`: ml experiment / model tracking
  - `mlserver`: model serving
- `vaex`: for interactive exploration of data
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
- model evaluation / analysis page
- allow passing arrow schema for validation purposes
- add tracing to example app to see differences between vaex and fusion
- write delta sensor ...
- check metadata and asset config for framework stuff. e.g. save mode
- add asset materialization event listener to create delta checkpoints

### Links

- https://blog.dataiku.com/predicting-taxi-fares-in-new-york-using-machine-learning-in-real-time
- https://github.com/sdaulton/TaxiPrediction
- https://medium.com/analytics-vidhya/new-york-yellow-taxi-demand-prediction-using-machine-learning-fc697d20ff86
- https://github.com/vaexio/dash-120million-taxi-app
