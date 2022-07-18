import {atom} from 'recoil';

import {EuiGlobalToastListToast} from '@elastic/eui';

export const toastsState = atom({
	key: 'toasts',
	default: [] as EuiGlobalToastListToast[],
});
