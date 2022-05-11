import App from './App.svelte';
import {provideFASTDesignSystem, fastCard, fastButton } from '@microsoft/fast-components';

provideFASTDesignSystem()
.register(
	fastCard(),
	fastButton()
);

const app = new App({
	target: document.body
	// props: {
	// 	name: 'world'
	// }
});

export default app;