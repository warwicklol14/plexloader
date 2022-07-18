export type PlexMediaResource = PlexVideoResource | PlexDirectoryResource;

export interface PlexVideoResource {
	VideoResource: PlexVideoResourceItem[]
}

export interface PlexVideoResourceItem {
	title: string,
	file_name: string,
	access_token: string,
	resource_path: string,
}

export interface PlexDirectoryResource {
	DirectoryResource: PlexDirectoryResourceItem[]
}

export interface PlexDirectoryResourceItem {
	title: string,
	access_token: string,
	children: PlexDirectoryChild[]
}

export interface PlexDirectoryChild {
	title: string,
	file_name: string,
	resource_path: string,
}
