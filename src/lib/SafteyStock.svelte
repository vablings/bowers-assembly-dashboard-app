<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  type MaterialStockData = {
    material: string,
    description: string,
    stock: number,
    safety: number
  }
  
  let msd: MaterialStockData[] = [];
  let keys: string[] = [];


  async function loadData() { 
    msd = JSON.parse(await invoke("fetch")) as MaterialStockData[]
    keys = Object.keys(msd[0])
  }
  
  function getPercentage(u: number, l: number) { 
    let pcr = (u/l * 100); 
    if (pcr > 100) { 
      return 100;
    } else if (pcr < 2) { 
      return 2;
    } else {
      return pcr;
    }
  }

  function perc2color(perc: number) {
      var r, g, b = 0;
      if(perc < 50) {
        r = 255;
        g = Math.round(5.1 * perc);
      }
      else {
        g = 255;
        r = Math.round(510 - 5.10 * perc);
      }
      var h = r * 0x10000 + g * 0x100 + b * 0x1;
      return '#' + ('000000' + h.toString(16)).slice(-6);
  }
  
  let tableBody: HTMLElement | undefined = undefined
  let lastScrollPosition: number = 0
  let direction = 1;


	onMount(async () => {
    loadData()
    setInterval(loadData, 300000);
    setInterval(() => {
            if (tableBody) {
                const scrollPosition = tableBody.scrollTop;
                const rowHeight = (tableBody.firstChild as HTMLElement).clientHeight;
                if (scrollPosition == lastScrollPosition) direction = direction * -1;
                lastScrollPosition = scrollPosition;
                tableBody.scroll({top: lastScrollPosition + rowHeight * direction, behavior: "smooth"})
            }
        }, 1000);
	});
</script>


<body>
  <table class="styled-table" id="table" style="overflow-x:scroll">
      <thead>
      <tr>
          {#each keys as header}
              <th>{header}</th>
          {/each}
          <th style="padding-right: 150px">saftey stock %</th>
      </tr>
      </thead>
      <tbody bind:this={tableBody} style="display: block; max-height: 720px; overflow-y: scroll;">
      {#each msd as row}
          <tr>
              <td>{row.material}</td>
              <td>{row.description}</td>
              <td>{row.stock}</td>
              <td>{row.safety}</td>
              <td>
                  <progress-meter>
                      <progress-percent style="--progress: {getPercentage(row.stock, row.safety)}; background-color:{perc2color(getPercentage(row.stock, row.safety))}"></progress-percent>
                  </progress-meter>
              </td>
          </tr>
      {/each}
      </tbody>
  </table>
  </body>
  
  
  <style>
      progress-meter {
          display: block;
          height: 10px;
          box-shadow: 0px 0px 6px 2px rgba(0, 0, 0, 0.2);
          border-radius: 5px;
          padding: 1px;
          margin-right: 2rem;
          position: relative;
      }
  
      progress-percent {
          display: block;
          height: 100%;
          border-radius: 5px;
          width: calc(var(--progress) * 1%);
      }
  
      .styled-table {
          border-collapse: collapse;
          margin: 25px 0;
          font-size: 0.9em;
          font-family: sans-serif;
          min-width: 400px;
          box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
      }
  
      .styled-table thead tr {
          background-color: #009879;
          color: #ffffff;
          text-align: left;
      }
  
      .styled-table th,
      .styled-table td {
          padding: 12px 15px;
      }
  
      .styled-table tr {
          display: table;
          border-bottom: 1px solid #dddddd;
          table-layout: fixed;
          width: 100%;
      }
  
      .styled-table tbody tr:nth-of-type(even) {
          background-color: #f3f3f3;
      }
  </style>