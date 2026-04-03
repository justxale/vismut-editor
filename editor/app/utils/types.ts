export type RegistrySchema = {
    nodes: NodeSchema[];
    total: number;
}

export type NodeSchema = {
    node_id: string,
    is_executable: boolean,
    is_evaluable: boolean,
    outputs: Port[],
    inputs: Port[],
}

export type Port = {
    name: string,
    kind: string
    types: string[],
}