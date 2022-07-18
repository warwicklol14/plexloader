import React from 'react';

import {EuiFieldText, EuiFlexGroup, EuiFlexItem, EuiButton, EuiPage} from '@elastic/eui';

import {Outlet} from 'react-router-dom';

import {useLinkInfo} from '../hooks/useLinkInfo';

const LinkInfo = () => {
	const {link, linkHandler, submitHandler} = useLinkInfo();
	return (
		<EuiPage
		>
			<EuiFlexGroup direction="column">
				<EuiFlexItem>
					<EuiFieldText 
						fullWidth
						value={link}
						onChange={(e) => linkHandler(e)}
						placeholder="Paste a plex resource link"
						append={
							<EuiButton
								onClick={(e) => submitHandler(e)}
								fill
							>
								Submit
							</EuiButton>
						}
					/>
				</EuiFlexItem>
				<EuiFlexItem>
					<Outlet />
				</EuiFlexItem>
			</EuiFlexGroup>
		</EuiPage>
	);
};

export default LinkInfo;
