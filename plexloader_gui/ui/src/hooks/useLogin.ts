import {useState, useEffect} from 'react';

import {useNavigate} from 'react-router-dom';

import {useToasts} from './useToasts';

import backend from '../backend';

export const useLogin = () => {
	const [submitting, setSubmitting] = useState(false);
	const {addErrorToastHandler} = useToasts();

	useEffect(() => {
	 async function login() {
		try {
			await backend.login(username, password);
			setSubmitting(false);
			navigate("/link/empty");
		}
		catch(e) {
			addErrorToastHandler(e);
			setSubmitting(false);
		}
	}
	if (submitting) {
		login();
	}
	}, [submitting]);

	const navigate = useNavigate();
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const passwordHandler = (e) => {
		setPassword(e.target.value);
	};
	const usernameHandler = (e) => {
		setUsername(e.target.value);
	};
	const submitHandler = (_) => {
		setSubmitting(true);
	};

	return {submitting, username, password, usernameHandler, passwordHandler, submitHandler};
};
