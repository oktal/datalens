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

	import { type QueryData } from './types';

	import './style.scss';

	export let data: QueryData;

	$: start = data.currentPage * data.rowsPerPage;
	$: end = Math.min(start + data.rowsPerPage, data.rows.length);
	$: slice = data.rows.slice(start, end);
	$: lastPage = Math.max(Math.ceil(data.rows.length / data.rowsPerPage) - 1, 0);

	$: hasStream = typeof data.streamId !== 'undefined';

	let loaded: boolean = true;

	function reset() {
		data = {
			...data,
			streamId: undefined,
			queryError: undefined,
			columns: [],
			rows: []
		};
	}

	async function queryNext() {
		if (typeof data.streamId !== 'undefined') {
			data.queryError = undefined;
			try {
				loaded = false;
				let res = await stream_next(data.streamId);

				if (data.queryAll) {
					while (res?.length > 0 && data.queryAll) {
						const batch = res.map((r) => r.values);
						data.rows = [...data.rows, ...batch];
						res = await stream_next(data.streamId);
					}

					if (res?.length == 0) {
						data.streamId = undefined;
					}
				} else if (res?.length > 0) {
					data.columns = res[0].columns;

					const batch = res.map((r) => r.values);
					data.rows = [...data.rows, ...batch];
				} else {
					data.streamId = undefined;
				}
			} catch (e) {
				data.queryError = e;
			} finally {
				loaded = true;
				data = data;
			}
		}
	}

	async function runQuery() {
		reset();
		try {
			data.streamId = await sql_stream(data.queryString);
			await queryNext();
		} catch (e) {
			data.queryError = e;
		}
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
			<Switch focusRing bind:checked={data.queryAll} />
			<span slot="label">Query all</span>
		</FormField>
	</div>

	<Textfield variant="outlined" textarea ripple bind:value={data.queryString} label="Query" />
	{#if data.queryError}
		<p class="text-red-600">{data.queryError}</p>
	{/if}

	<DataTable table$aria-label="Table" class="rows-table">
		<Head>
			<Row>
				{#each data.columns as column}
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
				<Select variant="outlined" bind:value={data.rowsPerPage} noLabel>
					<Option value={10}>10</Option>
					<Option value={25}>25</Option>
					<Option value={100}>100</Option>
				</Select>
			</svelte:fragment>
			<svelte:fragment slot="total">
				{start + 1}-{end} of {data.rows.length}
			</svelte:fragment>

			<IconButton
				tag="button"
				on:click={() => (data.currentPage = 0)}
				disabled={data.currentPage === 0}
			>
				<Icon icon="material-symbols:first-page" />
			</IconButton>
			<IconButton
				tag="button"
				on:click={() => data.currentPage--}
				disabled={data.currentPage === 0}
			>
				<Icon icon="material-symbols:chevron-left" />
			</IconButton>
			<IconButton
				tag="button"
				on:click={() => data.currentPage++}
				disabled={data.currentPage === lastPage}
			>
				<Icon icon="material-symbols:chevron-right" />
			</IconButton>
			<IconButton
				tag="button"
				on:click={() => (data.currentPage = lastPage)}
				disabled={data.currentPage === lastPage}
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
