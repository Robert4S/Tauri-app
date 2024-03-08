<script setup lang="ts">
import { getClient, Body, ResponseType } from '@tauri-apps/api/http';

import { ref } from 'vue';
let city = ref('');
let temperature = ref('');
let mood = ref('');
let question = ref('');
let output = ref('');

async function post_data() {
  console.log("This is to check if the function gets here 1");
  const data = {
    city: city.value,
    temperature: temperature.value,
    response: mood.value,
    question: question.value
  };
  console.log(data);
  const client = await getClient();
  const response = await client.post('http://localhost:8000/api/response_creator/', {
    body: Body.json(data),
    headers: {
      'Content-Type': 'application/json'
    }
  }).catch((e) => {
    console.error(e);
  });
  console.log(response);
  output.value = response.body;
  console.log("This is to check if the function gets here");
}



</script>

<template>
  <div>
    <h1>Post Testing</h1>
    <form class="row" @submit.prevent="post_data">
      <input v-model="city" placeholder="City" />
      <input v-model="temperature" placeholder="Temperature" />
      <input v-model="mood" placeholder="Mood" />
      <input v-model="question" placeholder="Question" />
      <button type="submit" class="tlui-button">Post</button>
      <p>{{ output }}</p>
    </form>
  </div>
</template>

<style scoped>

</style>