# ML Fusion

The Ml Fusion services work together to enable a seamless data science workflow. In this we are
opinionated about the best way to use certain technologies to unlock advanced interactions
and more deeply integrate the different capabilities. The main idea is to accompany the entire
journey, from raw data to model deployed into production without the need to use different
APIs and minimal code changes along the way.

- Working locally
- Testing
- Deployment

![System Overview](assets/system.drawio.svg)

## APIs

### fusion api

Core data API

- read and write data assets

### mlserver api

- model serving / inference

### mlflow api

- model tracking

## Components

### flight-fusion

- clients for interacting with data

### mlflow-fusion

- plugin for custom artifact store

### mlserver-fusion

Customized version of mlserver to handle authorization and custom serving

### dagster-fusion

- fusion api backed io manager
