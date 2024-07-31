<script lang="ts" context="module">
	import { type Database, type FileType } from '$lib/lens/types';

	export type CreateTable = {
		database: string;
		schema: string;
		name: string;
		partitionColumns: string[];

		fileType: FileType;
		location: string;
		options: {
			[k: string]: string | number | boolean;
		};
	};
</script>

<script lang="ts">
	import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
	import Button, { Label } from '@smui/button';
	import List, { Item, Meta } from '@smui/list';
	import IconButton from '@smui/icon-button';
	import Textfield from '@smui/textfield';
	import Select, { Option } from '@smui/select';
	import SegmentedButton, { Segment } from '@smui/segmented-button';
	import Switch from '@smui/switch';
	import FormField from '@smui/form-field';
	import Icon from '@iconify/svelte';

	import { createHandle } from '$lib/dialog-handle';
	import Fab from '@smui/fab';
	import AwsParameters from './parameters/AwsParameters.svelte';
	import LocalParameters from './parameters/LocalParameters.svelte';

	type LocationType = 'local' | 'https' | 'aws' | 'gcs';
	type CsvOptions = {
		has_header: boolean;
		delimiter: string;
	};

	export let databases: Database[] = [];

	export function show() {
		return handle.show();
	}

	function addPartitionColumn() {
		create.partitionColumns = [...create.partitionColumns, ''];
	}

	function deletePartitionColumn(index: number) {
		create.partitionColumns = [
			...create.partitionColumns.slice(0, index),
			...create.partitionColumns.slice(index + 1)
		];
	}

	function createOptions(
		prefix: string,
		options: Record<string, number | string | boolean>
	): Record<string, number | string | boolean> {
		let opts: Record<string, number | string | boolean> = {};
		Object.entries(options).forEach((k, v) => {
			const key = `${prefix}.${k}`;
			opts[key] = v;
		});

		return opts;
	}

	let create: CreateTable = {
		database: databases[0]?.name ?? '',
		schema: databases[0]?.schemas[0]?.name ?? '',
		name: '',
		partitionColumns: [],
		fileType: 'parquet',
		location: '',
		options: {}
	};
	let locationType: LocationType = 'local';
	let location: Record<LocationType, { label: string; value: string }> = {
		local: {
			label: 'Path',
			value: ''
		},
		https: {
			label: 'URL',
			value: ''
		},
		aws: {
			label: 'S3 URL',
			value: ''
		},
		gcs: {
			label: 'URL',
			value: ''
		}
	};

	let csvOptions: CsvOptions = {
		has_header: true,
		delimiter: ','
	};
	let awsOptions: AwsOptions = {
		secret_key_id: '',
		secret_access_key: '',
		region: ''
	};

	let handle = createHandle(create);
	handle.beforeAccept = (value: CreateTable): boolean => {
		if (locationType == 'aws') {
			value.options = { ...value.options, ...createOptions('aws', awsOptions) };
		}

		if (create.fileType == 'csv') {
			value.options = { ...value.options, ...createOptions('csv', csvOptions) };
		}

		value.location = location[locationType].value;
		return true;
	};

	handle.reset = (value: CreateTable) => {
		value = {
			...value,
			name: '',
			partitionColumns: [],
			fileType: 'parquet',
			location: '',
			options: {}
		};
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
		.filter((db: Database) => db.name === create.database)
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
		<div class="flex flex-col gap-2 m-5">
			<Select bind:value={create.database} label="Database" use={[InitialFocus]}>
				{#each databases as { name }}
					<Option value={name}>{name}</Option>
				{/each}
			</Select>
			<Select bind:value={create.schema} label="Schema">
				{#each schemas as { name }}
					<Option value={name}>{name}</Option>
				{/each}
			</Select>
			<Textfield bind:value={create.name} label="Table" />
			<div class="flex flex-row mt-2 items-center">
				<span>Partionned by</span>
				<Fab color="primary" mini class="ml-auto" on:click={addPartitionColumn}>
					<Icon icon="mdi:add" width="24" height="24" />
				</Fab>
			</div>
			<List>
				{#each create.partitionColumns as _partitionColumn, i}
					<Item>
						<Textfield bind:value={create.partitionColumns[i]} label="Column" />
						<Meta>
							<IconButton on:click={() => deletePartitionColumn(i)}>
								<Icon icon="mdi:delete" />
							</IconButton>
						</Meta>
					</Item>
				{/each}
			</List>
			<span class="mt-2">File type</span>
			<SegmentedButton segments={Object.entries(fileTypes)} let:segment>
				<Segment
					{segment}
					selected={segment[0] == create.fileType}
					on:click$preventDefault={() => (create.fileType = segment[0])}
				>
					<Icon icon={segment[1]} width="24" height="24" />
					<Label class="ml-2">{segment[0]}</Label>
				</Segment>
			</SegmentedButton>

			{#if create.fileType == 'csv'}
				<span class="mt-2">Options</span>
				<FormField>
					<Switch bind:checked={csvOptions.has_header} />
					<span slot="label">Has header</span>
				</FormField>
				<Textfield bind:value={csvOptions.delimiter} label="Delimiter" />
			{/if}

			<span class="mt-2">Location</span>
			<SegmentedButton segments={Object.entries(locationTypes)} let:segment>
				<Segment
					{segment}
					selected={segment[0] == locationType}
					on:click$preventDefault={() => (locationType = segment[0])}
				>
					<Icon icon={segment[1]} width="24" height="24" />
					<Label class="ml-2">{segment[0]}</Label>
				</Segment>
			</SegmentedButton>
			{#if locationType == 'aws'}
				<AwsParameters options={awsOptions} />
			{:else if locationType == 'local'}
				<LocalParameters bind:path={location[locationType].value} />
			{/if}
			<Textfield
				bind:value={location[locationType].value}
				label={location[locationType].label}
				class="w-full"
			/>
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
