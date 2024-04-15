<script>
    import List from "$lib/List.svelte";
    import "$lib/dashcss.css";

    let /**@type {string[]}*/ institute = [],
        /**@type {string[]}*/ course = [],
        /**@type {"HS"|"OS"|"AI" []}*/ quota = [],
        /**@type {"OPEN"|"EWS"|"OBC-NCL"|"SC"|"ST"|"PwD"|""}*/ seat = "",
        /**@type {"neutral"|"female"|""}*/ gender = "",
        /**@type {number|null}*/ rank = null,
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
            + get_sub_str("rank", rank)
            + get_sub_str("year", year)
            + get_sub_str("round", round);
        alert(url);

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

<div class="container">
    <h1> JOSAA Counselling </h1>

    <form class="container centered" style="max-width: 40rem;" on:submit|preventDefault={handleSubmit}>
        <div>
            <label for="institute">Institute</label>

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

        <div>
            <label for="course">Course</label>
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

        <div class="flex-col">
            <label for="quota">
                Quota
            </label>
            
            <select name="quota" bind:value={quota} multiple>
                <option value="HS"> HS (Home State) </option>
                <option value="OS"> OS (Other State) </option>
                <option value="AI"> AI (All India - for IITs and IIITs) </option>
            </select>
        </div>
        
        <div class="flex-col">
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

        <div class="flex-col">
            <label for="gender"> Select Gender </label>

            <select name="gender" bind:value={gender}>
                <option value="" selected> Any </option>
                <option value="neutral"> Neutral </option>
                <option value="female"> Female Only (+Supernumerary) </option>
            </select>
        </div>

        <div class="flex-col">
            <label for="rank"> Opening Rank </label>

            <input
                name="rank"
                placeholder="Eg. 14000"
                type="number"
                bind:value={rank}
            />
        </div>

        <div class="flex-col">
            <label for="year">Year</label>

            <select name="year" bind:value={year} multiple>
                {#each [2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023] as year}
                    <option value={year}> {year} </option>
                {/each}
            </select>
        </div>

        <div class="flex-col">
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
