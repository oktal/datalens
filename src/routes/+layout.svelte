<script>
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import '../app.css';
</script>

<div class="flex justify-center toast-bottom">
	<SvelteToast
		options={{
			classes: ['log'],
			theme: {}
		}}
	/>
</div>
<slot />

<style>
	:global(.log.info) {
		--toastBackground: green;
	}
	:global(.log.warn) {
		--toastBackground: red;
	}

	.toast-bottom {
		--toastContainerTop: auto;
		--toastContainerRight: auto;
		--toastContainerBottom: 1rem;
		--toastContainerLeft: auto;
		--toastWidth: 100%;
	}
</style>
