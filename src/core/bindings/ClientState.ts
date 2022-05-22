import type { ReminderState } from "./ReminderState";

export interface ClientState { client_uuid: string, data_path: string, reminders: Array<ReminderState>, }