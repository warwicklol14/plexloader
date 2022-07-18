import {useEffect} from 'react';

import {useNavigate} from 'react-router-dom';

import backend from '../backend';

export const useCheckAuth = () => {
	let navigate = useNavigate();
	useEffect(() => {
		async function checkAuth() {
			try {
				await backend.checkAuth();
				navigate("/link/empty");
			}
			catch(e) {
				navigate("/login");
			}
		}
		checkAuth();
	},[]);
};
