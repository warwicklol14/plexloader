import React from 'react';
import '@elastic/eui/dist/eui_theme_dark.css';

import { RecoilRoot } from 'recoil';
import { EuiProvider } from '@elastic/eui';
import AppToasts from './components/AppToasts';
import './icons.ts';

import AppRouter from './router';

const App = () => (
  <EuiProvider colorMode="dark">
  	<RecoilRoot>
  		<AppRouter />
		<AppToasts />
	</RecoilRoot>
  </EuiProvider>
);

export default App;
