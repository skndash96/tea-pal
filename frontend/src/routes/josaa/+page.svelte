<script>
    import List from "$lib/List.svelte";

    /**
     * query by
     * name
     * course
     * quota (HS OS AI GO JK LA AP)
     * seat (OPEN, EWS, OBC-NCL, SC, ST, PwD)
     * gender (Neutral, FemaleOnly(+Supernumerary), NA)
     * rank (lower bound rank)
     * year 2016 to 2023
     * round
     */
    let
        /**@type {string|null}*/institute,
        /**@type {string|null}*/course,
        /**@type {"HS"|"OS"|"AI"|null}*/quota, //no support for other quotas
        /**@type {"OPEN"|"EWS"|"OBC-NCL"|"SC"|"ST"|"PwD"|null}*/seat,
        /**@type {"neutral"|"female"|null}*/gender,
        /**@type {number|null}*/rank,
        /**@type {2016|2017|2018|2019|2020|2021|2022|null}*/year,
        /**@type {1|2|3|4|5|6|null}*/round,
        /**@type {Promise<any[]> | null}*/results;

    const URL_PREFIX = "/josaa?";

    /**@param {SubmitEvent} e */
    function handleSubmit(e) {
        let url = URL_PREFIX
            + query_sub_str("name", institute)
            + query_sub_str("course", course)
            + query_sub_str("quote", quota)
            + query_sub_str("seat", seat)
            + query_sub_str("gender", gender)
            + query_sub_str("rank", rank)
            + query_sub_str("year", year)
            + query_sub_str("round", round);
        
        results = new Promise(async (resolve, reject) => {
            try {
                let res = await fetch(url);
                let json = await res.json();

                resolve(json);
            } catch (e) {
                reject(e);
            }
        });
    }

    /**
     * @param {string|null} prop
     * @param {string|number|null} val
     * @returns {string}
     */
    function query_sub_str(prop, val) {
        if (prop && val) {
            return `${prop}=${val}&`;
        } else {
            return "";
        }
    }
</script>

<div class="container">
    <h2>JOSAA Counselling Query</h2>

    <form on:submit|preventDefault={handleSubmit}>
        <div>
            <label for="institute">
                Name of institute
                <span class="tooltip">
                    <span>?</span>
                    <span>
                        Don't use abbreviations like NIT, IIT. Use words even
                        just one like National to describe NIT.
                    </span>
                </span>
            </label>

            <input
                name="institute"
                placeholder="Eg: Birla, Indian Institute"
                type="text"
                bind:value={institute}
            />
        </div>

        <div>
            <label for="course">
                Filter courses
                <span class="tooltip">
                    <span>?</span>
                    <span>
                        Don't use abbreviations like BE, CSE. Use words even
                        just one like Computer to describe CSE course.
                    </span>
                </span>
            </label>

            <input
                name="course"
                placeholder="Eg: Mechanical, Civil"
                type="text"
                bind:value={course}
            />
        </div>

        <div id="quota">
            <label for="quota">
                Select Quota Type

                <span class="tooltip">
                    <span>?</span>
                    <span>
                        HS, OS, AI <small>(All India)</small>, AP
                        <small>(NIT Warangal)</small>, LA
                        <small>(NIT Srinagar)</small>, JK
                        <small>(NIT Srinagar)</small>, GO
                        <small>(NIT Goa)</small>
                    </span>
                </span>
            </label>

            <select name="quota" bind:value={quota}>
                <option value="" selected> Any </option>
                <option value="HS"> HS (Home State) </option>
                <option value="OS"> OS (Other State) </option>
                <option value="AI">
                    AI (All India - for IITs and IIITs)
                </option>
            </select>
        </div>

        <div>
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

        <div>
            <label for="gender"> Select Gender </label>
            <select name="gender" bind:value={gender}>
                <option value="" selected> Any </option>
                <option value="neutral"> Neutral </option>
                <option value="female"> Female Only (+Supernumerary) </option>
            </select>
        </div>

        <div>
            <label for="rank"> Opening Rank </label>
            <div>
                <input
                    name="rank"
                    placeholder="Eg. 14000"
                    type="number"
                    bind:value={rank}
                />
            </div>
        </div>

        <div>
            <label for="year">Year</label>
            <select name="year" bind:value={year}>
                {#each [null, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023] as year}
                    <option value={year}> {year || "Any"} </option>
                {/each}
            </select>
        </div>

        <div>
            <label for="round">Round</label>
            <select name="round" bind:value={round}>
                {#each [null, 1, 2, 3, 4, 5, 6] as r}
                    <option value={r}> {r || "Any"} </option>
                {/each}
            </select>
        </div>

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
        padding: 2rem;
    }

    h2 {
        text-align: center;
    }

    form {
        margin: 5rem auto;
        width: fit-content;
    }
    form > div {
        padding: 1rem 0 0 0;
        display: grid;
        grid-template-columns: 1fr 2fr;
    }
    form > div > *:first-child {
        justify-self: end;
        margin-right: 1rem;
    }
    form input,
    form select {
        background-color: #ffffffaa;
    }

    span.tooltip {
        position: relative;
    }
    span.tooltip span:first-child {
        padding: 0.2rem;
        color: khaki;
        position: relative;
        cursor: help;
    }
    span.tooltip span:last-child {
        position: absolute;
        top: 0;
        right: 0;
        transform: translate(100%, -100%);
        background: slateblue;
        opacity: 0;
        width: 20rem;
        z-index: -1;
    }
    span.tooltip span:first-child:hover + span {
        opacity: 1;
        z-index: 2;
    }

    form button {
        background: khaki;
        width: fit-content;
        margin: 2rem auto;
        display: block;
    }

    small {
        font-size: 0.7rem;
        opacity: 0.7;
        margin: 0 0.1rem;
    }
</style>
