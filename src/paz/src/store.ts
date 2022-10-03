import { readable, writable } from 'svelte/store'
import type { ClientState, CoreEvent } from '@paz/core';
import { Transport } from './transport';

export const state = writable({} as ClientState);
export const transport = readable(new  Transport());
export const reminderStatusEvent = writable({ReminderNewStatus: {id: "", next_duration_ms: -1}} as CoreEvent)