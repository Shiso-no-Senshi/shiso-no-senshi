<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<main class="container">
	<h1>Shisō no Senshi</h1>
	<h2>Loading...</h2>
	<div>
		<h3>Environment</h3>
		<div>Architecture: {data.os.architecture}</div>
		<div>Operating System: {data.os.platform} v{data.os.version}</div>
		<div>Hostname: {data.os.hostname}</div>
		<div>End of Line: {data.os.eol == '\n' ? 'Posix' : 'Windows'}</div>
		<div>Locale: {data.os.locale}</div>
	</div>
</main>
