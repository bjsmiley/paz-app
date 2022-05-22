import { readable, writable } from 'svelte/store'
import type { ClientState } from '@paz/core';
import { Transport } from './transport';

export const state = writable({} as ClientState);
export const transport = readable(new  Transport())