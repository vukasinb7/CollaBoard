

let elements='[]'
export function render_excalidraw(token,id,data,role) {
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
        exampleSocket.send('{"token":"'+ token +'","board_id":"'+id+'"}');
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
        else {
            jSuites.notification({
                name: 'Info',
                message: message.data,
                position: 'bottom-left'
            })
        }
        if (excalidrawapi!=null){
            excalidrawapi.updateScene(elements);
        }
    }

    window.addEventListener('popstate', () => {
        if (exampleSocket.readyState === WebSocket.OPEN) {
            exampleSocket.close();
        }
    });
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

    }, 200);

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
