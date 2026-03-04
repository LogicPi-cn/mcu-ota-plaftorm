import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import Cookies from 'js-cookie';
import { login as apiLogin, logout as apiLogout } from '@/api/auth';
import type { LoginRequest, User } from '@/api/types';

export const useUserStore = defineStore('user', () => {
  const token = ref<string>(Cookies.get('token') || '');
  const userInfo = ref<User | null>(null);

  const isLoggedIn = computed(() => !!token.value);

  /**
   * 用户登录
   */
  async function login(credentials: LoginRequest) {
    const response = await apiLogin(credentials);
    token.value = response.token;
    Cookies.set('token', token.value, { expires: 1 });
  }

  /**
   * 用户登出
   */
  async function logout() {
    try {
      await apiLogout();
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      token.value = '';
      userInfo.value = null;
      Cookies.remove('token');
    }
  }

  /**
   * 获取用户信息
   */
  async function fetchUserInfo() {
    // 延迟导入避免循环依赖
    const { getCurrentUser } = await import('@/api/auth');
    try {
      const userData = await getCurrentUser();
      userInfo.value = userData.user;
    } catch (error) {
      console.error('Failed to fetch user info:', error);
      await logout();
    }
  }

  return {
    token,
    userInfo,
    isLoggedIn,
    login,
    logout,
    fetchUserInfo,
  };
});
