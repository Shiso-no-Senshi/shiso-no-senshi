<script lang="ts">
	import type { PageData } from './$types';
	import { TreeView } from '$lib/panels/TreeView';
	import { createDockview, DockviewApi } from 'dockview-core';
	import { onMount } from 'svelte';

	let { data }: { data: PageData } = $props();

	let api: DockviewApi;
	let rootElement: HTMLElement;

	onMount(() => {
		let rootEl = document.getElementById('root');
		if (!rootEl) {
			rootEl = document.createElement('div');
			rootEl.id = 'root';
			document.body.appendChild(rootEl);
		}
		rootElement = rootEl;

		api = createDockview(rootElement, {
			className: 'dockview-theme',
			createComponent: (options) => {
				switch (options.name) {
					case 'TreeView':
						return new TreeView();
				}
				throw new Error(`Invalid panel type: ${options.name}`);
			},
		});

		api.addPanel({
			id: 'treeView',
			component: 'TreeView',
			title: 'Hello World!',
		});
	});
</script>

<div id="root"></div>
