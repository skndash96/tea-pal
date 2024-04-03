<script>
    import List from "$lib/List.svelte";

    const QUERY_PREFIX = "http://127.0.0.1:8080/query?";

    let /**@type {"by_cutoff"|"by_rank"|"by_college"|"by_coll_code"}*/ by,
        /**@type {number}*/ cutoff, //within [0,200]
        /**@type {number}*/ rank,
        /**@type {string}*/ college,
        /**@type {number}*/ coll_code,
        /**@type {"BC"|"OC"|"MBC"|"BCM"|"ST"|"SC"}*/ category,
        /**@type {Promise<any[][]> | null}*/ results;

    /**
     * @param {SubmitEvent} e
     */
    function handleSubmit(e) {
        /**@type {string}*/
        //@ts-ignore
        let prop = by.split("_").slice(1).join("_");

        let val =
            prop === "cutoff"
                ? cutoff
                : prop === "rank"
                  ? rank
                  : prop === "college"
                    ? college
                    : prop === "coll_code"
                      ? coll_code
                      : null;

        if (!val) {
            results = new Promise((_, rej) =>
                rej(`No valid Input for ${prop} was given.`),
            );
            return;
        }

        let url =
            QUERY_PREFIX +
            query_sub_string(prop, val) +
            query_sub_string("category", category);

        results = new Promise(async (resolve, reject) => {
            try {
                let data = await fetch(url, { mode: "cors" });
                let json = await data.json();

                resolve(json);
            } catch (e) {
                console.log(e);
                reject(e);
            }
        });
    }

    /**
     * @param {string} prop
     * @param {string|number} val
     * @returns {string}
     */
    function query_sub_string(prop, val) {
        return val ? `${prop}=${val}&` : "";
    }
</script>

<svelte:head>
    <title>TNEA Counselling</title>
</svelte:head>

<div class="container">    
    <h2>TNEA Counselling Stats</h2>

    <select name="query_by" bind:value={by}>
        <option selected value="by_cutoff"> Query By Cutoff </option>
        <option value="by_rank"> Query By Rank </option>
        <option value="by_college"> Query By College Name </option>
        <option value="by_coll_code"> Query By College code </option>
    </select>

    <form on:submit|preventDefault={handleSubmit}>
        {#if by === "by_cutoff"}
            <label for="cutoff">Cutoff:</label>
            <input
                name="cutoff"
                bind:value={cutoff}
                type="number"
                placeholder="Enter Cutoff to query"
            />

            <label for="category"> Category </label>
            <select name="category" bind:value={category}>
                <option selected value="">Any</option>
                <option value="BC">BC</option>
                <option value="OC">OC</option>
                <option value="MBC">MBC</option>
                <option value="BCM">BCM</option>
                <option value="SC">SC</option>
                <option value="ST">ST</option>
            </select>
        {:else if by === "by_rank"}
            <label for="rank">Rank:</label>
            <input
                name="rank"
                bind:value={rank}
                type="number"
                placeholder="Enter Rank to query"
            />

            <label for="category"> Category </label>
            <select name="category" bind:value={category}>
                <option selected value="">Any</option>
                <option value="BC">BC</option>
                <option value="OC">OC</option>
                <option value="MBC">MBC</option>
                <option value="BCM">BCM</option>
                <option value="SC">SC</option>
                <option value="ST">ST</option>
            </select>
        {:else if by === "by_college"}
            <label for="college">College Name:</label>
            <input
                name="college"
                bind:value={college}
                type="text"
                placeholder="Enter College name to query"
            />
        {:else if by === "by_coll_code"}
            <label for="coll_code">College code:</label>
            <input
                name="coll_code"
                bind:value={coll_code}
                type="number"
                placeholder="Enter College code to query"
            />
        {:else}
            Select a criteria to query
        {/if}

        <button type="submit"> Query </button>
    </form>

    {#if results}
        {#await results}
            <span> Loading.... </span>
        {:then json}
            <List records={json} />
        {:catch error}
            <span>
                Failed: {error}
            </span>
        {/await}
    {:else}
        <span> Pull a Query and the records will be displayed here </span>
    {/if}
</div>

<style>
    div.container {
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        gap: 2rem;
        padding-top: 5rem;
    }
    form {
        width: fit-content;
        margin: 0 auto;
        vertical-align: center;
        /* float: left; */
        height: 16rem;
    }
    form > * {
        display: block;
        margin: 1rem 0;
    }

    form button {
        background: khaki;
        margin: 0 auto;
    }
</style>
