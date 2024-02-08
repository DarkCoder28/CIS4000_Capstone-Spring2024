let websocket;


register_websocket_plugin = function (importObject) { // To give Rust a JS function
    importObject.env.connect_ws_socket = async function (js_object) {
        if (websocket === undefined || websocket === null || websocket.readyState == WebSocket.CLOSED) {
            websocket = new WebSocket((location.protocol=="https:"?"wss":"ws") + "://" + location.host + "/api/connect_session");
            websocket.onmessage = (event) => wasm_exports.receive_ws_message(event.data);
            wasm_exports.update_ws_status("CONNECTING");
            while (websocket.readyState == WebSocket.CONNECTING) {
                await delay(100);
            }
            if (websocket.readyState == WebSocket.CONNECTING)
                wasm_exports.update_ws_status("CONNECTING");
            else if (websocket.readyState == WebSocket.OPEN)
                wasm_exports.update_ws_status("OPEN");
            else if (websocket.readyState == WebSocket.CLOSING)
                wasm_exports.update_ws_status("CLOSING");
            else if (websocket.readyState == WebSocket.CLOSED)
                wasm_exports.update_ws_status("CLOSED");
        }
    }
    importObject.env.send_ws_message = function (js_object) {
        console.log(js_object)
    }
}

// miniquad_add_plugin receive an object with two fields: register_plugin and on_init. Both are functions, both are optional.
miniquad_add_plugin({register_plugin: register_websocket_plugin});


async function delay(milliseconds){
    return new Promise(resolve => {
        setTimeout(resolve, milliseconds);
    });
}