<template>
  <div class="oauth-callback">
    <p v-if="loading">Processing OAuth callback...</p>
    <p v-if="error" class="error">{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { TemplateServiceClient } from '../templateServiceClient'
import type { OAuthResult } from '../models/models'
import { getClientOptionsWithCredentials } from '../utils/credentialsPolicy'

const router = useRouter()
const route = useRoute()
const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    const code = route.query.code as string
    const state = route.query.state as string

    if (!code || !state) {
      throw new Error('Missing required OAuth parameters')
    }

    const oauthResult: OAuthResult = {
      code,
      state
    }

    const clientOptions = getClientOptionsWithCredentials({
      allowInsecureConnection: true
    })
    const client = new TemplateServiceClient("http://localhost:3000/", clientOptions)
    const result = await client.loginClient.oauth(oauthResult)
    
    if (result.resultCode === 'success') {
      await router.push('/')
    } else {
      throw new Error(`OAuth failed with result code: ${result.resultCode}`)
    }
  } catch (err) {
    console.error('OAuth callback error:', err)
    error.value = err instanceof Error ? err.message : 'An error occurred during OAuth callback'
    loading.value = false
  }
})
</script>

<style scoped>
.oauth-callback {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 50vh;
}

.error {
  color: red;
  margin-top: 1rem;
}
</style>