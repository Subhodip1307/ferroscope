export interface Node {
  id: number;
  name: string;
}

export interface CPUData {
  cpu: number;
  timestamp: string;
}

export interface RAMData {
  free: string;
  total: string;
  timestamp: string;
}

export interface Service {
  service_name: string;
}

export interface ServiceStatus {
  service_name: string;
  status: "up" | "down";
  service_status: string;
  error_msg?: string;
}

export interface NodeInfo {
  system_name: string;
  kernel_version: string;
  os_version: string;
  uptime: number;
  cpu_threads: number;
  cpu_vendor: string;
}

export interface LoginCredentials {
  username: string;
  password?: string;
}

export interface LoginResponse {
  token: string;
}

export interface UserDetails {
  user_id: number;
  username: string;
}

export interface ChangePasswordCredentials {
  username: string;
  password: string;
}

// ─── Raw API Response Shapes ──────────────────────────────────────────────────
export interface CPUStatRaw {
  value: number;
  date_time: string;
}

export interface RAMStatRaw {
  free: string;
  total: string;
  timestamp: string;
}
