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
        /**@type {Promise<any[]> | any[] | null}*/ results = [{"name":"Indian Institute of Technology Bhubaneswar","course":"Civil Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":9193,"cr":11771,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Civil Engineering and M. Tech. in Structural Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":12316,"cr":12489,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Civil Engineering and M.Tech in Transportation Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":12668,"cr":12852,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Civil Engineering and M.Tech. in Environmental Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":11628,"cr":12500,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Computer Science and Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1702,"cr":2486,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Computer Science and Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":2821,"cr":3017,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Electrical Engineering and M.Tech Power Electronics and Drives (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":6355,"cr":6789,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Electrical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":4127,"cr":6257,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Electronics and Communication Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":3084,"cr":4110,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Electronics and Communication Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":178,"cr":178,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Mechanical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":4245,"cr":8494,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Mechanical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":0,"cr":0,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Mechanical Engineering and M. Tech. in Mechanical System Design (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":8741,"cr":9322,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Mechanical Engineering and M. Tech. in Thermal Science & Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":8812,"cr":9727,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Mechanical Engineering with M.Tech. in Manufacturing Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":9634,"cr":9963,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Metallurgical and Materials Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":9407,"cr":12271,"year":2022,"round":1},{"name":"Indian Institute of Technology Bhubaneswar","course":"Metallurgical and Materials Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":11753,"cr":12759,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Aerospace Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":577,"cr":2119,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Aerospace Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":164,"cr":164,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"BS in Mathematics (4 Years Bachelor of Science)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":552,"cr":1557,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Chemical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1420,"cr":2081,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Chemical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":94,"cr":137,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Chemistry (4 Years Bachelor of Science)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1664,"cr":5323,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Civil Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1941,"cr":3418,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Civil Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":141,"cr":156,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Computer Science and Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1,"cr":60,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Computer Science and Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":2,"cr":5,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Economics (4 Years Bachelor of Science)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1577,"cr":2107,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Economics (4 Years Bachelor of Science)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":61,"cr":61,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Electrical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":103,"cr":369,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Electrical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":38,"cr":38,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Electrical Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":374,"cr":843,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Electrical Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":114,"cr":114,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Energy Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":1116,"cr":2381,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Energy Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":0,"cr":0,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Engineering Physics (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":39,"cr":1397,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Engineering Physics (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":0,"cr":0,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Environmental Science and Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":3926,"cr":4736,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Environmental Science and Engineering (5 Years Bachelor and Master of Technology (Dual Degree))","quota":"AI","seat":"OPEN (PwD)","gender":"Gender-Neutral","or":0,"cr":0,"year":2022,"round":1},{"name":"Indian Institute of Technology Bombay","course":"Mechanical Engineering (4 Years Bachelor of Technology)","quota":"AI","seat":"OPEN","gender":"Gender-Neutral","or":141,"cr":1382,"year":2022,"round":1}];

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

    <form class="p-4 flex flex-col gap-4" on:submit|preventDefault={handleSubmit}>
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
