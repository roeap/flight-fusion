#!/bin/bash

ENV_CONFIGMAP_NAME=mlflow-env
SSH_KEY_SECRET_NAME=mlflow-ssh-key

# check if ssh key secret already exists
kubectl get secret $SSH_KEY_SECRET_NAME
if [ $? -eq 0 ]; then
  exit 0
fi

echo "Generate a new ssh-key"

# generate ssh key pair /tmp/id_rsa and /tmp/id_rsa.pub
ssh-keygen -b 2048 -t rsa -f /tmp/id_rsa -q -N ""

# create secret
echo "create secret $SSH_KEY_SECRET_NAME"
kubectl create secret generic $SSH_KEY_SECRET_NAME \
  --from-file=/tmp/id_rsa \
  --from-file=/tmp/id_rsa.pub

# update configmap
echo "update $ENV_CONFIGMAP_NAME"
kubectl get configmap $ENV_CONFIGMAP_NAME -o yaml > /tmp/$ENV_CONFIGMAP_NAME
PUBLIC_KEY=`cat /tmp/id_rsa.pub | base64 -w 0`
PRIVATE_KEY=`cat /tmp/id_rsa | base64 -w 0`
sed -i "s/  MLFLOW_ARTIFACT_STORE_RSA_PUB.*/  MLFLOW_ARTIFACT_STORE_RSA_PUB\: $PUBLIC_KEY/" /tmp/$ENV_CONFIGMAP_NAME
sed -i "s/  MLFLOW_ARTIFACT_STORE_RSA_PRI.*/  MLFLOW_ARTIFACT_STORE_RSA_PRI\: $PRIVATE_KEY/" /tmp/$ENV_CONFIGMAP_NAME
kubectl apply -f /tmp/$ENV_CONFIGMAP_NAME
