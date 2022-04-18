import * as vscode from "vscode";
import {
  createChannel,
  Client,
  ChannelCredentials,
  createClientFactory,
} from "nice-grpc";
import { ModelRepositoryServiceDefinition } from "../generated/inference/model_repository";
import { GRPCInferenceServiceDefinition } from "../generated/inference/dataplane";

export const getModelRepositoryClient = (): Client<
  typeof ModelRepositoryServiceDefinition
> => {
  const { host, port, ssl } = vscode.workspace.getConfiguration("mlfusion");
  const channel = createChannel(
    `${host}:${port}`,
    ssl ? ChannelCredentials.createSsl() : ChannelCredentials.createInsecure()
  );
  return createClientFactory().create(
    ModelRepositoryServiceDefinition,
    channel
  );
};

export const geInferenceClient = (): Client<
  typeof GRPCInferenceServiceDefinition
> => {
  const { host, port, ssl } = vscode.workspace.getConfiguration("mlfusion");
  const channel = createChannel(
    `${host}:${port}`,
    ssl ? ChannelCredentials.createSsl() : ChannelCredentials.createInsecure()
  );
  return createClientFactory().create(
    GRPCInferenceServiceDefinition,
    channel
  );
};
