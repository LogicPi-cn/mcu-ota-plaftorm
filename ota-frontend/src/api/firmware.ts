import apiClient from './index';
import type { ApiResponse, FirmwareData, NewFirmwareData, UpdateFirmwareData } from './types';

/**
 * 获取所有固件列表
 */
export async function getFirmwareList(): Promise<FirmwareData[]> {
  const response = await apiClient.get<FirmwareData[]>('/firmware');
  return response.data;
}

/**
 * 获取固件详情
 */
export async function getFirmwareById(id: number): Promise<FirmwareData> {
  const response = await apiClient.get<FirmwareData>(`/firmware/${id}`);
  return response.data;
}

/**
 * 上传新固件
 */
export async function uploadFirmware(data: NewFirmwareData): Promise<FirmwareData> {
  const response = await apiClient.post<ApiResponse<FirmwareData>>('/firmware', data);
  return response.data.data!;
}

/**
 * 更新固件
 */
export async function updateFirmware(id: number, data: UpdateFirmwareData): Promise<FirmwareData> {
  const response = await apiClient.patch<ApiResponse<FirmwareData>>(`/firmware/${id}`, data);
  return response.data.data!;
}

/**
 * 删除固件
 */
export async function deleteFirmware(id: number): Promise<void> {
  await apiClient.delete(`/firmware/${id}`);
}
