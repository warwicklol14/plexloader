import React from 'react';

import {EuiPanel, EuiFieldText, EuiFlexGroup, EuiFlexItem, EuiButton, EuiText, EuiTitle, EuiButtonIcon} from '@elastic/eui';

import PlexResourceItem from './PlexResourceItem';

interface PlexDirectoryResourceProps {
	name: string,
	children?: React.ReactNode
}

function PlexDirectoryResourceInfo(props: PlexDirectoryResourceProps) {
	return (
		<EuiPanel key={props.name}>
			<EuiTitle><h2>{props.name}</h2></EuiTitle>
			{props.children}
		</EuiPanel>
	);
}

export default PlexDirectoryResourceInfo;
