import React, {useState} from 'react';

import {EuiEmptyPrompt} from '@elastic/eui';


function EmptyResource() {
	return (
	<EuiEmptyPrompt
  		iconType="notebookApp"
  		iconColor="default"
  		title={<h2>Paste a link to get started</h2>}
  		titleSize="xs"
	/>
	);
}

export default EmptyResource;
