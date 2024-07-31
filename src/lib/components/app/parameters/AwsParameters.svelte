<script lang="ts">
	import Textfield from '@smui/textfield';
	import { type AwsOptions } from './types';
	import TabBar from '@smui/tab-bar';
	import Tab, { Label } from '@smui/tab';
	import Button from '@smui/button';

	import { awsSSOLogin } from '$lib/lens/api';

	export let options: AwsOptions;

	let sso_start_url: string = '';
	let sso_account_id: string = '';
	let sso_role: string = '';

	let mode: string = 'Basic';

	async function loginWithSSO() {
		const [accessKeyId, secretKey] = await awsSSOLogin(
			sso_start_url,
			options.region,
			sso_account_id,
			sso_role
		);

		options.secret_key_id = accessKeyId;
		options.secret_key_id = secretKey;
	}
</script>

<div class="flex flex-col gap-2 w-full">
	<TabBar tabs={['Basic', 'SSO']} let:tab bind:active={mode}>
		<Tab {tab} minWidth>
			<Label>
				{tab}
			</Label>
		</Tab>
	</TabBar>

	<div class="flex flex-col gap-1 w-80 ml-2">
		{#if mode == 'Basic'}
			<Textfield bind:value={options.secret_key_id} label="Secret key id" />
			<Textfield bind:value={options.secret_access_key} label="Secret access key" />
		{:else if mode == 'SSO'}
			<Textfield bind:value={sso_start_url} label="Start url" />
			<Textfield bind:value={sso_account_id} label="Account id" />
			<Textfield bind:value={sso_role} label="Role" />
		{/if}
		<Textfield bind:value={options.region} label="Region" />
	</div>

	{#if mode == 'SSO'}
		<Button variant="raised" class="mt-2 w-min" on:click={loginWithSSO}>Login</Button>
	{/if}
</div>
