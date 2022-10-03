import type { ClientCommand, ClientQuery, CoreEvent } from "@paz/core";
import { invoke } from "@tauri-apps/api";
import { EventCallback, listen, UnlistenFn } from "@tauri-apps/api/event";

export class Transport {

    constructor(){

    }

    query(query: ClientQuery): Promise<unknown> {
        return invoke('client_query', { data: query });
    }
    
    command(cmd: ClientCommand): Promise<unknown> {
        return invoke('client_command', { data: cmd });
    }

    subscribe(callback: EventCallback<CoreEvent> ): Promise<UnlistenFn> {
        return listen<CoreEvent>('core_event', callback)
    }
}