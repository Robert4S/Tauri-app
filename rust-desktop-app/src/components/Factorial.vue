<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
const input = ref("");
const out_fact = ref("");

async function factorial() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  out_fact.value = await invoke("intermediate_factorial", { n: input.value});

}
</script>

<template>
  <form class="row" @submit.prevent="factorial">
    <input id="input" v-model="input" placeholder="Enter a number..." />
    <button type="submit">Factorial</button>
  </form>

  <p>{{ out_fact }}</p>

</template>