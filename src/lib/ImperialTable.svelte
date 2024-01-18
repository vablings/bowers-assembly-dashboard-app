<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  type MaterialStockData = {
    material: string;
    description: string;
    stock: number;
    safety: number;
  };

  let msd: MaterialStockData[] = [];
  let keys: string[] = [];

  function compareHeadSize(a: MaterialStockData, b: MaterialStockData) { 
    let a_size = Number(a.material.split("-")[2]);
    let b_size = Number(b.material.split("-")[2]);
    console.log("dddcomparing a = %s vs b = %s", a_size, b_size);
    if (a_size < b_size) {
        return -1;
      }
      if (a_size > b_size) {
        return 1;
      }
      return 0;
  }

  async function loadData() {
    msd = (JSON.parse(await invoke("fetch")) as MaterialStockData[]).filter(e => e.description.includes("\""));
    msd.sort(compareHeadSize)
    keys = Object.keys(msd[0]);
  }

  function getPercentage(u: number, l: number) {
    let pcr = (u / l) * 100;
    if (pcr > 100) {
      return 100;
    } else if (pcr < 2) {
      return 2;
    } else {
      return pcr;
    }
  }

  function perc2color(perc: number) {
    var r,
      g,
      b = 0;
    if (perc < 50) {
      r = 255;
      g = Math.round(5.1 * perc);
    } else {
      g = 255;
      r = Math.round(510 - 5.1 * perc);
    }
    var h = r * 0x10000 + g * 0x100 + b * 0x1;
    return "#" + ("000000" + h.toString(16)).slice(-6);
  }

  let tableBody: HTMLElement | undefined = undefined;
  let lastScrollPosition: number = 0;
  let direction = 1;

  onMount(async () => {
    loadData();
    setInterval(loadData, 300000);
  });
</script>

<body>
  <table class="styled-table" id="table" style="overflow-x:scroll">
    <thead>
      <tr>
        {#each keys as header}
          <th>{header}</th>
        {/each}
        <th style="padding-right: 100px">saftey stock %</th>
      </tr>
    </thead>
    <tbody bind:this={tableBody}>
      {#each msd as row}
        <tr>
          <td>{row.material}</td>
          <td style="width: 23%;">{row.description}</td>
          <td>{row.stock}</td>
          <td>{row.safety}</td>
          <td>
            <progress-meter>
              <progress-percent
                style="--progress: {getPercentage(
                  row.stock,
                  row.safety,
                )}; background-color:{perc2color(
                  getPercentage(row.stock, row.safety),
                )}"
              ></progress-percent>
            </progress-meter>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</body>

<style>
  @import './tableview.css';
</style>
