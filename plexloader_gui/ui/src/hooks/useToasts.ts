import {toastsState} from '../state/toasts';

import {useRecoilState} from 'recoil';

import {errorToast} from '../components/AppToasts';

export const useToasts = () => {
	const [toasts, setToasts] = useRecoilState(toastsState);
	const addErrorToastHandler = (e) => {
		let toast = [{
			id: `${Math.random() * 10000}`,
			text: errorToast(e)
		}];
		setToasts(toasts.concat(toast));
	};


	const removeToast = (removedToast) => {
		setToasts(toasts.filter((toast) => toast.id !== removedToast.id));
	};

	return {toasts, addErrorToastHandler, removeToast};
};
