import { invoke } from '@tauri-apps/api/tauri';

import {PlexMediaResource, PlexVideoResource} from '../interfaces/PlexMediaResource';

import {PlexLoaderAPI} from './PlexLoaderAPI';

export class TauriBackend implements PlexLoaderAPI {
	login(username: string, password: string) : Promise<null> {
		return invoke('login', { username, password });
	}

	checkAuth() : Promise<boolean> {
		return invoke('check_login', {});
	}

	getMedia(media_link: string) : Promise<PlexMediaResource> {
		return invoke('get_media', {mediaLink: media_link});
	}

	playMedia(media_resource: PlexMediaResource) : Promise<null> {
		return invoke('play_media', {mediaResource: media_resource});
	}
}
