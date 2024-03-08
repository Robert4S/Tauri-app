<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
const inputx = ref("");
const inputy = ref("");
const outsum = ref("");

async function add_values() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  outsum.value = await invoke("calculate_sum", { x: inputx.value, y: inputy.value });

}
</script>

<template>
  <form class="row" @submit.prevent="add_values">
    <input id="inputx" v-model="inputx" placeholder="Enter a number..." />
    <input id="inputy" v-model="inputy" placeholder="Enter a number..." />
    <button type="submit">Add</button>
  </form>

  <p>{{ outsum }}</p>

</template>
