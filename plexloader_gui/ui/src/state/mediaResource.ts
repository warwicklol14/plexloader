import {selector} from 'recoil';

import {linkState} from './link';

import backend from '../backend';

export const mediaResourceState = selector({
	key: 'mediaResource',
	get: async ({get}) => {
		let resource = await backend.getMedia(get(linkState));
		return resource;
	},
});

