<script>
    //@ts-nocheck

    import { browser } from "$app/environment";

    import Chart, { scales } from "chart.js/auto";
    import { onMount } from "svelte";

    //struct
    //college city: { branch: [[year, cr]] }
    import nits from "$lib/nits";

    let name;

    const colors = [
        "darkgreen",
        "#FF5733", // Orange Red
        "#33FF57", // Lime Green
        "#3357FF", // Bright Blue
        "#FF33A1", // Pink
        "#FFD700", // Gold
        "#800080", // Purple
        "#00FFFF", // Cyan
        "#4682B4", // Steel Blue
        "#D2691E", // Chocolate
        "#00FF00", // Green
        "#FF8C00", // Dark Orange
        "#8A2BE2", // Blue Violet
        "#DA70D6", // Orchid
        "#7FFF00", // Chartreuse
        "#FF1493", // Deep Pink
        "#ADFF2F", // Green Yellow
        "#1E90FF", // Dodger Blue
        "#9400D3", // Dark Violet
    ];

    $: [name, browser && name && paint()];

    let myChart;

    function paint() {
        console.log("PAINTING", name, nits[name]);

        const ctx = document.getElementById("chart");

        let labels;

        let datasets = Object.entries(nits[name]).map(([branch, v2], i) => {
            if (!labels && v2.every(x => x)) labels = v2.map(x => x[0]);

            return {
                label: branch,
                data: v2.map((x) => x?.[1]),
                borderColor: colors[i],
            };
        });

        if (myChart) myChart.destroy();

        myChart = new Chart(ctx, {
            type: "line",
            data: {
                datasets,
                labels,
            },
            options: {
                scales: {
                    y: {
                        ticks: {
                            stepSize: 1000
                        }
                    }
                }
            }
        });
    }
</script>

<main>
    <select bind:value={name} name="college">
        <option value=""> ----- SELECT ----- </option>

        {#each Object.keys(nits) as k}
            <option value={k}>{k}</option>
        {/each}
    </select>

    <canvas id="chart"></canvas>
</main>

<style>
    main {
        background: white;
        padding: 2rem;
    }
</style>
