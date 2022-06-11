import type { ReminderState } from "./ReminderState";

export type ClientCommand = { key: "AddOne", params: { value: number, } } | { key: "Add", params: { x: number, y: number, } } | { key: "SaveReminders", params: { reminders: Array<ReminderState>, } };