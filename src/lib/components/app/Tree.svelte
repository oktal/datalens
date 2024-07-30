<script lang="ts">
	import Icon from '@iconify/svelte';
	import { TreeView, TreeViewItem } from '$lib/components/core/tree';

	import type { Database } from '$lib/lens/types';
	import TreeItemContent from './TreeItemContent.svelte';

	export let databases: Database[];
</script>

<TreeView>
	{#each databases as { name, schemas }}
		<TreeViewItem>
			<TreeItemContent
				slot="content"
				value={{
					kind: 'database',
					name
				}}
			/>
			{#each schemas as { name, tables }}
				<TreeViewItem>
					<TreeItemContent
						slot="content"
						value={{
							kind: 'schema',
							name
						}}
					/>
					{#each tables as { name, schema: { fields } }}
						<TreeViewItem>
							<TreeItemContent value={{ kind: 'table', name }} slot="content" />
							<TreeViewItem>
								<svelte:fragment slot="content">
									<Icon icon="mdi:table-column" width="18" height="18" />
									Columns
								</svelte:fragment>
								{#each fields as { name, data_type: { kind, logical } }}
									<TreeItemContent
										value={{
											kind: 'field',
											name,
											logical,
											type: kind
										}}
									/>
								{/each}
							</TreeViewItem>
						</TreeViewItem>
					{/each}
				</TreeViewItem>
			{/each}
		</TreeViewItem>
	{/each}
</TreeView>
