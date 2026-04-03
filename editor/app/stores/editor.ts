import type {RegistrySchema} from "~/utils/types";

const api = "http://localhost:11811/api"

export const useEditorStore = defineStore("editor", () => {
    const schema = shallowRef<RegistrySchema | undefined>(undefined);

    async function fetchSchema() {
        schema.value = await $fetch<RegistrySchema>(api + "/schema");
    }

    return {
        schema, fetchSchema
    }
})