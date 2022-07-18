import {useToasts} from './useToasts';

import {mediaResourceState} from '../state/mediaResource';

import {useRecoilValue} from 'recoil';

import {find_media_resource} from '../utils/resource';

import backend from '../backend';

export const usePlayResource = () => {
	const {addErrorToastHandler} = useToasts();
	const resource = useRecoilValue(mediaResourceState);
	const playHandler = (file_name: string) => {
		let req_media = find_media_resource(resource, file_name);
		backend.playMedia(req_media)
				.catch(e => addErrorToastHandler(e));
	};

	return {playHandler};
}
