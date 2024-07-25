<script lang="ts" context="module">
	import { type Database, type FileType } from '$lib/lens/types';

	export type TableFormData = {
		database: string;
		schema: string;
		name: string;

		file_type: FileType;
		location: string;
		options: Record<string, string | boolean>;
	};
</script>

<script lang="ts">
	import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
	import Button, { Label } from '@smui/button';
	import IconButton from '@smui/icon-button';
	import Textfield from '@smui/textfield';
	import Select, { Option } from '@smui/select';
	import SegmentedButton, { Segment } from '@smui/segmented-button';
	import Switch from '@smui/switch';
	import FormField from '@smui/form-field';
	import Icon from '@iconify/svelte';

	import { createHandle } from '$lib/dialog-handle';
	import { open } from '@tauri-apps/api/dialog';

	type LocationType = 'local' | 'https' | 'aws' | 'gcs';
	type CsvOptions = {
		has_header: boolean;
		delimiter: string;
	};

	type AwsOptions = {
		secret_key_id: string;
		secret_access_key: string;
		region: string;
	};

	export let databases: Database[] = [];

	export function show() {
		return handle.show();
	}

	function updateFileType(file_type: FileType) {
		data.file_type = file_type;
	}

	function updateLocationType(location_type: LocationType) {
		locationType = location_type;
	}

	async function browseTable() {
		const selected = await open({
			multiple: false,
			directory: browse_folder
		});

		if (Array.isArray(selected)) {
			location = selected[0];
		} else if (selected != null) {
			location = selected;
		}
	}

	let data: TableFormData = {
		database: databases[0]?.name ?? '',
		schema: databases[0]?.schemas[0]?.name ?? '',
		name: '',
		file_type: 'parquet',
		location: '',
		options: {}
	};
	let locationType: LocationType;
	let location: string = '';
	let browse_folder: boolean = false;

	let csv_options: CsvOptions = {
		has_header: true,
		delimiter: ','
	};
	let aws_options: AwsOptions = {
		secret_key_id: '',
		secret_access_key: '',
		region: ''
	};

	let handle = createHandle(data);
	handle.beforeAccept = function (value: TableFormData): boolean {
		if (locationType == 'aws') {
			value.options['aws'] = aws_options;
		}

		value.location = location;

		return true;
	};

	const fileTypes: Record<FileType, string> = {
		csv: 'carbon:csv',
		arrow: 'mdi:arrow',
		parquet: 'simple-icons:apacheparquet',
		avro: 'simple-icons:favro',
		json: 'carbon:json'
	};

	const locationTypes: Record<LocationType, string> = {
		local: 'mdi:folder',
		https: 'carbon:http',
		aws: 'mdi:aws',
		gcs: 'mdi:google'
	};

	$: schemas = databases
		.filter((db: Database) => db.name === data.database)
		.flatMap((db: Database) => db.schemas);
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
			<Select bind:value={data.database} label="Database" use={[InitialFocus]}>
				{#each databases as { name }}
					<Option value={name}>{name}</Option>
				{/each}
			</Select>
			<Select bind:value={data.schema} label="Schema">
				{#each schemas as { name }}
					<Option value={name}>{name}</Option>
				{/each}
			</Select>
			<Textfield bind:value={data.name} label="Table" />
			<span class="mt-2">File type</span>
			<SegmentedButton segments={Object.entries(fileTypes)} let:segment>
				<Segment
					{segment}
					selected={segment[0] == data.file_type}
					on:click$preventDefault={() => updateFileType(segment[0])}
				>
					<Icon icon={segment[1]} width="24" height="24" />
					<Label class="ml-2">{segment[0]}</Label>
				</Segment>
			</SegmentedButton>

			{#if data.file_type == 'csv'}
				<span class="mt-2">Options</span>
				<FormField>
					<Switch bind:checked={csv_options.has_header} />
					<span slot="label">Has header</span>
				</FormField>
				<Textfield bind:value={csv_options.delimiter} label="Delimiter" />
			{/if}
			<span class="mt-2">Location</span>
			<SegmentedButton segments={Object.entries(locationTypes)} let:segment>
				<Segment
					{segment}
					selected={segment[0] == locationType}
					on:click$preventDefault={() => updateLocationType(segment[0])}
				>
					<Icon icon={segment[1]} width="24" height="24" />
					<Label class="ml-2">{segment[0]}</Label>
				</Segment>
			</SegmentedButton>
			{#if locationType == 'aws'}
				<span class="mt-2">Options</span>
				<Textfield bind:value={aws_options.secret_key_id} label="Secret key id" />
				<Textfield bind:value={aws_options.secret_access_key} label="Secret access key" />
				<Textfield bind:value={aws_options.region} label="Region" />
			{/if}

			{#if locationType == 'local'}
				<div class="flex tems-center">
					<Textfield bind:value={location} label="Location" class="w-full" />
					<FormField>
						<Switch bind:checked={browse_folder} />
						<span slot="label">Folder</span>
					</FormField>
					<IconButton on:click={browseTable}>
						<Icon icon="flowbite:dots-horizontal-outline" width="24" height="24" />
					</IconButton>
				</div>
			{/if}
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
