<script>
    export /**@type {any[][]}*/ let records;

    let rows = records.length ? records : [{ error: "No Matches" }];
    
    let external = ["college_name"]; //Column name for google hyperlink
</script>

<div>
    <span style="margin-left: 2rem;"> Showing {rows.length} Records </span>
    
    <table>
        <thead>
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
                                                  x.substring(1).toLowerCase(),
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

<style>
    table {
        border-collapse: collapse;
        margin: 0 2rem;
    }
    table th {
        padding: 0 0.5rem;
    }
    table td {
        border: 1px solid white;
        padding: 0.5rem 0.2rem;
    }
    table tr:nth-child(odd) {
        background: #ffffff11;
    }
</style>
