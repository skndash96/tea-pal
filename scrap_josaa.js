let ROUND = 6;
const YEAR = 2023;

let tr = document.querySelectorAll("tr");
let rows = Array.from(tr).map(row/*tr*/ => {
    return Array.from(row.children).map(cell/*td*/ => {
        let txt = cell.textContent.trim();
        return parseInt(txt) || txt.replaceAll(",", "");
    }).concat(YEAR, ROUND);
});

rows = rows.slice(1).map(cells => cells.join(","));
console.log(rows.join("\n"));