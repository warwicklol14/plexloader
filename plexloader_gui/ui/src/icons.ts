import { appendIconComponentCache } from '@elastic/eui/es/components/icon/icon';
import { icon as EuiIconEye } from '@elastic/eui/es/components/icon/assets/eye';
import { icon as EuiIconEyeClosed } from '@elastic/eui/es/components/icon/assets/eye_closed';
import { icon as EuiIconLock } from '@elastic/eui/es/components/icon/assets/lock';
import { icon as EuiAlert } from '@elastic/eui/es/components/icon/assets/alert';
import { icon as EuiSecurityLoading } from '@elastic/eui/es/components/icon/assets/logo_security';
import { icon as EuiLoading} from '@elastic/eui/es/components/icon/assets/logo_elastic';
import { icon as EuiIconPlay } from '@elastic/eui/es/components/icon/assets/playFilled';
import { icon as EuiIconDownload } from '@elastic/eui/es/components/icon/assets/download';
import { icon as EuiNotebookApp} from '@elastic/eui/es/components/icon/assets/app_notebook';
import { icon as EuiCross } from '@elastic/eui/es/components/icon/assets/cross';

appendIconComponentCache({
  eye: EuiIconEye,
  cross: EuiCross,
  eyeClosed: EuiIconEyeClosed,
  lock: EuiIconLock,
  alert: EuiAlert,
  logoSecurity: EuiSecurityLoading,
  logoElastic: EuiLoading,
  download: EuiIconDownload,
  playFilled: EuiIconPlay,
  notebookApp: EuiNotebookApp,
});
