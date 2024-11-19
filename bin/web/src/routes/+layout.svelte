<script lang="ts">
	import '../app.css';
	let { children } = $props();
	export const prerender = true;
</script>

{@render children()}
