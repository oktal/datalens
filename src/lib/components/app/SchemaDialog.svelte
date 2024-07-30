<script lang="ts" context="module">
	export type SchemaFormData = {
		database: string;
		name: string;
	};
</script>

<script lang="ts">
	import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
	import Button, { Label } from '@smui/button';
	import Textfield from '@smui/textfield';
	import Select, { Option } from '@smui/select';

	import { createHandle } from '$lib/dialog-handle';

	export let databases: string[] = [];

	export function show() {
		return handle.show();
	}

	let data: SchemaFormData = {
		database: databases[0] ?? '',
		name: ''
	};
	let handle = createHandle(data);
	handle.reset = (value: SchemaFormData) => {
		value = {
			database: databases[0] ?? '',
			name: ''
		};
	};
</script>

<Dialog
	bind:this={handle.dialog}
	selection
	aria-labelledby=""
	aria-describedby="list-selection-content"
	on:SMUIDialog:closed={(e) => handle.handleAction(e)}
>
	<Title id="list-selection-title">Create a new database</Title>
	<Content id="list-selection-content">
		<div class="flex flex-col m-5">
			{#if databases}
				<Select bind:value={data.database} label="Database" use={[InitialFocus]}>
					{#each databases as database}
						<Option value={database}>{database}</Option>
					{/each}
				</Select>
			{/if}
			<Textfield bind:value={data.name} label="Schema" />
		</div>
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
