import type { GroupPanelPartInitParameters, IContentRenderer } from 'dockview-core';

export class TreeView implements IContentRenderer {
	private readonly _element: HTMLElement;

	get element(): HTMLElement {
		return this._element;
	}

	constructor() {
		this._element = document.createElement('div');
	}

	init(_parameters: GroupPanelPartInitParameters): void {
		this._element.textContent = 'Hello world!';
	}
}
