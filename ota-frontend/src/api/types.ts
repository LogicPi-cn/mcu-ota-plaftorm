// API 响应类型
export interface ApiResponse<T = any> {
  status: string;
  data?: T;
  message?: string;
}

// JWT 相关类型
export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  status: string;
  token: string;
}

export interface RegisterRequest {
  name: string;
  email: string;
  password: string;
}

// 用户类型
export interface User {
  id: string;
  name: string;
  email: string;
  role: string;
  photo: string;
  verified: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface UserData {
  user: User;
}

// 固件相关类型
export interface FirmwareData {
  id: number;
  fwcode: number;
  version_m: number;
  version_n: number;
  version_l: number;
  fwsize: number;
  fwdata?: number[];
  created_at: string;
  updated_at: string;
}

export interface NewFirmwareData {
  fwcode: number;
  version_m: number;
  version_n: number;
  version_l: number;
  fwdata: number[];
}

export interface UpdateFirmwareData {
  fwcode?: number;
  version_m?: number;
  version_n?: number;
  version_l?: number;
  fwdata?: number[];
}

// 升级历史相关类型
export interface UpgradeHistory {
  id: number;
  sn: string;
  device_id: string;
  fwcode: number;
  version_m: number;
  version_n: number;
  version_l: number;
  success: boolean;
  created_at: string;
  updated_at: string;
}

export interface NewUpgradeHistory {
  sn: string;
  device_id: string;
  fwcode: number;
  version_m: number;
  version_n: number;
  version_l: number;
  success: boolean;
}

export interface UpdateUpgradeHistory {
  sn?: string;
  device_id?: string;
  fwcode?: number;
  version_m?: number;
  version_n?: number;
  version_l?: number;
  success?: boolean;
}

// 仪表盘统计类型
export interface DashboardStats {
  firmwareCount: number;
  deviceCount: number;
  successCount: number;
  failCount: number;
}
