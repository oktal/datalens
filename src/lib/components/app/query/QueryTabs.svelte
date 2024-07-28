<script lang="ts">
	import Tab, { Label, type SMUITabAccessor } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Paper, { Content } from '@smui/paper';
	import Icon from '@iconify/svelte';
	import IconButton from '@smui/icon-button';
	import QueryTab from './QueryTab.svelte';
	import { type QueryData } from './types';
	import TableDialog from '../TableDialog.svelte';

	export function createTab() {
		let nextQueryId = ++queryId;
		queries = [
			...queries,
			{
				id: nextQueryId,
				data: createQuery()
			}
		];
	}

	let queries: Array<{ id: number; data: QueryData }> = [
		{
			id: 1,
			data: {
				queryString: '',
				queryAll: false,
				streamId: undefined,
				queryError: undefined,

				columns: [],
				rows: [],

				rowsPerPage: 10,
				currentPage: 0
			}
		}
	];

	let active = queries[0];
	let queryId = queries[0].id;

	function createQuery(): QueryData {
		return {
			queryString: '',
			queryAll: false,
			streamId: undefined,
			queryError: undefined,

			columns: [],
			rows: [],

			rowsPerPage: 10,
			currentPage: 0
		};
	}

	function closeTab(tab: { id: number }): void {
		const tabIndex = queries.findIndex((q) => q.id == tab.id);
		if (tabIndex != -1) {
			queries = [...queries.slice(0, tabIndex), ...queries.slice(tabIndex + 1)];
		}
	}

	function onTabMounted(e: CustomEvent<SMUITabAccessor>): void {
		const { tabId, element } = e.detail;

		const closeButtonQueryId = `#tab-close-${tabId.id}`;

		element?.addEventListener('click', (e: MouseEvent) => {
			if (e.target instanceof Element) {
				const closeButton = e.target.parentElement?.querySelector(closeButtonQueryId);
				if (closeButton != undefined) {
					const { clientX, clientY } = e;
					const { left, right, bottom, top } = closeButton.getBoundingClientRect();
					if (left <= clientX && clientX <= right && top <= clientY && clientY <= bottom) {
						e.stopPropagation();
						closeTab(tabId);
					}
				}
			}
		});
	}
</script>

<div class="flex flex-col gap-2 w-full">
	<!--
    Note: tabs must be unique. (They cannot === each other.)
  -->
	<TabBar tabs={queries} let:tab bind:active>
		<Tab {tab} minWidth on:SMUITab:mount={(e) => onTabMounted(e)}>
			<Label>
				Query{tab.id}
			</Label>
			<IconButton size="mini" id="tab-close-{tab.id}">
				<Icon icon="mdi:close" />
			</IconButton>
		</Tab>
	</TabBar>

	<Paper variant="unelevated" class="w-full">
		<Content>
			<QueryTab data={active.data} />
		</Content>
	</Paper>
</div>
