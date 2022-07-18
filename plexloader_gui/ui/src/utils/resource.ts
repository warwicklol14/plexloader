import {PlexMediaResource, PlexVideoResourceItem, PlexDirectoryResourceItem, PlexDirectoryChild} from '../interfaces/PlexMediaResource';

function find_video_resource(video_resources: PlexVideoResourceItem[], file_name: string): PlexVideoResourceItem {
	return video_resources.find(v => v.file_name == file_name) as PlexVideoResourceItem;
}

function find_directory_resource(directory_resources: PlexDirectoryResourceItem[], file_name: string): PlexDirectoryResourceItem {
	let req_directory_resource = directory_resources.find(d => 
		d.children.find(c => c.file_name === file_name) !== undefined
	) as PlexDirectoryResourceItem;
	return { title: req_directory_resource.title, access_token: req_directory_resource.access_token, children: req_directory_resource.children.filter(c => c.file_name === file_name) };
}

export function find_media_resource(media_resource: PlexMediaResource, file_name: string) : PlexMediaResource {
	if ("VideoResource" in media_resource) {
		return { VideoResource: [find_video_resource(media_resource.VideoResource, file_name)]};
	}
	else {
		return { DirectoryResource: [find_directory_resource(media_resource.DirectoryResource, file_name)]};
	}
}
