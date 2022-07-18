import React from 'react';

import {EuiPanel, EuiFieldText, EuiFlexGroup, EuiFlexItem, EuiButton, EuiText, EuiTitle, EuiButtonIcon} from '@elastic/eui';

import PlexResourceItem from './PlexResourceItem';

interface PlexVideoResourceProps {
	title:string,
	file_name:string,
}

function PlexVideoResourceInfo(props: PlexVideoResourceProps) {
	return (
	<EuiPanel>
		<EuiTitle><h2>{props.title}</h2></EuiTitle>
		<PlexResourceItem 
			title={props.title} 
			file_name={props.file_name} 
		/>
	</EuiPanel>
	);
}

export default PlexVideoResourceInfo;
