

let elements=' [{"id":"InTX9cQUE9-aAQGR7wnXe","type":"freedraw","x":473.77777099609375,"y":200.2222442626953,"width":184,"height":255.99998474121094,"angle":0,"str' +
    'okeColor":"#1e1e1e","backgroundColor":"transparent","fillStyle":"solid","strokeWidth":2,"strokeStyle":"solid","roughness":1,"opacity":100,"groupIds":[],"frameId":null,"roundness":n' +
    'ull,"seed":34880528,"version":47,"versionNonce":241796848,"isDeleted":false,"boundElements":null,"updated":1719772562269,"link":null,"locked":false,"points":[[0,0],[0.8888854980468' +
    '75,6.22222900390625],[3.555572509765625,18.666641235351562],[7.111114501953125,31.999984741210938],[13.333343505859375,47.99998474121094],[19.555572509765625,76.44441223144531],[24' +
    '.888885498046875,91.55552673339844],[27.555572509765625,106.66664123535156],[32,131.55552673339844],[33.777801513671875,148.44444274902344],[35.555572509765625,161.7777862548828],[' +
    '37.333343505859375,173.3333282470703],[40.888916015625,191.99998474121094],[41.77777099609375,200.8888702392578],[42.66668701171875,209.7777557373047],[43.5555419921875,215.1111297' +
    '607422],[45.3333740234375,224.00001525878906],[48.888916015625,235.55552673339844],[49.77777099609375,239.99998474121094],[51.5555419921875,243.55552673339844],[52.4444580078125,24' +
    '8.8888702392578],[54.22222900390625,251.55552673339844],[55.11114501953125,252.4444122314453],[56,253.3333282470703],[57.77777099609375,253.3333282470703],[60.4444580078125,255.999' +
    '98474121094],[62.22222900390625,255.99998474121094],[64.888916015625,252.4444122314453],[72.888916015625,239.11109924316406],[77.3333740234375,228.44444274902344],[80.888916015625,' +
    '217.7777557373047],[86.22222900390625,204.4444122314453],[97.77777099609375,169.7777557373047],[107.5555419921875,150.2222137451172],[118.22222900390625,129.7777557373047],[128.888' +
    '916015625,109.33332824707031],[140.4444580078125,93.33332824707031],[156.4444580078125,70.22221374511719],[162.66668701171875,60.44441223144531],[167.11114501953125,54.222198486328' +
    '125],[172.44451904296875,48.88887023925781],[178.66668701171875,38.222198486328125],[180.4444580078125,36.444427490234375],[183.11114501953125,32.88890075683594],[184,31.999984741210938],[184,31.999984741210938]],"pressures":[],"simulatePressure":true,"lastCommittedPoint":[184,31.999984741210938]}]'
export function render_excalidraw(email,id,data,role) {
    elements=JSON.parse(data)
    for (let i = 0; i <elements['elements'].length; i++) {
        elements['elements'][i]=JSON.parse(elements['elements'][i])
    }

    console.log(elements)
    let old_elements=JSON.parse(JSON.stringify(elements['elements']));
    const exampleSocket = new WebSocket(
        "ws://localhost:3000/ws",
    );
    let excalidrawapi=null;

    exampleSocket.onopen = () => {
        exampleSocket.send('{"email":"'+ email +'","board_id":"'+id+'"}');
    }

    exampleSocket.onmessage=(message)=>{
        console.log(message.data)
        let index = message.data.indexOf(':');
        if (index!==-1) {
            let new_msg = message.data.substring(index + 1);
            let item=JSON.parse(new_msg)
            let index2 = elements['elements'].findIndex(el => el.id === item.id);
            index2 !== -1 ? elements['elements'][index2] = item : elements['elements'].push(item);
        }
        if (excalidrawapi!=null){
            excalidrawapi.updateScene(elements);
        }
    }
    function debounce(func, wait) {
        let timeout;
        return function(...args) {
            clearTimeout(timeout);
            timeout = setTimeout(() => func.apply(this, args), wait);
        };
    }
    const debouncedOnChange = debounce((e) => {
        let diff=e.length-elements['elements'].length
        if (diff>0){
            let i=1;
            while (i<=diff){
                exampleSocket.send(JSON.stringify(e[e.length-i]));
                elements['elements'].push(e[e.length-i]);
                i++
            }
        }else{
            e.forEach(itemA => {
                const itemB = old_elements.find(item => item.id === itemA.id);
                if (itemB && itemA.version !== itemB.version) {
                    exampleSocket.send(JSON.stringify(itemA));
                }
            });
        }
        old_elements=JSON.parse(JSON.stringify(e));

    }, 800);

    let element = React.createElement(
        React.Fragment,
        null,
        React.createElement(
            "div",
            {
                style: { height: "100vh" },
            },
            React.createElement(ExcalidrawLib.Excalidraw, { viewModeEnabled: role==="Viewer",initialData:elements ,onChange:(e)=>{debouncedOnChange(e)},excalidrawAPI:(api)=>{excalidrawapi=api;} })
        )
    );
    const excalidrawWrapper = document.getElementById("root");

    ReactDOM.render(element, excalidrawWrapper);
}
