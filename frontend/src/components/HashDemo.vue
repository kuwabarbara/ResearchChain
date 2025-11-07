<script setup lang="ts">
import { ref } from 'vue';

const file = ref<File | null>(null);
const hashHex = ref<string>('');

async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', bytes);
  const arr = new Uint8Array(digest);
  return Array.from(arr).map(b => b.toString(16).padStart(2, '0')).join('');
}

async function onFileChange(e: Event) {
  const input = e.target as HTMLInputElement;
  const f = input.files?.[0] || null;
  file.value = f;
  hashHex.value = '';
  if (!f) return;
  const buf = new Uint8Array(await f.arrayBuffer());
  hashHex.value = await sha256Hex(buf);
}
</script>

<template>
  <div style="padding:16px;">
    <h2>SHA-256 Hash Demo</h2>
    <input type="file" @change="onFileChange" />
    <p v-if="file">File: {{ file.name }}</p>
    <p v-if="hashHex">SHA-256: {{ hashHex }}</p>
  </div>
</template>
