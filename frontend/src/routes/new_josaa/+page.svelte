<script>
    import List from "$lib/List.svelte";
    import "$lib/dashcss.css";

    let /**@type {string[]}*/ institute = [],
        /**@type {string[]}*/ course = [],
        /**@type {"HS"|"OS"|"AI" []}*/ quota = [],
        /**@type {"OPEN"|"EWS"|"OBC-NCL"|"SC"|"ST"|"PwD"|""}*/ seat = "",
        /**@type {"neutral"|"female"|""}*/ gender = "",
        /**@type {number|null}*/ cr = null,
        /**@type {2016|2017|2018|2019|2020|2021|2022|2023 []}*/ year = [],
        /**@type {1|2|3|4|5|6 []}*/ round = [],
        /**@type {Promise<any[]> | null}*/ results = null;

    let /**@type {string}*/ _institute, /**@type {string}*/ _course;

    function addInstitute() {
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

<svelte:head>
    <title>JOSAA Counselling</title>
    <meta name="title" content="JOSAA Counselling Query" />
    <meta name="description" content="Query the TNEA Counselling results (2016-23) for aid in decision of your counselling list." />
</svelte:head>

<div class="container">
    <h1> JOSAA Counselling </h1>

    <form class="container centered" style="max-width: 32rem; width: 100%;" on:submit|preventDefault={handleSubmit}>
        <div style="text-align: left;">
            <label for="institute">
                Institute
                <span>
                    ?
                    <small class="tooltip"> Add institute names. Avoid abbreviations like NIT, IIT. A word like 'National' would do. </small>
                </span>
            </label>

            <div>
                <ul class="flex-row wrap">
                    {#each institute as name, i}
                        <button
                            on:click={() => removeInstitute(i)}
                            class="outline"
                            style="padding: .25rem;"
                        >
                            {name} x
                        </button>
                    {/each}
                </ul>
                <form class="group" on:submit|preventDefault={addInstitute}>
                    <input
                        type="text"
                        bind:value={_institute}
                        placeholder="name"
                    />
                    <button type="submit" disabled={!_institute}> Add </button>
                </form>
            </div>
        </div>

        <div style="text-align: left;">
            <label for="course">
                Course
                <span>
                    ?
                    <small class="tooltip"> Add courses. Just a word like 'Computer' would do.  Avoid abbreviations like CS. </small>
                </span>
            </label>
            <div>
                <ul class="flex-row">
                    {#each course as name, i}
                        <button
                            on:click={() => removeCourse(i)}
                            class="outline small"
                        >
                            {name} x
                        </button>
                    {/each}
                </ul>
                <form class="group" on:submit|preventDefault={addCourse}>
                    <input
                        type="text"
                        bind:value={_course}
                        placeholder="course"
                    />
                    <button type="submit" disabled={!_course}> Add </button>
                </form>
            </div>
        </div>

        <div style="text-align: left;" class="flex-col">
            <label for="quota">
                Quota
                <span>
                    ?
                    <small class="tooltip"> Select quota. If a particular inst. is selected in name of the inst., then select HS or OS. If not leave this. </small>
                </span>
            </label>

            <select name="quota" bind:value={quota} multiple>
                <option value="HS"> HS (Home State) </option>
                <option value="OS"> OS (Other State) </option>
                <option value="AI"> AI (All India - for IITs and IIITs) </option>
            </select>
        </div>
        
        <div style="text-align: left;" class="flex-col">
            <label for="seat"> Select Seat Type </label>

            <select name="seat" bind:value={seat}>
                <option value="" selected> Any </option>
                <option value="OPEN"> OPEN </option>
                <option value="EWS"> EWS </option>
                <option value="OBC-NCL"> OBC-NCL </option>
                <option value="SC"> SC </option>
                <option value="ST"> ST </option>
                <option value="PwD"> PwD (OPEN, EWS, OBC-NCL, SC, ST) </option>
            </select>
        </div>

        <div style="text-align: left;" class="flex-col">
            <label for="gender">
                Select Gender
            </label>

            <select name="gender" bind:value={gender}>
                <option value="" selected> Any </option>
                <option value="neutral"> Neutral </option>
                <option value="female"> Female Only (+Supernumerary) </option>
            </select>
        </div>

        <div style="text-align: left;" class="flex-col">
            <label for="rank">
                Closing Rank
                <span>
                    ?
                    <small class="tooltip"> Upper limit of Rank. Results are shown in Descending order lesser than the input. </small>
                </span>
            </label>

            <input
                name="rank"
                placeholder="Eg. 14000"
                type="number"
                bind:value={cr}
            />
        </div>

        <div style="text-align: left;" class="flex-col">
            <label for="year">Year</label>

            <select name="year" bind:value={year} multiple>
                {#each [2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023] as year}
                    <option value={year}> {year} </option>
                {/each}
            </select>
        </div>

        <div style="text-align: left;" class="flex-col">
            <label for="round">Round</label>
            <select name="round" bind:value={round} multiple>
                {#each [1, 2, 3, 4, 5, 6] as r}
                    <option value={r}> {r} </option>
                {/each}
            </select>
        </div>

        <button type="submit" class="centered"> Query </button>
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
