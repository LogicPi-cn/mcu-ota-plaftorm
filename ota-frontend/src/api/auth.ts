import apiClient from './index';
import type {
  ApiResponse,
  LoginRequest,
  LoginResponse,
  UserData,
  RegisterRequest,
} from './types';

/**
 * 用户登录
 */
export async function login(data: LoginRequest): Promise<LoginResponse> {
  const response = await apiClient.post<ApiResponse<LoginResponse>>('/auth/login', data);
  return response.data.data!;
}

/**
 * 用户注册
 */
export async function register(data: RegisterRequest): Promise<void> {
  await apiClient.post<ApiResponse>('/auth/register', data);
}

/**
 * 用户登出
 */
export async function logout(): Promise<void> {
  await apiClient.get('/auth/logout');
}

/**
 * 获取当前用户信息
 */
export async function getCurrentUser(): Promise<UserData> {
  const response = await apiClient.get<ApiResponse<UserData>>('/users/me');
  return response.data.data!;
}
