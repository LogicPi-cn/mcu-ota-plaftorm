<template>
  <div class="login-container">
    <div class="login-box">
      <div class="login-header">
        <h1 class="login-title">MCU OTA 平台</h1>
        <p class="login-subtitle">{{ isRegister ? '创建新账号' : '请输入账号密码登录' }}</p>
      </div>

      <!-- 登录/注册切换 -->
      <div class="mode-switch">
        <a-radio-group v-model="isRegister" type="button" long>
          <a-radio :value="false">登录</a-radio>
          <a-radio :value="true">注册</a-radio>
        </a-radio-group>
      </div>

      <!-- 登录表单 -->
      <a-form
        v-if="!isRegister"
        ref="loginFormRef"
        :model="loginFormData"
        :rules="loginFormRules"
        layout="vertical"
        @submit="handleLogin"
      >
        <a-form-item field="email" label="邮箱">
          <a-input
            v-model="loginFormData.email"
            placeholder="请输入邮箱"
            size="large"
          >
            <template #prefix>
              <icon-email />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item field="password" label="密码">
          <a-input-password
            v-model="loginFormData.password"
            placeholder="请输入密码"
            size="large"
          >
            <template #prefix>
              <icon-lock />
            </template>
          </a-input-password>
        </a-form-item>

        <a-form-item>
          <a-button
            type="primary"
            html-type="submit"
            size="large"
            long
            :loading="loginLoading"
          >
            登录
          </a-button>
        </a-form-item>
      </a-form>

      <!-- 注册表单 -->
      <a-form
        v-else
        ref="registerFormRef"
        :model="registerFormData"
        :rules="registerFormRules"
        layout="vertical"
        @submit="handleRegister"
      >
        <a-form-item field="name" label="用户名">
          <a-input
            v-model="registerFormData.name"
            placeholder="请输入用户名"
            size="large"
          >
            <template #prefix>
              <icon-user />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item field="email" label="邮箱">
          <a-input
            v-model="registerFormData.email"
            placeholder="请输入邮箱"
            size="large"
          >
            <template #prefix>
              <icon-email />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item field="password" label="密码">
          <a-input-password
            v-model="registerFormData.password"
            placeholder="请输入密码（至少 6 位）"
            size="large"
          >
            <template #prefix>
              <icon-lock />
            </template>
          </a-input-password>
        </a-form-item>

        <a-form-item>
          <a-button
            type="primary"
            html-type="submit"
            size="large"
            long
            :loading="registerLoading"
          >
            注册
          </a-button>
        </a-form-item>
      </a-form>

      <a-alert v-if="error" :type="errorType" style="margin-top: 16px">
        {{ error }}
      </a-alert>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/user';
import {
  IconEmail,
  IconLock,
  IconUser,
} from '@arco-design/web-vue/es/icon';
import type { FormInstance } from '@arco-design/web-vue';
import { register as apiRegister } from '@/api/auth';

const router = useRouter();
const userStore = useUserStore();

const isRegister = ref(false);
const loginFormRef = ref<FormInstance>();
const registerFormRef = ref<FormInstance>();
const loginLoading = ref(false);
const registerLoading = ref(false);
const error = ref('');
const errorType = ref<'error' | 'success'>('error');

const loginFormData = reactive({
  email: '',
  password: '',
});

const registerFormData = reactive({
  name: '',
  email: '',
  password: '',
});

const loginFormRules = {
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { minLength: 6, message: '密码长度至少 6 位', trigger: 'blur' },
  ],
};

const registerFormRules = {
  name: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { minLength: 2, message: '用户名长度至少 2 位', trigger: 'blur' },
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { minLength: 6, message: '密码长度至少 6 位', trigger: 'blur' },
  ],
};

const handleLogin = async () => {
  error.value = '';
  errorType.value = 'error';
  const valid = await loginFormRef.value?.validate();
  if (valid) return;

  loginLoading.value = true;

  try {
    await userStore.login({
      email: loginFormData.email,
      password: loginFormData.password,
    });

    // 登录成功后获取用户信息
    await userStore.fetchUserInfo();

    router.push('/');
  } catch (err: any) {
    error.value = err.response?.data?.message || '登录失败，请检查账号密码';
  } finally {
    loginLoading.value = false;
  }
};

const handleRegister = async () => {
  error.value = '';
  errorType.value = 'error';
  const valid = await registerFormRef.value?.validate();
  if (valid) return;

  registerLoading.value = true;

  try {
    await apiRegister({
      name: registerFormData.name,
      email: registerFormData.email,
      password: registerFormData.password,
    });

    error.value = '注册成功，请登录';
    errorType.value = 'success';
    isRegister.value = false;

    // 清空注册表单
    registerFormData.name = '';
    registerFormData.email = '';
    registerFormData.password = '';
  } catch (err: any) {
    error.value = err.response?.data?.message || '注册失败，请稍后重试';
  } finally {
    registerLoading.value = false;
  }
};
</script>

<style scoped lang="scss">
.login-container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-box {
  width: 400px;
  padding: 40px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);

  .login-header {
    text-align: center;
    margin-bottom: 24px;

    .login-title {
      font-size: 24px;
      font-weight: 600;
      color: #1d2129;
      margin-bottom: 8px;
    }

    .login-subtitle {
      font-size: 14px;
      color: #86909c;
    }
  }

  .mode-switch {
    margin-bottom: 24px;
  }
}
</style>
