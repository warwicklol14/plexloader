import React from 'react';

import {EuiPanel, EuiForm, EuiFormRow, EuiFieldText, EuiFieldPassword, EuiButton, EuiPageTemplate} from '@elastic/eui';

import {useLogin} from '../hooks/useLogin';


function Login() {
	const {submitting, username, password, usernameHandler, passwordHandler, submitHandler} = useLogin();

	return (
		<EuiPageTemplate
			template="centeredBody"
			pageContentProps={{ paddingSize: 'none'}}
		>
			<EuiPanel>
				<EuiForm>
					<EuiFormRow>
						<EuiFieldText 
							placeholder="Plex username"
							value={username} 
							onChange={(e) => usernameHandler(e)}
							readOnly={submitting}
						/>
					</EuiFormRow>
					<EuiFormRow>
						<EuiFieldPassword
							placeholder="Plex password"
							type="dual"
							readOnly={submitting}
							value={password}
							onChange={(e) => passwordHandler(e)}
						/>
					</EuiFormRow>
					<EuiFormRow>
						<EuiButton
							fill
							fullWidth
							isLoading={submitting}
							onClick={(e) => submitHandler(e)}
						>
							Login
						</EuiButton>
					</EuiFormRow>
				</EuiForm>
			</EuiPanel>
		</EuiPageTemplate>
	);
};

export default Login;
