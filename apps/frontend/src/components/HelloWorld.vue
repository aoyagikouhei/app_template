<script setup lang="ts">
import { ref } from 'vue'
import { createLoginClientContext } from '../api/loginClient/loginClientContext'
import { login as loginOperation } from '../api/loginClient/loginClientOperations'
import { getClientOptionsWithCredentials } from '../utils/credentialsPolicy'

defineProps<{ msg: string }>()

const count = ref(0)

const login = async () => {
  try {
    console.log('Login button clicked')
    const clientOptions = getClientOptionsWithCredentials({
      allowInsecureConnection: true
    })
    const client = createLoginClientContext("http://localhost:3000/", clientOptions)
    const result = await loginOperation(client)
    console.log('Login result:', result)
    
    if (result.data?.url) {
      window.location.href = result.data.url
    }
  } catch (error) {
    console.error('Login error:', error)
  }
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <div class="login-container">
    <button type="button" @click="login" class="login-button">ログイン</button>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Learn more about IDE Support for Vue in the
    <a
      href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support"
      target="_blank"
      >Vue Docs Scaling up Guide</a
    >.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}

.login-container {
  margin: 20px 0;
}

.login-button {
  background-color: #4CAF50;
  color: white;
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

.login-button:hover {
  background-color: #45a049;
}
</style>
