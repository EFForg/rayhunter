<script lang="ts">
    let {
        color = 'white',
        disabled = false,
        label,
        onclick,
    }: {
        color?: 'blue' | 'white';
        disabled?: boolean;
        label: string;
        onclick: () => void;
    } = $props();
</script>

<button
    class="
        {color === 'blue' ? 'bg-rayhunter-blue text-white' : ''}
        {disabled ? '' : 'cursor-pointer'} rayhunter-button"
    {disabled}
    {onclick}
>
    {label}
</button>
