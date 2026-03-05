import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { login as apiLogin, logout as apiLogout } from '@/api/auth';
import type { LoginRequest, User } from '@/api/types';

// 本地存储 key
const TOKEN_KEY = 'auth_token';
const USER_INFO_KEY = 'user_info';

export const useUserStore = defineStore('user', () => {
  // 从 localStorage 读取 token 和用户信息
  const token = ref<string>(localStorage.getItem(TOKEN_KEY) || '');
  const userInfo = ref<User | null>(null);

  // 初始化时尝试从 localStorage 加载用户信息
  const savedUserInfo = localStorage.getItem(USER_INFO_KEY);
  if (savedUserInfo) {
    try {
      userInfo.value = JSON.parse(savedUserInfo);
    } catch (e) {
      console.error('Failed to parse saved user info:', e);
    }
  }

  const isLoggedIn = computed(() => !!token.value);

  /**
   * 用户登录
   */
  async function login(credentials: LoginRequest) {
    const response = await apiLogin(credentials);
    token.value = response.token;
    // 保存到 localStorage
    localStorage.setItem(TOKEN_KEY, token.value);
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
      localStorage.removeItem(TOKEN_KEY);
      localStorage.removeItem(USER_INFO_KEY);
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
      // 保存用户信息到 localStorage
      localStorage.setItem(USER_INFO_KEY, JSON.stringify(userData.user));
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
