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
  category: string;
  error_msg?: string;
  ssl_exp?: string | null;
}

export type ServiceStatusGrouped = Record<string, ServiceStatus[]>;

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

// ─── Rule Types ───────────────────────────────────────────────────────────────
export type EventType = "CPU" | "RAM" | "SERVICE" | "NODE";

export interface Condition {
  field: "Status" | "Value";
  operator: "=" | ">" | "<" | ">=" | "<=";
  value: number;
}

export type RuleChannel = "Webhook" | "Email";

export interface RuleAction {
  channel: RuleChannel;
  to: string[];
  message: string;
}

export interface Rule {
  id?: number;
  name: string;
  active: boolean;
  event_type: EventType;
  condition: Condition;
  action: RuleAction;
}
