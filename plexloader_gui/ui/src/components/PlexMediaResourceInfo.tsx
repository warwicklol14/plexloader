import React from 'react';

import {EuiPanel, EuiFieldText, EuiFlexGroup, EuiFlexItem, EuiButton, EuiText, EuiTitle, EuiButtonIcon, EuiLoadingElastic, EuiEmptyPrompt} from '@elastic/eui';

import PlexVideoResourceInfo from './PlexVideoResourceInfo';
import PlexDirectoryResourceInfo from './PlexDirectoryResourceInfo';
import PlexResourceItem from './PlexResourceItem';

import {useMediaResource} from '../hooks/useMediaResource';

function PlexMediaResourceInfo() {
	const {resourceState, resource} = useMediaResource();
	switch (resourceState) {
		case 'hasValue':
			if ("VideoResource" in resource) {
				return	(<EuiFlexGroup direction="column">
						{
					resource.VideoResource.map(v => 
					<EuiFlexItem key={v.file_name}>
						<PlexVideoResourceInfo
							title={v.title}
							file_name={v.file_name}
						/>
					</EuiFlexItem>
					)
						}
					</EuiFlexGroup>);
			}
			else {
				return (<EuiFlexGroup direction="column">
							{
							resource.DirectoryResource.map(d =>
								<EuiFlexItem key={d.title}>
								<PlexDirectoryResourceInfo
									name={d.title}
								>
									{
										d.children.map(e =>
										<PlexResourceItem 
											
											title={e.title}
											file_name={e.file_name}
										/>
										)
								}
							</PlexDirectoryResourceInfo>
						</EuiFlexItem>
						)
						}
					</EuiFlexGroup>
				);
			}
		case 'loading':
			return (
				<EuiEmptyPrompt
					icon={<EuiLoadingElastic size="xl" />}
					title={<h2>Loading link details</h2>}
				/>
			);

		default:
			return (
				<></>
			);
	}
	
};

export default PlexMediaResourceInfo;
