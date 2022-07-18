import React from 'react';

import {EuiEmptyPrompt, EuiLoadingLogo} from '@elastic/eui';

import {useCheckAuth} from '../hooks/useCheckAuth';

function CheckAuth() {
	useCheckAuth();
	return (
		<EuiEmptyPrompt
  			icon={<EuiLoadingLogo logo="logoSecurity" size="xl" />}
  			title={<h2>Checking if you're already logged in</h2>}
		/>
	);
}

export default CheckAuth;
