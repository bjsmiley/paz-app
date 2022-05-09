
export type ClientCommand = { key: "AddOne", params: { value: number, } } | { key: "Add", params: { x: number, y: number, } };