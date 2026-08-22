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
  id: number;
  service_name: string;
}

export interface NodeWithServices {
  node_id: number;
  node_name: string;
  services: Service[];
}

export interface NodesWithServicesPayload {
  obj_id: number[];
}

export interface ServiceStatus {
  service_name: string;
  status: "up" | "down";
  category: string;
  error_msg?: string;
  ssl_exp?: number[] | null;
}

export type ServiceStatusGrouped = Record<string, ServiceStatus[]>;

export interface NodeInfo {
  system_name: string;
  kernel_version: string;
  os_version: string;
  uptime: number;
  cpu_threads: number;
  cpu_vendor: string;
  node_name: string;
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

export interface DiskData {
  read: number;
  write: number;
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

// ─── User Access Control Types ───────────────────────────────────────────────
export interface UserAccessControlItem {
  id: number;
  username: string;
  is_admin: boolean;
  email: string | null;
  joined_date: string;
  permissions?: UserPermissionsResponse;
}

export interface CreateUserPayload {
  username: string;
  email: string | null;
  password: string;
  is_admin: boolean;
}

export interface EditUserPayload {
  id: number;
  username: string;
  is_admin: boolean;
  email?: string | null;
  password?: string | null;
}

// ─── Permission ───────────────────────────────────────────────────────────────

export type MetricType = "RAM" | "CPU" | "DISK";

export interface UserPermission {
  node_id: number;
  metrix: MetricType[] | null;
  services: number[] | null;
  full_permission: boolean | null;
}

export interface AssignPermissionPayload {
  user_id: number;
  nodes_permissions: UserPermission[];
}

export interface NodePermissionView {
  node_id: number;
  is_full_access: boolean;
  metrix: MetricType[];
  services: number[];
}

export interface UserPermissionsResponse {
  user_id: number;
  nodes_permissions: NodePermissionView[];
}
