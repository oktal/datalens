<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from '@smui/button';
	import FormField from '@smui/form-field';
	import IconButton from '@smui/icon-button';
	import Switch from '@smui/switch';
	import Textfield from '@smui/textfield';
	import { open } from '@tauri-apps/api/dialog';

	export let path: string = '';

	let directory: boolean = false;

	async function openFileBrowser() {
		const selected = await open({
			multiple: false,
			directory
		});

		if (Array.isArray(selected)) {
			path = selected[0];
		} else if (selected != null) {
			path = selected;
		}
	}
</script>

<div class="flex flex-row justify-items-center items-center gap-2">
	<FormField>
		<Switch bind:checked={directory} />
		<span slot="label">Directory</span>
	</FormField>
	<Button variant="outlined" on:click={openFileBrowser}>Browse</Button>
</div>
