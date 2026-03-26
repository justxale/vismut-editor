import {NodeEditor, type GetSchemes, ClassicPreset, type BaseSchemes} from "rete";
import { AreaPlugin, AreaExtensions } from "rete-area-plugin";
import {
    ConnectionPlugin,
    Presets as ConnectionPresets
} from "rete-connection-plugin";
import { VuePlugin, Presets, type VueArea2D } from "rete-vue-plugin";
import { getDOMSocketPosition } from 'rete-render-utils'
import VismutNode from "~/components/editor/graph/VismutNode.vue"
import VismutConnection from "~/components/editor/graph/VismutConnection.vue"
import VismutSocket from "~/components/editor/graph/VismutSocket.vue"

type Schemes = GetSchemes<
    ClassicPreset.Node,
    ClassicPreset.Connection<ClassicPreset.Node, ClassicPreset.Node>
>;
type AreaExtra = VueArea2D<Schemes>;

export async function createEditor(container: HTMLElement) {
    const header = useHeader()
    const socket = new ClassicPreset.Socket("socket")
    const editor = new NodeEditor<Schemes>()
    const area = new AreaPlugin<Schemes, AreaExtra>(container)

    area.signal.addPipe((event) => {
        if (event.type === 'pointerdown') {
            header.value = ""
        }
        return event
    })
    const bg = document.createElement("div");
    bg.style.backgroundImage = "linear-gradient(rgba(255,255,255,0.1) 2px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.1) 2px, transparent 1px)"
    bg.style.backgroundSize = "32px 32px, 32px 32px, 16px 16px, 16px 16px"
    bg.style.top = "-50000vh"
    bg.style.left = "-50000vw"
    bg.style.width = "100000vw"
    bg.style.height = "100000vh"
    bg.style.backgroundRepeat = "repeat"
    bg.style.position = "absolute"
    bg.style.zIndex = "-1"
    area.area.content.add(bg)

    const connection = new ConnectionPlugin<Schemes, AreaExtra>();
    const render = new VuePlugin<Schemes, AreaExtra>();

    AreaExtensions.selectableNodes(area, AreaExtensions.selector(), {
        accumulating: AreaExtensions.accumulateOnCtrl()
    });

    render.addPreset(Presets.classic.setup({
        socketPositionWatcher: getDOMSocketPosition({
            offset({x, y}, nodeId, side, key) {
                return {
                    x: x,
                    y: y + 2
                }
            }
        }),
        customize: {
            node(_) {
                return VismutNode
            },
            socket(_) {
                return VismutSocket
            },
            connection(_) {
                return VismutConnection
            }
        }
    }));

    connection.addPreset(ConnectionPresets.classic.setup());

    editor.use(area);
    area.use(connection);
    area.use(render);

    AreaExtensions.simpleNodesOrder(area);

    const a = new ClassicPreset.Node("Vismut");
    a.addControl(
        "a",
        new ClassicPreset.InputControl("text", { initial: "hello" })
    );
    a.addOutput("a", new ClassicPreset.Output(socket));
    await editor.addNode(a);

    const b = new ClassicPreset.Node("Vismut");
    b.addControl(
        "b",
        new ClassicPreset.InputControl("text", { initial: "hello" })
    );
    b.addInput("b", new ClassicPreset.Input(socket));
    await editor.addNode(b);

    await area.translate(b.id, { x: 320, y: 0 });

    await editor.addConnection(new ClassicPreset.Connection(a, "a", b, "b"));

    AreaExtensions.zoomAt(area, editor.getNodes());
    AreaExtensions.restrictor(area, {scaling: {min: 0.8, max: 3}})

    return () => {
        area.destroy()
    };
}
