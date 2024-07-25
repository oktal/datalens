<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from '@smui/button';
	import Textfield from '@smui/textfield';
	import DataTable, { Head, Body, Row, Cell, Pagination } from '@smui/data-table';
	import LinearProgress from '@smui/linear-progress';
	import { Label } from '@smui/common';

	import { sql_stream, stream_next } from '$lib/lens/api';
	import Select, { Option } from '@smui/select';
	import IconButton from '@smui/icon-button';
	import FormField from '@smui/form-field';
	import Switch from '@smui/switch';

	import './style.scss';

	let query: string = '';
	let queryAll: boolean = false;
	let streamId: string | undefined = undefined;

	let columns: string[] = [];
	let rows: string[][] = [];

	let rowsPerPage = 10;
	let currentPage = 0;

	$: start = currentPage * rowsPerPage;
	$: end = Math.min(start + rowsPerPage, rows.length);
	$: slice = rows.slice(start, end);
	$: lastPage = Math.max(Math.ceil(rows.length / rowsPerPage) - 1, 0);

	$: hasStream = typeof streamId !== 'undefined';

	$: if (currentPage > lastPage) {
		currentPage = lastPage;
	}

	let loaded: boolean = true;

	async function queryNext() {
		if (typeof streamId !== 'undefined') {
			loaded = false;
			let res = await stream_next(streamId);

			if (queryAll) {
				while (res.length > 0) {
					columns = res[0].columns;

					const batch = res.map((r) => r.values);
					rows = [...rows, ...batch];
					res = await stream_next(streamId);
				}
			} else if (res.length > 0) {
				columns = res[0].columns;

				const batch = res.map((r) => r.values);
				rows = [...rows, ...batch];
			}

			loaded = true;
		}
	}

	async function runQuery() {
		columns = [];
		rows = [];

		streamId = await sql_stream(query);
		await queryNext();
	}
</script>

<div class="mr-2 flex flex-col gap-5">
	<div class="flex flex-row gap-2 items-center">
		<Button variant="raised" class="flex gap-2" on:click={runQuery}>
			<Icon icon="mdi:triangle" class="rotate-90" />
			Run query
		</Button>

		<Button
			tag="button"
			variant="outlined"
			class="flex gap-2"
			disabled={!hasStream}
			on:click={queryNext}
		>
			<Icon icon="material-symbols:skip-next-rounded" width="24" height="24" />
			Query next batch
		</Button>

		<FormField>
			<Switch focusRing bind:checked={queryAll} />
			<span slot="label">Query all</span>
		</FormField>
	</div>

	<Textfield variant="outlined" textarea ripple bind:value={query} label="Query" />

	<DataTable table$aria-label="Table" class="rows-table">
		<Head>
			<Row>
				{#each columns as column}
					<Cell>{column}</Cell>
				{/each}
			</Row>
		</Head>

		<Body>
			{#each slice as row}
				<Row>
					{#each row as cell}
						<Cell>{cell}</Cell>
					{/each}
				</Row>
			{/each}
		</Body>

		<Pagination slot="paginate">
			<svelte:fragment slot="rowsPerPage">
				<Label>Rows Per Page</Label>
				<Select variant="outlined" bind:value={rowsPerPage} noLabel>
					<Option value={10}>10</Option>
					<Option value={25}>25</Option>
					<Option value={100}>100</Option>
				</Select>
			</svelte:fragment>
			<svelte:fragment slot="total">
				{start + 1}-{end} of {rows.length}
			</svelte:fragment>

			<IconButton tag="button" on:click={() => (currentPage = 0)} disabled={currentPage === 0}>
				<Icon icon="material-symbols:first-page" />
			</IconButton>
			<IconButton tag="button" on:click={() => currentPage--} disabled={currentPage === 0}>
				<Icon icon="material-symbols:chevron-left" />
			</IconButton>
			<IconButton tag="button" on:click={() => currentPage++} disabled={currentPage === lastPage}>
				<Icon icon="material-symbols:chevron-right" />
			</IconButton>
			<IconButton
				tag="button"
				on:click={() => (currentPage = lastPage)}
				disabled={currentPage === lastPage}
			>
				<Icon icon="material-symbols:last-page" />
			</IconButton>
		</Pagination>

		<LinearProgress
			indeterminate
			bind:closed={loaded}
			aria-label="Data is being loaded..."
			slot="progress"
		/>
	</DataTable>
</div>
