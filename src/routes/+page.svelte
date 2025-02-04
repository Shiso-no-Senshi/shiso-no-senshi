<script lang="ts">
	//import type { PageData } from './$types';
	import { onMount } from 'svelte';
	import { Dock } from '@shiso-no-senshi/sanbashi-no-senshi';

	//let { _data }: { data: PageData } = $props();

	//let _dock: Dock;
	let rootElement: HTMLElement;

	onMount(() => {
		let rootEl = document.getElementById('root');
		if (!rootEl) {
			console.info('Root element not found, creating');
			rootEl = document.createElement('div');
			rootEl.id = 'root';
			document.body.appendChild(rootEl);
		}
		rootElement = rootEl;

		new Dock(rootElement);
	});
</script>

<div id="root" class="container"></div>
