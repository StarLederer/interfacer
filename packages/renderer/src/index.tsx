/* @refresh reload */
import 'virtual:uno.css';
import './style.css';
import { render } from 'solid-js/web';

import App from './App';

render(() => <App />, document.getElementById("app") as HTMLElement);
