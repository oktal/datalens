<script lang="ts">
	import Icon from '@iconify/svelte';
	import IconButton from '@smui/icon-button';

	let expanded = false;

	$: hasChildren = $$slots.default;

	$: needToggle = expanded || hasChildren;
	$: display = expanded ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0';

	function expand() {
		expanded = !expanded;
	}
</script>

<div class="flex flex-col">
	<div class="flex items-center overflow-auto">
		{#if hasChildren}
			<IconButton on:click={expand} class="w-8 h-8 p-1">
				{#if expanded}
					<Icon icon="carbon:caret-down" />
				{:else if needToggle}
					<Icon icon="carbon:caret-right" />
				{/if}
			</IconButton>
		{/if}
		<slot name="content" />
	</div>

	{#if hasChildren}
		<div class="grid ml-5 transition-all duration-300 ease-in-out {display}">
			<div class="flex flex-col gap-2 overflow-hidden">
				<slot />
			</div>
		</div>
	{/if}
</div>
