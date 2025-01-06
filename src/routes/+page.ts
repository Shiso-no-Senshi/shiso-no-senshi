import type { PageLoad } from './$types';
import { arch, eol, hostname, locale, platform, version } from '@tauri-apps/plugin-os';

export const load: PageLoad = async () => {
	return {
		os: {
			architecture: arch(),
			platform: platform(),
			version: version(),
			hostname: await hostname(),
			eol: eol(),
			locale: await locale(),
		},
	};
};
