import {PlexMediaResource} from '../interfaces/PlexMediaResource';

export interface PlexLoaderAPI {
	login(username: string, password: string):Promise<null>;
	checkAuth():Promise<boolean>;
	getMedia(media_link: string):Promise<PlexMediaResource>;
	playMedia(media_resource: PlexMediaResource):Promise<null>;
}
