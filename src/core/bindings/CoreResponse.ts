import type { ClientState } from "./ClientState";

export type CoreResponse = { key: "Success", data: null } | { key: "ClientGetState", data: ClientState } | { key: "Sum", data: number };