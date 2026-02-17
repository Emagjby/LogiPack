<script lang="ts">
    export let data: { name?: string };
    function logout() {
        window.location.href = "/logout";
    }
</script>

<div class="flex flex-col justify-center items-center h-screen">
    <h1>Welcome, {data.name}</h1>
    <p>This is the dashboard page.</p>

    <div class="flex gap-2 mt-4">
        <button
            class="px-3 rounded-md py-2 bg-accent hover:bg-accent-hover cursor-pointer text-sm"
            on:click={logout}>Logout</button
        >
    </div>
</div>
