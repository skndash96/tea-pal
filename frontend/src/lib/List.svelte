<script>
    export /**@type {any[][]}*/ let records;

    let rows = records.length ? records : [{ error: "No Matches" }];

    let external = ["college_name"]; //Column name for google hyperlink
</script>

<div>
    <span style="margin-left: 2rem;"> Showing {rows.length} Records </span>

    <div class="w-full overflow-x-auto">
        <table class="table table-zebra table-pin-rows">
            <thead class="table-header-group">
                {#each Object.keys(rows[0]) as field}
                    <th> {field} </th>
                {/each}
            </thead>

            <tbody>
                {#each rows as row}
                    <tr>
                        {#each Object.entries(row) as [field, val]}
                            <td>
                                {#if external.includes(field)}
                                    <a
                                        href="https://www.google.com/?q={val}"
                                        target="_blank"
                                    >
                                        {val}
                                    </a>
                                {:else}
                                    {field === "branch_name"
                                        ? val.split(" ").shift() +
                                          " " +
                                          val
                                              .split(" ")
                                              .slice(1)
                                              .map(
                                                  (/**@type {string}*/ x) =>
                                                      x[0] +
                                                      x
                                                          .substring(1)
                                                          .toLowerCase(),
                                              )
                                              .join(" ")
                                        : val}
                                {/if}
                            </td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</div>

<style>
    div.table {
        width: 100%;
        overflow-x: scroll;
    }
</style>
