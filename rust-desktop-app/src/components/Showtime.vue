<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

let timer_message = ref("Timer will start soon...");
let time = ref("");

async function timer() {
  timer_message.value = "Timer started"
  timer_message.value = await invoke("timer", { time: time.value });
}
</script>

<template>
<div class="container">
  <form class="row" @submit.prevent="timer">
    <input id="time" v-model="time" placeholder="Enter time in seconds..." />
    <button type="submit">Start Timer</button>
    <p>{{ timer_message }}</p>
  </form>
</div>
</template>

<style scoped>

</style>