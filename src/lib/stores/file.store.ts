import { writable } from "svelte/store";

type CurrentFile = {
  name: string;
  data: string;
};

export const currentFile = writable<CurrentFile>({
  name: "Initiation Script",
  data: `it's not a lake it's an **ocean**.
## Saga
**Saga** was back at Cauldron Lake. He was there too. Nightingale. Was, but wasn't. A Taken. A creature of darkness. He was beyond her reach. Where some other strange reality, the Dark Place, merged with ours. This place and the Dark Place. A tarp thrown over top. Drowning everything beneath it. A flood of darkness. Soaking into everything. Spoiling it. Rotting it. The page called this area an Overlap. Saga had to purse
Nightingale into the Overlap. Finding a way in would be difficult. Required precise steps. A ritual. Saga would
learn how. Stop the monster. Her job. Before he killed again. He'd be inside. Waiting for her.
`,
});
const unsubscribe = currentFile.subscribe((value) => {
  // api will be called from here to save file updates
  console.log("Store updated");
});
// unsubscribe() to stop listening to updates
