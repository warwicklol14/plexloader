import {useState} from 'react';

import {useNavigate} from 'react-router-dom';

import {useRecoilState} from 'recoil';

import {linkState} from '../state/link';

export const useLinkInfo = () => {
	const navigate = useNavigate();
	const [link, setLink] = useRecoilState(linkState);
	const linkHandler = (e) => {
		setLink(e.target.value);
	};
	const submitHandler = (e) => {
		navigate('/link/info');
	};

	return {link, linkHandler, submitHandler};
};
