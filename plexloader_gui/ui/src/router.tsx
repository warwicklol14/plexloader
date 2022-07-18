import {BrowserRouter, Routes, Route} from 'react-router-dom';

import CheckAuth from './components/CheckAuth';
import Login from './components/Login';
import LinkInfo from './components/LinkInfo';
import EmptyResource from './components/EmptyResource';
import PlexMediaResourceInfo from './components/PlexMediaResourceInfo';

function AppRouter() {
	return (
		<BrowserRouter>
			<Routes>
				<Route path="/" element={<CheckAuth />} />
				<Route path="/login" element={<Login />} />
				<Route path="/link" element={<LinkInfo />} >
					<Route path="empty" element={<EmptyResource />} />
					<Route path="info" element={<PlexMediaResourceInfo />} />
				</Route>
			</Routes>
		</BrowserRouter>
	);
}

export default AppRouter;
