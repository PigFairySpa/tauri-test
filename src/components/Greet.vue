<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");


const greetMsg1 = ref("");
const name1 = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}

async function greet1() {
  greetMsg1.value = await invoke("greet1", {name: name1.value})
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <form class="row" @submit.prevent="greet1">
    <input id="greet-input" v-model="name1" placeholder="Enter a name..."/>
    <button type="submit">Greet1</button>
  </form>

  <p>{{ greetMsg1 }}</p>
</template>
