let elements = '[]'

export function render_excalidraw(token, id, data, role) {
    elements = JSON.parse(data)
    for (let i = 0; i < elements['elements'].length; i++) {
        elements['elements'][i] = JSON.parse(elements['elements'][i])
    }
    let old_elements = JSON.parse(JSON.stringify(elements['elements']));

    let excalidrawApi = null;
    const ws_client = new WebSocket("ws://localhost:3000/ws");

    ws_client.onopen = () => {
        ws_client.send('{"token":"' + token + '","board_id":"' + id + '"}');
    }

    ws_client.onmessage = (message) => {
        let index = message.data.indexOf(':');
        if (index !== -1) {
            let itemStr = message.data.substring(index + 1);
            let item = JSON.parse(itemStr)
            let elementIndex = elements['elements'].findIndex(el => el.id === item.id);
            elementIndex !== -1 ? elements['elements'][elementIndex] = item : elements['elements'].push(item);
        } else {
            jSuites.notification({
                name: 'Info',
                message: message.data,
                position: 'bottom-left'
            })
        }

        if (excalidrawApi != null) {
            excalidrawApi.updateScene(elements);
        }
    }

    window.addEventListener('popstate', () => {
        if (ws_client.readyState === WebSocket.OPEN) {
            ws_client.close();
        }
    });


    const debounce = (func, wait) => {
        let timeout;
        return function (...args) {
            clearTimeout(timeout);
            timeout = setTimeout(() => func.apply(this, args), wait);
        };
    }

    const debouncedOnChange = debounce((e) => {
        let diff = e.length - elements['elements'].length
        if (diff > 0) {
            let i = 1;
            while (i <= diff) {
                ws_client.send(JSON.stringify(e[e.length - i]));
                elements['elements'].push(e[e.length - i]);
                i++
            }
        } else {
            e.forEach(itemA => {
                const itemB = old_elements.find(item => item.id === itemA.id);
                if (itemB && itemA.version !== itemB.version) {
                    ws_client.send(JSON.stringify(itemA));
                }
            });
        }
        old_elements = JSON.parse(JSON.stringify(e));

    }, 200);

    let element = React.createElement(
        React.Fragment,
        null,
        React.createElement(
            "div",
            {
                style: {height: "100vh"},
            },
            React.createElement(ExcalidrawLib.Excalidraw, {
                viewModeEnabled: role === "Viewer",
                initialData: elements,
                onChange: (e) => {
                    debouncedOnChange(e)
                },
                excalidrawAPI: (api) => {
                    excalidrawApi = api;
                }
            })
        )
    );
    const excalidrawWrapper = document.getElementById("excalidraw-root");

    ReactDOM.render(element, excalidrawWrapper);
}
