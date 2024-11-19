<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest";
    import { get_manifest, get_system_stats } from "$lib/utils";
	import ManifestTable from "$lib/components/ManifestTable.svelte";
	import { onMount } from "svelte";
	import type { SystemStats } from "$lib/systemStats";
	import { AnalysisManager } from "$lib/analysisManager";

	let manifest: Manifest | undefined = $state(undefined);
	let system_stats: SystemStats | undefined = $state(undefined);
	let manager: AnalysisManager = new AnalysisManager();
	let analysis_status = $state([]);
	async function update(): Promise<void> {
    	manifest = await get_manifest();
	    system_stats = await get_system_stats();
	}

	onMount(() => {
	   const interval = setInterval(() => {
			update();
		}, 1000);

		return () => clearInterval(interval);
	});
</script>

<div class="p-8">
{#if manifest !== undefined}
    <ManifestTable manifest={manifest} />
{:else}
    <p>Loading...</p>
{/if}
</div>
