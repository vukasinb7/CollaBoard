import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';

export const renderApp= () => {
    const root = ReactDOM.createRoot(document.getElementById('tldraw-container'));
    root.render(
        <App/>
    );
}





