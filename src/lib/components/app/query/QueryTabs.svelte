<script lang="ts">
	import Tab, { Label, type SMUITabAccessor } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Paper, { Content } from '@smui/paper';
	import Icon from '@iconify/svelte';
	import IconButton from '@smui/icon-button';
	import QueryTab from './QueryTab.svelte';

	let active = 'Home';

	function closeTab(tab: string): void {
		console.log(`close ${tab}`);
	}

	function onTabMounted(e: CustomEvent<SMUITabAccessor>): void {
		const { tabId, element } = e.detail;

		const closeButtonQueryId = `#tab-close-${tabId}`;

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
	<TabBar tabs={['Query1', 'Query2', 'Query3']} let:tab bind:active>
		<Tab {tab} minWidth on:SMUITab:mount={(e) => onTabMounted(e)}>
			<Label>
				{tab}
			</Label>
			<IconButton size="mini" id="tab-close-{tab}">
				<Icon icon="mdi:close" />
			</IconButton>
		</Tab>
	</TabBar>

	<Paper variant="unelevated" class="w-full">
		<Content>
			<QueryTab />
		</Content>
	</Paper>
</div>
