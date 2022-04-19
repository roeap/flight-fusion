import * as vscode from "vscode";
import {
  createChannel,
  Client,
  ChannelCredentials,
  createClientFactory,
} from "nice-grpc";
import { FlightServiceDefinition } from "../generated/arrow/flight/protocol/Flight";

export const getFlightClient = (): Client<typeof FlightServiceDefinition> => {
  const { host, port, ssl } = vscode.workspace.getConfiguration("mlfusion");
  const channel = createChannel(
    `${host}:${port}`,
    ssl ? ChannelCredentials.createSsl() : ChannelCredentials.createInsecure()
  );
  return createClientFactory().create(FlightServiceDefinition, channel);
};

export function getMetaData(name: string, namespace: string[]) {
  return;
}
