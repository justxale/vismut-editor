import { useVueFlow } from '@vue-flow/core'
import { ref, watch } from 'vue'

export default function useDragAndDrop() {
    const draggedType = ref<string | undefined>(undefined)
    const isDragOver = ref<boolean>(false)
    const isDragging = ref<boolean>(false)

    const { addNodes, screenToFlowCoordinate, onNodesInitialized, updateNode } = useVueFlow()

    watch(isDragging, (dragging) => {
        document.body.style.userSelect = dragging ? 'none' : ''
    })

    function onDragStart(event: DragEvent) {
        if (event.dataTransfer) {
            event.dataTransfer.setData('application/vueflow', 'vismut')
            event.dataTransfer.effectAllowed = 'move'
        }

        draggedType.value = 'vismut'
        isDragging.value = true

        document.addEventListener('drop', onDragEnd)
    }

    function onDragOver(event: DragEvent) {
        event.preventDefault()

        if (draggedType.value) {
            isDragOver.value = true

            if (event.dataTransfer) {
                event.dataTransfer.dropEffect = 'move'
            }
        }
    }

    function onDragLeave() {
        isDragOver.value = false
    }

    function onDragEnd() {
        isDragging.value = false
        isDragOver.value = false
        draggedType.value = undefined
        document.removeEventListener('drop', onDragEnd)
    }

    function onDrop(event: DragEvent) {
        const position = screenToFlowCoordinate({
            x: event.clientX,
            y: event.clientY,
        })

        const nodeId = crypto.randomUUID()

        const newNode = {
            id: nodeId,
            type: 'vismut',
            position,
            data: { label: nodeId },
        }

        const { off } = onNodesInitialized(() => {
            updateNode(nodeId, (node) => ({
                position: { x: node.position.x - node.dimensions.width / 2, y: node.position.y - node.dimensions.height / 2 },
            }))

            off()
        })
        addNodes(newNode)
    }

    return {
        draggedType,
        isDragOver,
        isDragging,
        onDragStart,
        onDragLeave,
        onDragOver,
        onDrop,
    }
}
