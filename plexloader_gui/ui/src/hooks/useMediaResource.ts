import {useRecoilValueLoadable} from 'recoil';

import {useNavigate} from 'react-router-dom';

import {mediaResourceState} from '../state/mediaResource';

import {useToasts} from './useToasts';

export const useMediaResource = () => {
	const {addErrorToastHandler} = useToasts();
	const navigate = useNavigate();
	const resourceLoadable = useRecoilValueLoadable(mediaResourceState);

	const resourceState = resourceLoadable.state;
	const resource = resourceLoadable.contents;

	if (resourceState == 'hasError') {
		addErrorToastHandler(resource);
		navigate('/link/empty');
	}

	return {resourceState, resource};
}
