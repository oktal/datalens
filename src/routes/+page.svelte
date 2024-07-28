<script lang="ts">
	import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
	import Icon from '@iconify/svelte';
	import Button, { Label } from '@smui/button';
	import Menu from '@smui/menu';
	import List, { Item, Text, Graphic } from '@smui/list';
	import Fab from '@smui/fab';
	import { Anchor } from '@smui/menu-surface';
	import DatabaseDialog from '$lib/components/app/DatabaseDialog.svelte';
	import ExploreTree from '$lib/components/app/ExploreTree.svelte';

	import { sql, listDatabases } from '$lib/lens/api';
	import Kitchen from '@smui/snackbar/kitchen';
	import type { Database, DatabaseModel } from '$lib/lens/types';
	import { toDatabase } from '$lib/lens/types';
	import SchemaDialog from '$lib/components/app/SchemaDialog.svelte';
	import TableDialog from '$lib/components/app/TableDialog.svelte';
	import QueryTabs from '$lib/components/app/query/QueryTabs.svelte';

	let menu: Menu;
	let anchor: HTMLDivElement;
	let anchorClasses: { [k: string]: boolean } = {};

	let databaseDialog: DatabaseDialog;
	let schemaDialog: SchemaDialog;
	let tableDialog: TableDialog;
	let kitchen: Kitchen;

	let databases: Promise<Database[]> = refresh();

	function snack(text: string) {
		kitchen.push({
			props: {
				variant: 'stacked'
			},
			label: text,
			dismissButton: true
		});
	}

	async function refresh() {
		const dbs = await listDatabases().then((models: DatabaseModel[]) => {
			return models.map((model: DatabaseModel) => toDatabase(model));
		});

		return dbs;
	}

	async function createDatabase() {
		const db = await databaseDialog.show();
		const query = `CREATE DATABASE ${db}`;

		await sql(query)
			.then(() => {
				snack(`Database ${db} has been created`);
			})
			.catch((err: string) => {
				snack(`Error creating database: ${err}`);
			});

		databases = refresh();
	}

	async function createSchema() {
		const schema = await schemaDialog.show();

		const query = `CREATE SCHEMA ${schema.database}.${schema.name}`;
		await sql(query)
			.then(() => {
				snack(`Schema ${schema.name} on database ${schema.database} has been created`);
			})
			.catch((err: string) => {
				snack(`Error creating schema: ${err}`);
			});

		databases = refresh();
	}

	async function createTable() {
		const table = await tableDialog.show();
		const table_ref = `${table.database}.${table.schema}.${table.name}`;

		const query = `CREATE EXTERNAL TABLE ${table_ref} STORED AS ${table.fileType} LOCATION '${table.location}'`;

		await sql(query)
			.then(() => {
				snack(`Table ${table_ref} has been created`);
			})
			.catch((err: string) => {
				snack(`Error creating table ${table_ref}: ${err}`);
			});

		databases = refresh();
	}

	let databaseNames: string[];
	let tableDatabases: Database[];

	$: databases.then((dbs: Database[]) => (databaseNames = dbs.map((db: Database) => db.name)));
	$: databases.then((dbs: Database[]) => (tableDatabases = dbs));
</script>

<div class="flex flex-col gap-1">
	<TopAppBar variant="static" color="primary">
		<Row>
			<Section>
				<Icon icon="oui:app-lens" width="28" height="28" />
				<Title>Datalens</Title>
			</Section>
		</Row>
	</TopAppBar>

	<div class="flex flex-row gap-5">
		<div class="flex flex-col gap-2 w-1/3">
			<div class="flex flex-nowrap ml-2 gap-2 items-center">
				<div
					class={Object.keys(anchorClasses).join(' ')}
					use:Anchor={{
						addClass: (className) => {
							if (!anchorClasses[className]) {
								anchorClasses[className] = true;
							}
						},
						removeClass: (className) => {
							if (anchorClasses[className]) {
								delete anchorClasses[className];
								anchorClasses = anchorClasses;
							}
						}
					}}
					bind:this={anchor}
				>
					<Fab color="primary" class="w-12 h-12" on:click={() => menu.setOpen(true)}>
						<Icon icon="mdi:add" width="24" height="24" />
					</Fab>
					<Menu
						bind:this={menu}
						anchor={false}
						bind:anchorElement={anchor}
						anchorCorner="BOTTOM_LEFT"
					>
						<List dense>
							<Item on:SMUI:action={createDatabase}>
								<Graphic class="material-icons">
									<Icon icon="mdi:database-outline" width="24" height="24" />
								</Graphic>
								<Text>Database</Text>
							</Item>
							<Item on:SMUI:action={createSchema}>
								<Graphic class="material-icons">
									<Icon icon="ic:outline-schema" width="24" height="24" />
								</Graphic>
								<Text>Schema</Text>
							</Item>
							<Item on:SMUI:action={createTable}>
								<Graphic class="material-icons">
									<Icon icon="mdi:table" width="24" height="24" />
								</Graphic>
								<Text>Table</Text>
							</Item>
						</List>
					</Menu>

					<Button color="primary" variant="outlined">
						<Icon icon="carbon:query" width="24" height="24" />
						<Label>New query</Label>
					</Button>

					<Button color="primary" variant="outlined" on:click={() => (databases = refresh())}>
						<Icon icon="mdi:refresh" width="24" height="24" />
						<Label>Refresh</Label>
					</Button>
				</div>
			</div>
			<div class="bg-white shadow shadow-purple-500 shrink text-xs ml-2">
				{#await databases then databases}
					<ExploreTree {databases} />
				{/await}
			</div>
		</div>

		<QueryTabs />
	</div>
</div>

<DatabaseDialog bind:this={databaseDialog} />
<SchemaDialog bind:this={schemaDialog} databases={databaseNames} />
<TableDialog bind:this={tableDialog} databases={tableDatabases} />

<Kitchen bind:this={kitchen} dismiss$class="material-icons" />

<style>
	.top-app-bar-container {
		width: 100%;
		border: 1px solid var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
		margin: 0 18px 18px 0;
		background-color: var(--mdc-theme-background, #fff);

		overflow: auto;
		display: inline-block;
	}

	@media (max-width: 480px) {
		.top-app-bar-container {
			margin-right: 0;
		}
	}
</style>
