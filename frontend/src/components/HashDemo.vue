<script setup lang="ts">
import { ref } from 'vue';

const file = ref<File | null>(null);
const hashHex = ref<string>('');
const title = ref<string>('');
const author = ref<string>('');
const latest = ref<any>(null);

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

async function registerPaper() {
  if (!hashHex.value) return alert('先にファイル選択してハッシュを計算してね');
  const res = await fetch('/api/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      title: title.value || file.value?.name || 'untitled',
      author: author.value || 'anonymous',
      hashHex: hashHex.value,
    }),
  });
  if (!res.ok) return alert('登録に失敗しました');
  const json = await res.json();
  alert('登録完了: ' + json.id);
}

async function fetchLatest() {
  const res = await fetch('/api/latest');
  if (!res.ok) { latest.value = null; return; }
  latest.value = await res.json();
}
</script>

<template>
  <div style="padding:16px;">
    <h2>ResearchChain – MVP</h2>

    <div style="margin:8px 0;">
      <input type="file" @change="onFileChange" />
    </div>

    <div style="margin:8px 0;">
      <label>Title:</label>
      <input v-model="title" placeholder="paper title" />
    </div>

    <div style="margin:8px 0;">
      <label>Author:</label>
      <input v-model="author" placeholder="author" />
    </div>

    <p v-if="file">File: {{ file.name }}</p>
    <p v-if="hashHex"><b>SHA-256:</b> {{ hashHex }}</p>

    <div style="margin:12px 0;">
      <button @click="registerPaper">登録（MVP: ローカル保存）</button>
      <button @click="fetchLatest" style="margin-left:8px;">最新取得</button>
    </div>

    <div v-if="latest" style="margin-top:12px;">
      <h3>Latest</h3>
      <pre>{{ latest }}</pre>
    </div>
  </div>
</template>
