import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

function forwardConsole(
	fnName: 'log' | 'trace' | 'debug' | 'info' | 'warn' | 'error',
	logger: (...data: any[]) => void
) {
	const original = console[fnName];
	console[fnName] = (data) => {
		original(data);
		logger(typeof data === 'string' ? data : data.toString());
	};
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);
