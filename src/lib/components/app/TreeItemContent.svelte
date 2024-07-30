<script lang="ts">
	import type { LogicalDataType } from '$lib/lens/types';
	import Icon from '@iconify/svelte';

	export let value:
		| {
				kind: 'database';
				name: string;
		  }
		| { kind: 'schema'; name: string }
		| { kind: 'table'; name: string }
		| { kind: 'field'; name: string; logical: LogicalDataType; type: string };

	const icons: Record<(typeof value)['kind'], string> = {
		database: 'mdi:database',
		schema: 'material-symbols:schema-outline',
		table: 'mdi:table',
		field: ''
	};

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

{#if value.kind == 'field'}
	<div class="flex gap-1">
		<Icon icon={logicalTypeIcons[value.logical]} width="18" height="18" />
		{value.name}
		<span class="text-gray-400 text-xs font-bold ml-auto mr-5">
			{value.type}
		</span>
	</div>
{:else}
	<div class="flex gap-1">
		<Icon icon={icons[value.kind]} width="18" height="18" />
		{value.name}
	</div>
{/if}
