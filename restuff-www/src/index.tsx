import 'uno.css';
import { render } from 'solid-js/web';

import App from './App';
import { client } from './rpc';

const root = document.getElementById('root');

render(() => <App />, root!);

client.user_list().then(res => console.log(res));
