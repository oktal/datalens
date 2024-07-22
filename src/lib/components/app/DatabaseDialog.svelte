<script lang="ts">
	import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
	import Button, { Label } from '@smui/button';
	import Textfield from '@smui/textfield';

	import { createHandle } from '$lib/dialog-handle';

	export function show() {
		return handle.show();
	}

	let value: string = '';
	let handle = createHandle(value);
</script>

<Dialog
	bind:this={handle.dialog}
	selection
	aria-labelledby=""
	aria-describedby="list-selection-content"
	on:SMUIDialog:closed={(e) => handle.handleAction(e)}
>
	<Title id="list-selection-title">Create a new database</Title>
	<Content id="list-selection-content" class="ml-5 w-full">
		<Textfield bind:value={handle.value} label="Database" use={[InitialFocus]} />
	</Content>
	<Actions>
		<Button>
			<Label>Cancel</Label>
		</Button>
		<Button action="accept">
			<Label>Create</Label>
		</Button>
	</Actions>
</Dialog>
