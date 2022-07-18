import React from 'react';

import {EuiFlexGroup, EuiFlexItem, EuiButton, EuiText} from '@elastic/eui';

import {usePlayResource} from '../hooks/usePlayResource';

interface PlexResourceItemProps {
	title:string,
	file_name:string,
}

function PlexResourceItem(props: PlexResourceItemProps) {
	const {playHandler} = usePlayResource();
	return (
		<EuiFlexGroup alignItems="center" key={props.file_name}>
			<EuiFlexItem grow={8} >
				<EuiText>{props.file_name}</EuiText>
			</EuiFlexItem>
			<EuiFlexItem>
				<EuiButton
					iconType="playFilled"
					onClick={_ => playHandler(props.file_name)}
				>
					Play
				</EuiButton>
			</EuiFlexItem>
		</EuiFlexGroup>
	);
}

export default PlexResourceItem;
