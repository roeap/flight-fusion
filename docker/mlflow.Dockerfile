FROM python:3.9-slim

# install libyaml for file store performance https://mlflow.org/docs/latest/tracking.html#id53
# need to reinstall pyyaml after install libyaml
RUN pip install --upgrade pip
RUN apt-get update && \
  apt-get install -y libyaml-cpp-dev libyaml-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  pip --no-cache-dir install --force-reinstall -I pyyaml

RUN pip install psycopg2-binary azure-storage-blob mlflow

# RUN groupadd --gid 2000 mlflow \
#   && useradd -ms /bin/bash -d /home/mlflow mlflow --uid 2000 --gid 2000
# USER 2000

WORKDIR /opt/mlflow

# Tell Python *not* to buffer output. Useful to have "real-time" log output within containers.
ENV PYTHONUNBUFFERED 1

# CMD ["mlflow", "server", "--backend-store-uri", ${BACKEND_URI}, "--default-artifact-root", ${ARTIFACT_ROOT}, "--host", "0.0.0.0", "--port", "5000"]
