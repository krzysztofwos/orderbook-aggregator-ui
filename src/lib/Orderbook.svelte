<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  let bids = [];
  let asks = [];
  let spread = 0.0;
  let unlisten;
  let pretty_exchange = {
    binance: "Binance",
    bitstamp: "Bitstamp",
  };

  onMount(async () => {
    unlisten = await listen("summary", (event) => {
      spread = event.payload["spread"];
      bids = event.payload["bids"];
      asks = event.payload["asks"].reverse();
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });
</script>

<table>
  {#each asks as ask}
    <tr>
      <td width="100px" align="right">{ask.amount.toFixed(6)}</td>
      <td width="100px" align="right">{ask.price.toFixed(2)}</td>
      <td width="100px" align="left">
        {pretty_exchange[ask.exchange] || ask.exchange}
      </td>
    </tr>
  {/each}
  <tr>
    <td width="100px" align="right" />
    <td width="100px" align="right">{spread.toFixed(2)}</td>
    <td width="100px" align="left" />
  </tr>
  {#each bids as bid}
    <tr>
      <td width="100px" align="right">{bid.amount.toFixed(6)}</td>
      <td width="100px" align="right">{bid.price.toFixed(2)}</td>
      <td width="100px" align="left">
        {pretty_exchange[bid.exchange] || bid.exchange}
      </td>
    </tr>
  {/each}
</table>
