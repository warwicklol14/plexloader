import React, {useState} from 'react';

import {EuiGlobalToastList, EuiToast} from '@elastic/eui';

import {useToasts} from '../hooks/useToasts';

function AppToasts() {
	const {toasts, removeToast} = useToasts();
	return (
		<EuiGlobalToastList
			toasts={toasts}
			dismissToast={removeToast}
			toastLifeTimeMs={4000}
		/>
	);
}

export default AppToasts;

interface ErrToastProps {
	title: string,
	causes: string[],
}

function ErrToast(props: ErrToastProps) {
	if (props.causes.length) {
		return (
			<EuiToast
				title={`Error: ${props.title}`}
				color="danger"
				iconType="alert"
			>
				<p>{"Caused by:"}
				{
					props.causes.map(c => 
						<>
							<br /> {c}
						</>
					)
				}
				</p>
			</EuiToast>
		);
	}
	else {
		return (
			<EuiToast
				title={`Error: ${props.title}`}
				color="danger"
				iconType="alert"
			/>
		);
	}
}

export const errorToast = (error: string) => {
	let error_split = error.split("\n");
	return (
		<ErrToast title={error_split[0]} causes={error_split.splice(1)}/>
	);
};
