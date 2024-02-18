import 'uno.css';
import { render } from 'solid-js/web';

import App from './App';
import { client } from './rpc';

const root = document.getElementById('root');

render(() => <App />, root!);

client.userList().then(res => console.log(res[0]?.id));
