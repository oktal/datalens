<script lang="ts">
	import Icon from '@iconify/svelte';
	import { TreeView, TreeViewItem } from '$lib/components/core/tree';

	import type { Database, LogicalDataType } from '$lib/lens/types';

	export let databases: Database[];

	const logicalTypeIcons: Record<LogicalDataType, string> = {
		string: 'carbon:string-text',
		boolean: 'carbon:boolean',
		null: 'mdi:null',
		integer: 'carbon:string-integer',
		decimal: 'carbon:character-decimal',
		timestamp: 'carbon:time',
		date: 'carbon:calendar',
		time: 'carbon:time'
	};
</script>

<TreeView>
	{#each databases as { name, schemas }}
		<TreeViewItem label={name}>
			<Icon slot="item-start" icon="mdi:database" width="18" height="18" />
			{#each schemas as { name, tables }}
				<TreeViewItem label={name}>
					<Icon slot="item-start" icon="material-symbols:schema-outline" width="18" height="18" />
					{#each tables as { name, schema: { fields } }}
						<TreeViewItem label={name}>
							<Icon slot="item-start" icon="mdi:table" width="18" height="18" />
							<TreeViewItem label="Columns">
								<Icon slot="item-start" icon="mdi:table-column" width="18" height="18" />
								{#each fields as { name, data_type: { kind, logical } }}
									<TreeViewItem label="{name} [{kind}]">
										<Icon
											slot="item-start"
											icon={logicalTypeIcons[logical]}
											width="18"
											height="18"
										/>
									</TreeViewItem>
								{/each}
							</TreeViewItem>
						</TreeViewItem>
					{/each}
				</TreeViewItem>
			{/each}
		</TreeViewItem>
	{/each}
</TreeView>
