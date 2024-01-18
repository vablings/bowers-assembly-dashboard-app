<script lang="ts">
  import MetricTable from './lib/MetricTable.svelte'
  import ImperialTable from './lib/ImperialTable.svelte'
</script>

<div id="container">
  <div><MetricTable /> </div>
  <div><ImperialTable /> </div>
</div>


<style>
#container {
  display: flex;                  /* establish flex container */
  flex-direction: row;            /* default value; can be omitted */
  flex-wrap: nowrap;              /* default value; can be omitted */
  justify-content: space-between; /* switched from default (flex-start, see below) */
}
</style>