import apiClient from './index';
import type { ApiResponse, UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory } from './types';

/**
 * 获取所有升级历史记录
 */
export async function getHistoryList(): Promise<UpgradeHistory[]> {
  const response = await apiClient.get<UpgradeHistory[]>('/history');
  return response.data;
}

/**
 * 获取历史记录详情
 */
export async function getHistoryById(id: number): Promise<UpgradeHistory> {
  const response = await apiClient.get<UpgradeHistory>(`/history/${id}`);
  return response.data;
}

/**
 * 创建历史记录
 */
export async function createHistory(data: NewUpgradeHistory): Promise<UpgradeHistory> {
  const response = await apiClient.post<ApiResponse<UpgradeHistory>>('/history', data);
  return response.data.data!;
}

/**
 * 更新历史记录
 */
export async function updateHistory(id: number, data: UpdateUpgradeHistory): Promise<UpgradeHistory> {
  const response = await apiClient.patch<ApiResponse<UpgradeHistory>>(`/history/${id}`, data);
  return response.data.data!;
}

/**
 * 删除历史记录
 */
export async function deleteHistory(id: number): Promise<void> {
  await apiClient.delete(`/history/${id}`);
}
