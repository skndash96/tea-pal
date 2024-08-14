<script>
    import List from "$lib/List.svelte";

    let /**@type {string[]}*/ institute = [],
        /**@type {string[]}*/ course = [],
        /**@type {"HS"|"OS"|"AI" []}*/ quota = ["AI"],
        /**@type {"OPEN"|"EWS"|"OBC-NCL"|"SC"|"ST"|"PwD"|""}*/ seat = "OPEN",
        /**@type {"neutral"|"female"|""}*/ gender = "neutral",
        /**@type {number|null}*/ cr = null,
        /**@type {2016|2017|2018|2019|2020|2021|2022|2023|2024 []}*/ year = [],
        /**@type {1|2|3|4|5|6 []}*/ round = [],
        /**@type {Promise<any[]> | null}*/ results = [];

    let /**@type {string}*/ _institute, /**@type {string}*/ _course;

    function addInstitute() {
        if (!_institute) return;
        institute.push(_institute);
        institute = institute;
        _institute = "";
    }
    /**@param {number} i*/
    function removeInstitute(i) {
        institute.splice(i, 1);
        institute = institute;
    }

    function addCourse() {
        if (!_course) return;
        course.push(_course);
        course = course;
        _course = "";
    }
    /**@param {number} i*/
    function removeCourse(i) {
        course.splice(i, 1);
        course = course;
    }

    const URL_PREFIX = "/api/josaa?";

    function handleSubmit() {
        let url = URL_PREFIX
            + get_sub_str("name", institute)
            + get_sub_str("course", course)
            + get_sub_str("quota", quota)
            + get_sub_str("seat", seat)
            + get_sub_str("gender", gender)
            + get_sub_str("cr", cr)
            + get_sub_str("year", year)
            + get_sub_str("round", round);
        
        console.log(url);

        results = new Promise(async (res,rej) => {
            try {
                let response = await fetch(url);
                res(await response.json());
            } catch (e) {
                rej(e);
            }
        });
    }

    /**
     * @returns {string}
     * @param {string} key
     * @param {number|string|null|any[]} val
     */
    function get_sub_str(key, val) {
        let isarr = typeof(val) === "object";

        //@ts-ignore
        if (key && val && (isarr ? !!val.length : true)) {
            //@ts-ignore
            return `${key}=${isarr ? val.join(";") : val}&`
        } else {
            return "";
        }
    }
</script>

<style>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>

<svelte:head>
    <title>JOSAA Counselling</title>
    <meta name="title" content="JOSAA Counselling Query" />
    <meta name="description" content="Query the TNEA Counselling results (2016-23) for aid in decision of your counselling list." />
</svelte:head>

<div class="py-8 bg-white text-black">
    <h1 class="text-2xl font-black text-center"> JOSAA Counselling </h1>

    <form class="p-4 w-full mx-auto max-w-lg flex flex-col gap-4" on:submit|preventDefault={handleSubmit}>
        <div>
            <label for="institute">
                Institute
                <span>
                    ?
                    <small class="hidden"> Add institute names. Avoid abbreviations like NIT, IIT. A word like 'National' would do. </small>
                </span>
            </label>

            <div>
                <ul class="mb-2 flex flex-wrap gap-2">
                    {#each institute as name, i}
                        <button
                            class="badge badge-neutral"
                            on:click={() => removeInstitute(i)}
                        >
                            {name} <span class="ml-2 text-red-500">x</span>
                        </button>
                    {/each}
                </ul>
                <form class="flex" on:submit|preventDefault={addInstitute}>
                    <input
                        class="input input-bordered"
                        type="text"
                        bind:value={_institute}
                        placeholder="name"
                    />
                    <button class="btn" type="submit"> Add </button>
                </form>
            </div>
        </div>

        <div>
            <label for="course">
                Course
                <span>
                    ?
                    <small class="hidden"> Add courses. Just a word like 'Computer' would do.  Avoid abbreviations like CS. </small>
                </span>
            </label>
            <div>
                <ul class="mb-2 flex flex-wrap gap-2">
                    {#each course as name, i}
                        <button
                            class="badge badge-neutral"
                            on:click={() => removeCourse(i)}
                        >
                            {name} x
                        </button>
                    {/each}
                </ul>
                <form class="flex" on:submit|preventDefault={addCourse}>
                    <input
                        class="input input-bordered"
                        type="text"
                        bind:value={_course}
                        placeholder="course"
                    />
                    <button class="btn" type="submit"> Add </button>
                </form>
            </div>
        </div>

        <div class="flex flex-col">
            <label for="quota">
                Quota
                <span>
                    ?
                    <small class="hidden"> Select quota. If a particular inst. is selected in name of the inst., then select HS or OS. If not leave this. </small>
                </span>
            </label>

            <select class="select select-bordered" name="quota" bind:value={quota} multiple>
                <option value="HS"> HS (Home State) </option>
                <option value="OS"> OS (Other State) </option>
                <option value="AI" selected> AI (All India - for IITs and IIITs) </option>
            </select>
        </div>
        
        <div class="flex flex-col">
            <label for="seat"> Select Seat Type </label>

            <select class="select select-bordered" name="seat" bind:value={seat}>
                <option value=""> Any </option>
                <option value="OPEN" selected> OPEN </option>
                <option value="EWS"> EWS </option>
                <option value="OBC-NCL"> OBC-NCL </option>
                <option value="SC"> SC </option>
                <option value="ST"> ST </option>
                <option value="PwD"> PwD (OPEN, EWS, OBC-NCL, SC, ST) </option>
            </select>
        </div>

        <div class="flex flex-col">
            <label for="gender">
                Select Gender
            </label>

            <select class="select select-bordered" name="gender" bind:value={gender}>
                <option value=""> Any </option>
                <option value="neutral" selected> Neutral </option>
                <option value="female"> Female Only (+Supernumerary) </option>
            </select>
        </div>

        <div class="flex flex-col">
            <label for="rank">
                Closing Category Rank
                <span>
                    ?
                    <small class="hidden"> Upper limit of Rank. Results are shown in Descending order lesser than the input. </small>
                </span>
            </label>

            <input
                class="input input-bordered"
                name="rank"
                placeholder="Eg. 14000"
                type="number"
                bind:value={cr}
            />
        </div>

        <div class="flex flex-col">
            <label for="year">Year</label>

            <select class="select select-bordered" name="year" bind:value={year} multiple>
                {#each [2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024] as year}
                    <option value={year}> {year} </option>
                {/each}
            </select>
        </div>

        <div class="flex flex-col">
            <label for="round">Round</label>
            <select class="select select-bordered" name="round" bind:value={round} multiple>
                {#each [1, 2, 3, 4, 5, 6] as r}
                    <option value={r}> {r} </option>
                {/each}
            </select>
        </div>

        <button type="submit" class="btn bg-base-300"> Query </button>
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
