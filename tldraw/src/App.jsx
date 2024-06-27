import './App.css';
import 'tldraw/tldraw.css'
import React from "react";
import {Tldraw} from "tldraw";

export default class App extends React.Component {
    render(ro) {
        return (
            <div style={{ position: 'fixed', inset: 0 }} className="tldraw__editor"><Tldraw onMount={(editor) => {
                editor.updateInstanceState({ isReadonly: false })
            }} /></div>
        ) }
}

